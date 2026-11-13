#![cfg(feature = "code")]
//! A block of source code or configuration: a header bar with the file name,
//! language and a copy button, over the code itself.
//!
//! [`CodeBlock`] follows the theme of the page and can be told to stay dark.
//! With the `highlight` feature the code is coloured by the lexer for its
//! `language`; without it the same block shows the code plain. Line numbers
//! and marked lines are opt-in, and cost one element per line only when on.

use yew::prelude::*;

use crate::{Attrs, CopyButton, Size, Theme, is_empty};

/// A run of text and the class it is coloured with, if any.
type Piece<'a> = (&'a str, Option<&'static str>);

#[cfg(feature = "highlight")]
fn pieces<'a>(code: &'a str, language: &str) -> Vec<Piece<'a>> {
    crate::tokenize(code, language).into_iter().map(|token| (token.text, token.kind.map(crate::TokenKind::class))).collect()
}

#[cfg(not(feature = "highlight"))]
fn pieces<'a>(code: &'a str, _language: &str) -> Vec<Piece<'a>> {
    if code.is_empty() { Vec::new() } else { vec![(code, None)] }
}

/// The name shown in the header for `language`: the lexer's own name for it
/// where there is one (`Rust` for `rs`), otherwise the text as given.
fn language_label(language: &AttrValue) -> Option<AttrValue> {
    if language.trim().is_empty() {
        return None;
    }
    #[cfg(feature = "highlight")]
    if let Some(label) = crate::language_label(language) {
        return Some(label);
    }
    Some(language.clone())
}

fn piece((text, class): Piece<'_>) -> Html {
    match class {
        Some(class) => html! { <span {class}>{ text.to_owned() }</span> },
        None => html! { { text.to_owned() } },
    }
}

/// Splits pieces at line breaks: one list of pieces per line of code.
fn lines<'a>(pieces: &[Piece<'a>]) -> Vec<Vec<Piece<'a>>> {
    let mut lines = vec![Vec::new()];
    for &(text, class) in pieces {
        for (n, part) in text.split('\n').enumerate() {
            if n > 0 {
                lines.push(Vec::new());
            }
            if !part.is_empty() {
                lines.last_mut().expect("never empty").push((part, class));
            }
        }
    }
    lines
}

/// Parses a list of line numbers and ranges such as `2, 5-7` into inclusive
/// ranges. Anything that is not a number or a range is skipped.
fn parse_marked(spec: &str) -> Vec<(usize, usize)> {
    spec.split(',')
        .filter_map(|part| {
            let (from, to) = part.split_once('-').unwrap_or((part, part));
            Some((from.trim().parse().ok()?, to.trim().parse().ok()?))
        })
        .collect()
}

#[derive(Properties, PartialEq)]
pub struct CodeBlockProps {
    /// The source text. Trailing blank lines are not shown, and the copy
    /// button copies the text exactly as given.
    pub code: AttrValue,
    /// The language's name, alias or file extension, in any case: `rust`,
    /// `rs`, `yaml`. It is named in the header and, with the `highlight`
    /// feature, picks the lexer. Unknown or empty shows the code plain.
    #[prop_or_default]
    pub language: AttrValue,
    /// The file the code belongs to, shown at the start of the header.
    #[prop_or_default]
    pub filename: Option<AttrValue>,
    /// Shows the copy button. On by default.
    #[prop_or(true)]
    pub copy: bool,
    /// The accessible name and tooltip of the copy button.
    #[prop_or(AttrValue::Static("Copy code"))]
    pub copy_label: AttrValue,
    /// Extra controls for the header, placed before the copy button.
    #[prop_or_default]
    pub actions: Html,
    /// Numbers the lines. The numbers are not selected or copied with the code.
    #[prop_or_default]
    pub line_numbers: bool,
    /// Lines to draw attention to, as numbers and ranges counted from 1:
    /// `"3"`, `"2, 5-7"`.
    #[prop_or_default]
    pub marked: AttrValue,
    /// Wraps long lines instead of scrolling sideways. Use it for prose-like
    /// content such as logs and messages; code reads better unwrapped.
    #[prop_or_default]
    pub wrap: bool,
    /// Forces a theme. By default the block follows the page; `Theme::Dark`
    /// keeps it dark on a light page.
    #[prop_or_default]
    pub theme: Theme,
    /// Fired with the code once it is on the clipboard.
    #[prop_or_default]
    pub on_copy: Callback<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Source code in a framed block with a header bar.
///
/// The header appears when there is something to put in it: a `filename`, a
/// `language`, `actions` or the copy button. The code scrolls sideways
/// inside the block, which can be focused so that the keyboard scrolls it.
#[function_component]
pub fn CodeBlock(props: &CodeBlockProps) -> Html {
    let by_line = props.line_numbers || !props.marked.trim().is_empty();

    let rendered = use_memo(
        (props.code.clone(), props.language.clone(), props.marked.clone(), by_line),
        |(code, language, marked, by_line)| {
            let pieces = pieces(code.trim_end_matches(['\n', '\r']), language);
            if !*by_line {
                return (pieces.into_iter().map(piece).collect::<Html>(), 0);
            }
            let marked = parse_marked(marked);
            let lines = lines(&pieces);
            let count = lines.len();
            let body = lines
                .into_iter()
                .enumerate()
                .map(|(index, pieces)| {
                    let is_marked = marked.iter().any(|(from, to)| (*from..=*to).contains(&(index + 1)));
                    let class = classes!("yi-code-block__line", is_marked.then_some("yi-code-block__line--marked"));
                    // The line break stays in the text, so that selecting and
                    // copying by hand gives the code back line for line.
                    let end = (index + 1 < count).then_some("\n");
                    html! { <span {class}>{ for pieces.into_iter().map(piece) }{ end }</span> }
                })
                .collect::<Html>();
            (body, count)
        },
    );
    let (body, line_count) = (&rendered.0, rendered.1);
    let digits = props.line_numbers.then(|| line_count.max(1).to_string().len().to_string());

    let label = language_label(&props.language);
    let has_actions = !is_empty(&props.actions);
    let has_header = props.filename.is_some() || label.is_some() || has_actions || props.copy;

    let class = classes!(
        "yi-code-block",
        by_line.then_some("yi-code-block--lines"),
        props.line_numbers.then_some("yi-code-block--numbered"),
        props.wrap.then_some("yi-code-block--wrap"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} data-theme={props.theme.as_attr()}>
            if has_header {
                <div class="yi-code-block__header">
                    if let Some(filename) = &props.filename {
                        <span class="yi-code-block__filename" title={filename.clone()}>{ filename.clone() }</span>
                    }
                    if let Some(label) = label {
                        <span class="yi-code-block__language">{ label }</span>
                    }
                    if has_actions || props.copy {
                        <span class="yi-code-block__actions">
                            { props.actions.clone() }
                            if props.copy {
                                <CopyButton
                                    value={props.code.clone()}
                                    label={props.copy_label.clone()}
                                    size={Size::Small}
                                    on_copy={props.on_copy.clone()}
                                />
                            }
                        </span>
                    }
                </div>
            }
            // Focusable, so that the keyboard can scroll code wider than the block.
            <pre class="yi-code-block__body" tabindex="0" data-yi-digits={digits}>
                <code class="yi-highlight">{ body.clone() }</code>
            </pre>
        </div>
    })
}

#[cfg(test)]
mod tests;
