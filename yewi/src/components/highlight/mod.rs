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

mod lexer;
#[cfg(feature = "highlight-syntect")]
mod syntect;

/// What a token is, which decides its colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Comment,
    /// A reserved word: `fn`, `if`, `SELECT`, `@media`.
    Keyword,
    /// A named constant: `true`, `null`, `MAX_SIZE`, `&amp;`.
    Literal,
    Number,
    String,
    /// A type or class name.
    Type,
    /// A function being called, or a command in a shell.
    Function,
    /// A key in data or markup: a JSON key, a CSS property, an attribute.
    Property,
    /// An element name in markup.
    Tag,
    /// A variable reference such as `$HOME`.
    Variable,
    /// Code about code: attributes, decorators, macros, section headings.
    Meta,
    /// A line added in a diff.
    Inserted,
    /// A line removed in a diff.
    Deleted,
}

impl TokenKind {
    pub const ALL: [TokenKind; 13] = [
        TokenKind::Comment,
        TokenKind::Keyword,
        TokenKind::Literal,
        TokenKind::Number,
        TokenKind::String,
        TokenKind::Type,
        TokenKind::Function,
        TokenKind::Property,
        TokenKind::Tag,
        TokenKind::Variable,
        TokenKind::Meta,
        TokenKind::Inserted,
        TokenKind::Deleted,
    ];

    /// The name used in the class and the token: `keyword` for
    /// `.yi-tok-keyword` and `--yi-highlight-keyword`.
    pub const fn as_str(self) -> &'static str {
        match self {
            TokenKind::Comment => "comment",
            TokenKind::Keyword => "keyword",
            TokenKind::Literal => "literal",
            TokenKind::Number => "number",
            TokenKind::String => "string",
            TokenKind::Type => "type",
            TokenKind::Function => "function",
            TokenKind::Property => "property",
            TokenKind::Tag => "tag",
            TokenKind::Variable => "variable",
            TokenKind::Meta => "meta",
            TokenKind::Inserted => "inserted",
            TokenKind::Deleted => "deleted",
        }
    }

    /// The class a token of this kind is rendered with.
    pub const fn class(self) -> &'static str {
        match self {
            TokenKind::Comment => "yi-tok-comment",
            TokenKind::Keyword => "yi-tok-keyword",
            TokenKind::Literal => "yi-tok-literal",
            TokenKind::Number => "yi-tok-number",
            TokenKind::String => "yi-tok-string",
            TokenKind::Type => "yi-tok-type",
            TokenKind::Function => "yi-tok-function",
            TokenKind::Property => "yi-tok-property",
            TokenKind::Tag => "yi-tok-tag",
            TokenKind::Variable => "yi-tok-variable",
            TokenKind::Meta => "yi-tok-meta",
            TokenKind::Inserted => "yi-tok-inserted",
            TokenKind::Deleted => "yi-tok-deleted",
        }
    }
}

/// A run of source text and what it is. `kind` is `None` for plain text:
/// whitespace, punctuation and ordinary names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub text: &'a str,
    pub kind: Option<TokenKind>,
}

/// A language with a built-in lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Shell scripts and command lines: `bash`, `sh`, `shell`, `zsh`, `console`.
    Bash,
    /// CSS and SCSS.
    Css,
    /// Unified diffs: `diff`, `patch`.
    Diff,
    Go,
    /// HashiCorp configuration: `hcl`, `nomad`, `terraform`, `tf`.
    Hcl,
    /// HTML, XML and SVG, with CSS and JavaScript inside `<style>` and `<script>`.
    Html,
    /// JavaScript and JSX.
    JavaScript,
    /// JSON, with comments allowed.
    Json,
    Python,
    Rust,
    Sql,
    Toml,
    /// TypeScript and TSX.
    TypeScript,
    Yaml,
}

impl Language {
    pub const ALL: [Language; 14] = [
        Language::Bash,
        Language::Css,
        Language::Diff,
        Language::Go,
        Language::Hcl,
        Language::Html,
        Language::JavaScript,
        Language::Json,
        Language::Python,
        Language::Rust,
        Language::Sql,
        Language::Toml,
        Language::TypeScript,
        Language::Yaml,
    ];

    /// Finds a language by name, alias or file extension, in any case:
    /// `rust`, `rs`, `YAML`, `yml`, `sh`. This is the form a Markdown code
    /// fence uses.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim().to_ascii_lowercase();
        Self::ALL.into_iter().find(|language| language.names().split(' ').any(|known| known == name))
    }

    /// The canonical name, as written in a Markdown code fence.
    pub fn name(self) -> &'static str {
        self.names().split(' ').next().unwrap_or_default()
    }

    /// Every name `from_name` accepts, canonical first, space-separated.
    const fn names(self) -> &'static str {
        match self {
            Language::Bash => "bash sh shell zsh console",
            Language::Css => "css scss",
            Language::Diff => "diff patch",
            Language::Go => "go golang",
            Language::Hcl => "hcl nomad terraform tf",
            Language::Html => "html htm xml svg",
            Language::JavaScript => "javascript js jsx mjs cjs",
            Language::Json => "json jsonc json5",
            Language::Python => "python py",
            Language::Rust => "rust rs",
            Language::Sql => "sql",
            Language::Toml => "toml",
            Language::TypeScript => "typescript ts tsx",
            Language::Yaml => "yaml yml",
        }
    }

    /// The name to show a reader: `Rust`, `TOML`, `JavaScript`.
    pub const fn label(self) -> &'static str {
        match self {
            Language::Bash => "Shell",
            Language::Css => "CSS",
            Language::Diff => "Diff",
            Language::Go => "Go",
            Language::Hcl => "HCL",
            Language::Html => "HTML",
            Language::JavaScript => "JavaScript",
            Language::Json => "JSON",
            Language::Python => "Python",
            Language::Rust => "Rust",
            Language::Sql => "SQL",
            Language::Toml => "TOML",
            Language::TypeScript => "TypeScript",
            Language::Yaml => "YAML",
        }
    }
}

/// Splits `source` into tokens for the language called `language`.
///
/// The tokens cover the source exactly, in order: joining their text gives
/// the source back. A language nobody knows yields the source as one plain
/// token, so it is always safe to call with whatever a user or a document
/// supplied.
pub fn tokenize<'a>(source: &'a str, language: &str) -> Vec<Token<'a>> {
    #[cfg(feature = "highlight-syntect")]
    let spans = syntect::lex(source, language);
    #[cfg(not(feature = "highlight-syntect"))]
    let spans = None;

    let spans = spans.or_else(|| Language::from_name(language).map(|language| lexer::lex(source, language)));
    match spans {
        Some(spans) => spans.into_iter().map(|(start, end, kind)| Token { text: &source[start..end], kind }).collect(),
        None if source.is_empty() => Vec::new(),
        None => vec![Token { text: source, kind: None }],
    }
}

/// The name to show for the language called `language`, such as `Rust` for
/// `rs`. `None` when no lexer knows it.
pub fn language_label(language: &str) -> Option<AttrValue> {
    #[cfg(feature = "highlight-syntect")]
    if let Some(label) = syntect::label(language) {
        return Some(label.into());
    }
    Language::from_name(language).map(|language| AttrValue::Static(language.label()))
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
