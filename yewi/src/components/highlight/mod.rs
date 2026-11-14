#![cfg(feature = "highlight")]
//! Syntax highlighting: source text in, coloured spans out.
//!
//! - [`Highlight`] renders a `<code>` element with one `<span>` per coloured
//!   token. Put it in running text, in your own `<pre>`, or let `CodeBlock`
//!   (feature `code`) do so with a header and line numbers.
//! - [`tokenize`] is the same work without the markup, for a caller that
//!   renders tokens itself.
//! - [`Language`] lists the built-in lexers. They are deliberately small:
//!   comments, strings, numbers, keywords and a few shapes, in a few
//!   kilobytes of wasm.
//! - The lexers live in the `yewi-highlight` crate, which has no Yew
//!   dependency, and are re-exported here.
//!
//! The `highlight-syntect` feature adds [syntect](https://docs.rs/syntect)
//! and its Sublime Text grammars. Any language syntect knows is then lexed by
//! it, and the built-in lexers cover the rest (TOML, TypeScript, HCL). It is
//! accurate and wide, and costs about a megabyte of wasm, so prefer it on the
//! server or where code is the product.
//!
//! Tokens are coloured by the `--yi-highlight-*` tokens, which follow the
//! theme of the element they are in.

use yew::prelude::*;

use crate::Attrs;

pub use yewi_highlight::{Language, Token, TokenKind, tokenize};

/// The name to show for the language called `language`, such as `Rust` for
/// `rs`. `None` when no lexer knows it.
pub fn language_label(language: &str) -> Option<AttrValue> {
    yewi_highlight::language_label(language).map(AttrValue::from)
}

/// Renders tokens as text and `<span>` elements, without a wrapper. Plain
/// tokens become bare text, so the markup grows only with what is coloured.
pub fn render_tokens(tokens: &[Token<'_>]) -> Html {
    tokens
        .iter()
        .map(|token| match token.kind {
            Some(kind) => html! { <span class={kind.class()}>{ token.text.to_owned() }</span> },
            None => html! { { token.text.to_owned() } },
        })
        .collect()
}

#[derive(Properties, PartialEq)]
pub struct HighlightProps {
    /// The source text.
    pub code: AttrValue,
    /// The language's name, alias or file extension, in any case: `rust`,
    /// `rs`, `yaml`. Unknown or empty shows the code plain.
    #[prop_or_default]
    pub language: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Source text as a `<code>` element with its syntax coloured.
///
/// It adds no box of its own: inline it sits in a sentence, and inside a
/// `<pre>` it keeps the line breaks. The code is lexed again only when
/// `code` or `language` change.
#[function_component]
pub fn Highlight(props: &HighlightProps) -> Html {
    let rendered = use_memo((props.code.clone(), props.language.clone()), |(code, language)| {
        render_tokens(&tokenize(code, language))
    });

    props.attrs.apply(html! {
        <code id={props.id.clone()} class={classes!("yi-highlight", props.class.clone())}>
            { (*rendered).clone() }
        </code>
    })
}

#[cfg(test)]
mod tests;
