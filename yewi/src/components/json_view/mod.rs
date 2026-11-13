//! A JSON document, as a tree to explore or as source to read.
//!
//! - [`JsonView`] shows a [`Json`] value. `Tree` folds objects and arrays
//!   behind native `<details>` elements; `Source` prints the document as
//!   indented, coloured JSON.
//! - [`Json`] is the document: cheap to clone and compare, parsed with
//!   [`Json::parse`] or converted from `serde_json::Value` (feature
//!   `serde_json`).
//!
//! The tree is built for large documents. A folded node renders only its
//! summary line, its children are created when it is first opened, and a
//! long array or object is shown a page at a time. The page therefore holds
//! what the user has asked to see, not every node of the document.

use yew::prelude::*;

use crate::{Attrs, Button, ButtonVariant, CopyButton, Icon, Size, components::server_only::use_server_only, icons};

mod value;

pub use value::{Json, JsonError};

/// An array of at most this many scalars is written on one line.
const INLINE_ITEMS: usize = 8;

/// How a [`JsonView`] presents its document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum JsonViewVariant {
    /// A collapsible outline, for exploring a document of any size.
    #[default]
    Tree,
    /// The whole document as indented JSON, for payloads short enough to
    /// read from top to bottom. Every line is rendered at once.
    Source,
}

impl JsonViewVariant {
    pub const ALL: [JsonViewVariant; 2] = [JsonViewVariant::Tree, JsonViewVariant::Source];

    const fn as_str(self) -> &'static str {
        match self {
            JsonViewVariant::Tree => "tree",
            JsonViewVariant::Source => "source",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct JsonViewProps {
    /// The document. Keep it in a `use_memo` or state so that a re-render
    /// of the parent hands over the same value rather than a new parse.
    pub value: Json,
    #[prop_or_default]
    pub variant: JsonViewVariant,
    /// `Tree`: how many levels start open. 0 folds the whole document.
    #[prop_or(1)]
    pub open: usize,
    /// `Tree`: a name for the root node, such as "facts" or "payload".
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// `Tree`: how many children of one array or object are shown before a
    /// "show more" button. 0 shows them all.
    #[prop_or(100)]
    pub page_size: usize,
    /// Shows a button that copies the indented document.
    #[prop_or(true)]
    pub copy: bool,
    /// The accessible name and tooltip of the copy button.
    #[prop_or(AttrValue::Static("Copy JSON"))]
    pub copy_label: AttrValue,
    /// Drops the tinted surface and padding, for a view that sits inside a
    /// table row, a panel or another surface.
    #[prop_or_default]
    pub plain: bool,
    /// A CSS length that caps the height. A taller document scrolls inside
    /// the view, which then takes keyboard focus so it can be scrolled.
    #[prop_or_default]
    pub max_height: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Shows a JSON document as a collapsible tree or as highlighted source.
#[function_component]
pub fn JsonView(props: &JsonViewProps) -> Html {
    // Serialised once per document, not on every render.
    let text = use_memo((props.value.clone(), props.copy), |(value, copy)| {
        copy.then(|| AttrValue::from(value.to_pretty()))
    });

    let class = classes!(
        "yi-json-view",
        format!("yi-json-view--{}", props.variant.as_str()),
        props.plain.then_some("yi-json-view--plain"),
        props.class.clone(),
    );

    let body = match props.variant {
        JsonViewVariant::Tree => {
            let label = props.name.clone().map(Label::Key);
            node(&props.value, label, 0, 0, props.open, props.page_size)
        }
        JsonViewVariant::Source => html! { <pre class="yi-json-view__source"><code>{ source(&props.value) }</code></pre> },
    };

    // A scrolling region must be reachable by keyboard and named.
    let scrolls = props.max_height.is_some();
    let style = props.max_height.as_ref().map(|height| format!("max-block-size:{height}"));
    let region_label = scrolls.then(|| props.name.clone().unwrap_or(AttrValue::Static("JSON")));

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            if let Some(text) = (*text).clone() {
                <CopyButton
                    class="yi-json-view__copy"
                    value={text}
                    label={props.copy_label.clone()}
                    size={Size::Small}
                />
            }
            <div
                class="yi-json-view__body"
                {style}
                tabindex={scrolls.then_some("0")}
                role={scrolls.then_some("region")}
                aria-label={region_label}
            >
                { body }
            </div>
        </div>
    })
}

/// What a node is called under its parent.
#[derive(Clone, PartialEq)]
enum Label {
    Key(AttrValue),
    Index(usize),
}

impl Label {
    fn view(&self) -> Html {
        match self {
            Label::Key(key) => html! { <span class="yi-json-view__key">{ key_text(key) }</span> },
            Label::Index(index) => html! { <span class="yi-json-view__index">{ index.to_string() }</span> },
        }
    }
}

/// A key as the tree shows it: bare where that is unambiguous, quoted when
/// it is empty or holds whitespace or other characters that need escaping.
fn key_text(key: &AttrValue) -> AttrValue {
    let plain = !key.is_empty() && key.chars().all(|c| !c.is_whitespace() && !c.is_control() && c != '"' && c != '\\');
    if plain { key.clone() } else { quoted(key).into() }
}

fn quoted(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    value::quote(text, &mut out);
    out
}

/// A run of text and coloured tokens. Plain text between two tokens is
/// joined into one node: fewer nodes to create, and none that are adjacent
/// text nodes, which a server render would merge and hydration then miss.
#[derive(Default)]
struct Line {
    nodes: Vec<Html>,
    text: String,
}

impl Line {
    fn text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    fn flush(&mut self) {
        if !self.text.is_empty() {
            self.nodes.push(html! { { std::mem::take(&mut self.text) } });
        }
    }

