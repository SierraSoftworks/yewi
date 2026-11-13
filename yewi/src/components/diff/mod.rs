#![cfg(feature = "diff")]
//! Diff: what changed between two texts, unified or side by side.

use std::{ops::Range, rc::Rc};

use yew::prelude::*;

use crate::{Attrs, Icon, components::server_only::use_server_only, icons};

mod engine;

pub use engine::{DiffLine, DiffLineKind, diff_lines, parse_unified_diff};

/// How the two sides of a diff are laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum DiffMode {
    /// One column, removed lines above the lines that replace them.
    #[default]
    Unified,
    /// Old text on the left, new on the right. Long lines wrap, and the two
    /// sides stack when the diff is too narrow to hold them.
    Split,
}

impl DiffMode {
    pub const ALL: [DiffMode; 2] = [DiffMode::Unified, DiffMode::Split];

    const fn as_str(self) -> &'static str {
        match self {
            DiffMode::Unified => "unified",
            DiffMode::Split => "split",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DiffProps {
    /// The old text. With `after`, the diff is computed in the browser.
    #[prop_or_default]
    pub before: Option<AttrValue>,
    /// The new text.
    #[prop_or_default]
    pub after: Option<AttrValue>,
    /// A unified diff (`diff -u`, `git diff`) to draw as given. Takes
    /// precedence over `before` and `after`.
    #[prop_or_default]
    pub unified: Option<AttrValue>,
    /// Shown in the header, usually a file name. Defaults to the names in
    /// the unified diff's `---` and `+++` lines.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub mode: DiffMode,
    /// Unchanged lines kept around each change when comparing `before` and
    /// `after`; longer unchanged runs fold behind a button.
    #[prop_or(3)]
    pub context: usize,
    /// The text of the button that reveals a folded run, from the number of
    /// lines it hides. Defaults to "Show 12 unchanged lines" in English.
    #[prop_or_default]
    pub expand_label: Option<Callback<usize, AttrValue>>,
    /// Shows old and new line numbers.
    #[prop_or(true)]
    pub line_numbers: bool,
    /// Wraps long lines instead of scrolling sideways.
    #[prop_or_default]
    pub wrap: bool,
    /// Draws the text of a line, for example to highlight syntax or to show
    /// tokens. Changes within a line are not marked when this is set.
    #[prop_or_default]
    pub render_line: Option<Callback<AttrValue, Html>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Everything derived from the input text, computed once per input.
struct Model {
    lines: Vec<DiffLine>,
    /// Per line, the part that differs from the line it is paired with.
    emphasis: Vec<Option<Range<usize>>>,
    /// Runs of unchanged lines hidden until expanded.
    folds: Vec<Range<usize>>,
    added: usize,
    removed: usize,
    /// A title taken from the patch's file header.
    title: Option<AttrValue>,
    digits: usize,
}

impl Model {
    fn new(unified: Option<&str>, before: &str, after: &str, context: usize) -> Self {
        let (mut lines, foldable) = match unified {
            Some(unified) => (parse_unified_diff(unified), false),
            None => (diff_lines(before, after), true),
        };

        // The first file header becomes the title rather than two rows.
        let mut title = None;
        if let [old, new, ..] = lines.as_slice()
            && old.kind == DiffLineKind::File
            && new.kind == DiffLineKind::File
        {
            let (old, new) = (engine::file_name(&old.text), engine::file_name(&new.text));
            title = Some(AttrValue::from(match (old, new) {
                (old, new) if old == new || new == "/dev/null" => old.to_owned(),
                ("/dev/null", new) => new.to_owned(),
                (old, new) => format!("{old} \u{2192} {new}"),
            }));
            lines.drain(..2);
        }

        let mut emphasis = vec![None; lines.len()];
        for_each_block(&lines, 0..lines.len(), |removed, added| {
            for (old, new) in removed.zip(added) {
                if let Some((a, b)) = engine::changed_spans(&lines[old].text, &lines[new].text) {
                    emphasis[old] = Some(a).filter(|range| !range.is_empty());
                    emphasis[new] = Some(b).filter(|range| !range.is_empty());
                }
            }
        });

        let count = |kind| lines.iter().filter(|line| line.kind == kind).count();
        let max = lines.iter().filter_map(|line| line.old.max(line.new)).max().unwrap_or(0);
        Self {
            folds: if foldable { folds(&lines, context) } else { Vec::new() },
            added: count(DiffLineKind::Added),
            removed: count(DiffLineKind::Removed),
            emphasis,
            title,
            digits: max.to_string().len(),
            lines,
        }
    }
}

/// Unchanged runs further than `context` lines from any change. A lone
/// hidden line is not worth a row of its own, so it stays visible.
fn folds(lines: &[DiffLine], context: usize) -> Vec<Range<usize>> {
    let changes: Vec<usize> = lines.iter().enumerate().filter(|(_, l)| l.is_change()).map(|(i, _)| i).collect();
    let mut out = Vec::new();
    let mut start = 0;
    for next in changes.iter().copied().chain([lines.len() + context]) {
        let end = next.saturating_sub(context).min(lines.len());
        if end > start + 1 {
            out.push(start..end);
        }
        start = next + 1 + context;
    }
    out
}

/// Calls `f` with the removed and added line indices of each run of changes.
fn for_each_block(lines: &[DiffLine], range: Range<usize>, mut f: impl FnMut(Range<usize>, Range<usize>)) {
    let mut i = range.start;
    while i < range.end {
        let run = |kind, from: usize| from + lines[from..range.end].iter().take_while(|l| l.kind == kind).count();
        let removed_end = run(DiffLineKind::Removed, i);
        let added_end = run(DiffLineKind::Added, removed_end);
        if added_end > i {
            f(i..removed_end, removed_end..added_end);
        }
        i = added_end.max(i + 1);
    }
}

/// A unified or side-by-side view of what changed between two texts.
///
/// Pass `before` and `after` to compare them in the browser, or `unified`
/// to draw a diff computed elsewhere.
#[function_component]
pub fn Diff(props: &DiffProps) -> Html {
    let inputs = (props.unified.clone(), props.before.clone(), props.after.clone(), props.context);
    let model = use_memo(inputs.clone(), |(unified, before, after, context)| {
        Model::new(unified.as_deref(), before.as_deref().unwrap_or_default(), after.as_deref().unwrap_or_default(), *context)
    });

    // Folds the user has opened, by the index of their first line.
    let expanded = use_state_eq(Vec::<usize>::new);
    {
        let expanded = expanded.clone();
        use_effect_with(inputs, move |_| expanded.set(Vec::new()));
    }

    let server_only = use_server_only();
    let scroller = use_node_ref();
    let expand = {
        let (expanded, scroller) = (expanded.clone(), scroller.clone());
        Callback::from(move |start: usize| {
            let mut open = (*expanded).clone();
            open.push(start);
            expanded.set(open);
            // The button is about to go; keep keyboard focus in the diff.
            #[cfg(target_arch = "wasm32")]
            if let Some(element) = scroller.cast::<web_sys::HtmlElement>() {
                let _ = element.focus();
            }
            let _ = &scroller;
        })
    };

    let view = View { model: &model, props };
    let mut rows = Vec::new();
    let mut i = 0;
    for fold in model.folds.iter().filter(|fold| !expanded.contains(&fold.start)).cloned() {
        view.rows(i..fold.start, &mut rows);
        let hidden = fold.len();
        let onclick = expand.reform(move |_: MouseEvent| fold.start);
        rows.push(html! {
            <div
                key={format!("f{}", fold.start)}
                class="yi-diff__band yi-diff__band--fold"
                data-yi-reveal={server_only.then_some("")}
            >
                <button type="button" class="yi-diff__fold" {onclick}>
                    <Icon icon={icons::CHEVRONS_UP_DOWN} />
                    { match &props.expand_label {
                        Some(label) => label.emit(hidden),
                        None => format!("Show {hidden} unchanged lines").into(),
                    } }
                </button>
            </div>
        });
        // Without a client the folded lines are in the page, for
        // `BEHAVIOURS` to reveal.
        if server_only {
            let mut folded = Vec::new();
            view.rows(fold.clone(), &mut folded);
            rows.push(html! {
                <div key={format!("r{}", fold.start)} class="yi-diff__folded" hidden=true>{ for folded }</div>
            });
        }
        i = fold.end;
    }
    view.rows(i..model.lines.len(), &mut rows);

    let title = props.title.clone().or_else(|| model.title.clone());
    let unchanged = model.added + model.removed == 0;
    let class = classes!(
        "yi-diff",
        format!("yi-diff--{}", props.mode.as_str()),
        props.wrap.then_some("yi-diff--wrap"),
        props.line_numbers.then_some("yi-diff--numbered"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <figure id={props.id.clone()} {class} style={format!("--yi-diff-digits:{}", model.digits)}>
            <figcaption class="yi-diff__header">
                if let Some(title) = &title {
                    <span class="yi-diff__title">{ title }</span>
                }
                if unchanged {
                    <span class="yi-diff__stats">{ "No changes" }</span>
                } else {
                    <span class="yi-diff__stats">
                        <span aria-hidden="true" class="yi-diff__stat yi-diff__stat--added">{ format!("+{}", model.added) }</span>
                        <span aria-hidden="true" class="yi-diff__stat yi-diff__stat--removed">{ format!("\u{2212}{}", model.removed) }</span>
                        <span class="yi-visually-hidden">
                            { format!("{} added, {} removed", plural(model.added), plural(model.removed)) }
                        </span>
                    </span>
                }
            </figcaption>
            <div
                ref={scroller}
                class="yi-diff__scroll"
                role="group"
                aria-label={title.unwrap_or_else(|| "Changes".into())}
                tabindex="0"
            >
                <div class="yi-diff__body">{ for rows }</div>
            </div>
        </figure>
    })
}

fn plural(count: usize) -> String {
    format!("{count} {}", if count == 1 { "line" } else { "lines" })
}

struct View<'a> {
    model: &'a Rc<Model>,
    props: &'a DiffProps,
}

impl View<'_> {
    /// Appends the rows for a visible range of lines.
    fn rows(&self, range: Range<usize>, out: &mut Vec<Html>) {
        let lines = &self.model.lines;
        let split = self.props.mode == DiffMode::Split;
        let mut i = range.start;
        while i < range.end {
            let line = &lines[i];
            match line.kind {
                DiffLineKind::Hunk | DiffLineKind::File | DiffLineKind::Meta => {
                    let modifier = match line.kind {
                        DiffLineKind::Hunk => "yi-diff__band--hunk",
                        DiffLineKind::File => "yi-diff__band--file",
                        _ => "yi-diff__band--meta",
                    };
                    out.push(html! {
                        <div key={i} class={classes!("yi-diff__band", modifier)}>{ &line.text }</div>
                    });
                    i += 1;
                }
                DiffLineKind::Context if split => {
                    out.push(html! {
                        <div key={i} class="yi-diff__row yi-diff__row--context">
                            <div class="yi-diff__side yi-diff__side--old">{ self.cells(i, Numbers::Old) }</div>
                            <div class="yi-diff__side yi-diff__side--new">{ self.cells(i, Numbers::New) }</div>
                        </div>
                    });
                    i += 1;
                }
                _ if split => {
                    // Pair the nth removed line with the nth added line.
                    let end = i + lines[i..range.end].iter().take_while(|l| l.is_change()).count();
                    for_each_block(lines, i..end, |removed, added| {
                        for n in 0..removed.len().max(added.len()) {
                            let side = |index: Option<usize>, numbers| match index {
                                Some(index) => html! {
                                    <div class={classes!("yi-diff__side", row_modifier(lines[index].kind))}>
                                        { self.cells(index, numbers) }
                                    </div>
                                },
                                None => html! { <div class="yi-diff__side yi-diff__side--empty" /> },
                            };
                            let (old, new) = (removed.clone().nth(n), added.clone().nth(n));
                            out.push(html! {
                                <div key={old.or(new).unwrap_or(i)} class="yi-diff__row">
                                    { side(old, Numbers::Old) }
                                    { side(new, Numbers::New) }
                                </div>
                            });
                        }
                    });
                    i = end;
                }
                kind => {
                    out.push(html! {
                        <div key={i} class={classes!("yi-diff__row", "yi-diff__side", row_modifier(kind))}>
                            { self.cells(i, Numbers::Both) }
                        </div>
                    });
                    i += 1;
                }
            }
        }
    }

    /// Line numbers, the change marker and the text of one line.
    fn cells(&self, index: usize, numbers: Numbers) -> Html {
        let line = &self.model.lines[index];
        let number = |n: Option<u32>| {
            html! { <span class="yi-diff__num" aria-hidden="true" data-n={n.map(|n| n.to_string())} /> }
        };
        let text = match (&self.props.render_line, &self.model.emphasis[index]) {
            (Some(render), _) => render.emit(line.text.clone()),
            (None, Some(range)) => {
                let (before, rest) = line.text.split_at(range.start);
                let (changed, after) = rest.split_at(range.len());
                html! {
                    <>
                        if !before.is_empty() { { before.to_owned() } }
                        <mark class="yi-diff__changed">{ changed.to_owned() }</mark>
                        if !after.is_empty() { { after.to_owned() } }
                    </>
                }
            }
            (None, None) if line.text.is_empty() => Html::default(),
            (None, None) => html! { { &line.text } },
        };
        // `<ins>` and `<del>` tell assistive technology what the colour says.
        let (tag, sign) = match line.kind {
            DiffLineKind::Added => ("ins", Some("+")),
            DiffLineKind::Removed => ("del", Some("\u{2212}")),
            _ => ("span", None),
        };
        html! {
            <>
                if self.props.line_numbers {
                    if numbers != Numbers::New { { number(line.old) } }
                    if numbers != Numbers::Old { { number(line.new) } }
                }
                <span class="yi-diff__sign" aria-hidden="true">
                    if let Some(sign) = sign { { sign } }
                </span>
                <@{tag} class="yi-diff__text">{ text }</@>
            </>
        }
    }
}

/// Which line-number columns a row carries.
#[derive(Clone, Copy, PartialEq)]
enum Numbers {
    Old,
    New,
    Both,
}

fn row_modifier(kind: DiffLineKind) -> &'static str {
    match kind {
        DiffLineKind::Added => "yi-diff__side--added",
        DiffLineKind::Removed => "yi-diff__side--removed",
        _ => "yi-diff__side--context",
    }
}

#[cfg(test)]
mod tests;