    fn node(&mut self, node: Html) {
        self.flush();
        self.nodes.push(node);
    }

    fn token(&mut self, kind: &'static str, text: impl Into<AttrValue>) {
        let text: AttrValue = text.into();
        self.node(html! { <span class={kind}>{ text }</span> });
    }

    /// A value drawn on one line: a scalar or an empty container.
    fn scalar(&mut self, value: &Json) {
        match value {
            Json::Null => self.token("yi-json-view__null", "null"),
            Json::Bool(value) => self.token("yi-json-view__bool", if *value { "true" } else { "false" }),
            Json::Number(text) => self.token("yi-json-view__number", text.clone()),
            Json::String(text) => self.token("yi-json-view__string", quoted(text)),
            Json::Array(_) => self.text("[]"),
            Json::Object(_) => self.text("{}"),
        }
    }

    fn finish(mut self) -> Html {
        self.flush();
        self.nodes.into_iter().collect()
    }
}

/// Whether `value` is drawn as a collapsible branch rather than on one line.
fn is_branch(value: &Json) -> bool {
    match value {
        Json::Object(entries) => !entries.is_empty(),
        Json::Array(items) => items.len() > INLINE_ITEMS || items.iter().any(Json::is_container),
        _ => false,
    }
}

/// A one-line hint of what a folded branch holds: its first few values,
/// with long strings cut and nested containers reduced to their brackets.
fn preview(value: &Json) -> String {
    const ENTRIES: usize = 3;
    const CHARS: usize = 24;

    fn brief(value: &Json, out: &mut String) {
        match value {
            Json::String(text) if text.chars().count() > CHARS => {
                let cut: String = text.chars().take(CHARS).collect();
                value::quote(&cut, out);
                out.insert(out.len() - 1, '…');
            }
            Json::Array(items) if !items.is_empty() => out.push_str("[…]"),
            Json::Object(entries) if !entries.is_empty() => out.push_str("{…}"),
            value => out.push_str(&value.to_string()),
        }
    }

    let mut out = String::new();
    match value {
        Json::Object(entries) => {
            for (i, (key, value)) in entries.iter().take(ENTRIES).enumerate() {
                out.push_str(if i > 0 { ", " } else { "" });
                out.push_str(&key_text(key));
                out.push_str(": ");
                brief(value, &mut out);
            }
        }
        Json::Array(items) => {
            for (i, value) in items.iter().take(ENTRIES).enumerate() {
                out.push_str(if i > 0 { ", " } else { "" });
                brief(value, &mut out);
            }
        }
        _ => {}
    }
    if value.len() > ENTRIES {
        out.push_str(", …");
    }
    out
}

/// One node of the tree: a [`Branch`], or a single line. `at` is its place
/// under its parent and keys it, so that showing more siblings appends to the
/// list and every node already on the page keeps its state.
fn node(value: &Json, label: Option<Label>, at: usize, depth: usize, open: usize, page_size: usize) -> Html {
    if is_branch(value) {
        return html! { <Branch key={at} value={value.clone()} {label} {depth} {open} {page_size} /> };
    }

    let mut line = Line::default();
    if let Some(label) = &label {
        line.node(label.view());
        line.text(": ");
    }
    match value {
        Json::Array(items) if !items.is_empty() => {
            line.text("[");
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    line.text(", ");
                }
                line.scalar(item);
            }
            line.text("]");
        }
        value => line.scalar(value),
    }
    html! { <div key={at} class="yi-json-view__row">{ line.finish() }</div> }
}

#[derive(Properties, PartialEq)]
struct BranchProps {
    /// Compared by identity first, so re-rendering a parent costs one
    /// pointer comparison per child however large the child is.
    value: Json,
    label: Option<Label>,
    depth: usize,
    open: usize,
    page_size: usize,
}

/// A non-empty object or array. Its children exist only while it is open.
#[function_component]
fn Branch(props: &BranchProps) -> Html {
    let expanded = use_state_eq(|| props.depth < props.open);
    let shown = use_state_eq(|| props.page_size);
    // Without a client the whole tree is in the page and the browser folds
    // it: nothing could add the children, or the next page of them, later.
    let server_only = use_server_only();

    // The summary toggles the state and the state the element. Left to the
    // browser, the element would open a frame before it has any children.
    let on_toggle = {
        let expanded = expanded.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            expanded.set(!*expanded);
        })
    };

    let (len, count) = match &props.value {
        Json::Object(entries) => (entries.len(), format!("{{{}}}", entries.len())),
        Json::Array(items) => (items.len(), format!("[{}]", items.len())),
        _ => return Html::default(),
    };
    let visible = if props.page_size == 0 || server_only { len } else { len.min(*shown) };

    let children = (*expanded || server_only).then(|| {
        let (depth, open, page_size) = (props.depth + 1, props.open, props.page_size);
        let rows: Html = match &props.value {
            Json::Object(entries) => entries
                .iter()
                .take(visible)
                .enumerate()
                .map(|(i, (key, value))| node(value, Some(Label::Key(key.clone())), i, depth, open, page_size))
                .collect(),
            Json::Array(items) => items
                .iter()
                .take(visible)
                .enumerate()
                .map(|(i, value)| node(value, Some(Label::Index(i)), i, depth, open, page_size))
                .collect(),
            _ => Html::default(),
        };
        let more = (visible < len).then(|| {
            let (hidden, step) = (len - visible, props.page_size.min(len - visible));
            let on_click = {
                let (shown, next) = (shown.clone(), visible + step);
                Callback::from(move |_| shown.set(next))
            };
            html! {
                <div class="yi-json-view__more">
                    <Button variant={ButtonVariant::Subtle} size={Size::Small} {on_click}>
                        { format!("Show {step} more") }
                    </Button>
                    <span class="yi-json-view__hidden">{ format!("{hidden} hidden") }</span>
                </div>
            }
        });
        html! {
            <div key="children" class="yi-json-view__children">
                { rows }
                { more }
            </div>
        }
    });

    html! {
        <details class="yi-json-view__node" open={*expanded}>
            // Keyed, so that adding the children after it leaves the summary,
            // and the focus on it, in place.
            <summary key="summary" class="yi-json-view__summary" onclick={on_toggle}>
                <Icon class="yi-json-view__chevron" icon={icons::CHEVRON_RIGHT} />
                if let Some(label) = &props.label {
                    { label.view() }
                }
                <span class="yi-json-view__count">{ count }</span>
                <span class="yi-json-view__preview">
                    if !*expanded || server_only {
                        { preview(&props.value) }
                    }
                </span>
            </summary>
            { children }
        </details>
    }
}

/// The document as indented JSON with coloured tokens.
fn source(value: &Json) -> Html {
    fn write(value: &Json, depth: usize, line: &mut Line) {
        let indent = |line: &mut Line, depth: usize| {
            line.text("\n");
            (0..depth).for_each(|_| line.text("  "));
        };
        match value {
            Json::Array(items) if !items.is_empty() => {
                line.text("[");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        line.text(",");
                    }
                    indent(line, depth + 1);
                    write(item, depth + 1, line);
                }
                indent(line, depth);
                line.text("]");
            }
            Json::Object(entries) if !entries.is_empty() => {
                line.text("{");
                for (i, (key, item)) in entries.iter().enumerate() {
                    if i > 0 {
                        line.text(",");
                    }
                    indent(line, depth + 1);
                    line.token("yi-json-view__key", quoted(key));
                    line.text(": ");
                    write(item, depth + 1, line);
                }
                indent(line, depth);
                line.text("}");
            }
            scalar => line.scalar(scalar),
        }
    }

    let mut line = Line::default();
    write(value, 0, &mut line);
    line.finish()
}

#[cfg(test)]
mod tests;
