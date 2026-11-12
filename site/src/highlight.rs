//! A small Rust tokeniser for example source. It is a stopgap until the
//! `CodeBlock` component and its lexers land; then this module goes away.

use yew::prelude::*;

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
    "struct", "super", "trait", "true", "type", "use", "where", "while",
];

fn class_of(word: &str, next: Option<char>) -> Option<&'static str> {
    if KEYWORDS.contains(&word) {
        Some("tok-kw")
    } else if next == Some('!') {
        Some("tok-macro")
    } else if word.starts_with(char::is_uppercase) {
        Some("tok-type")
    } else if word.starts_with(|c: char| c.is_ascii_digit()) {
        Some("tok-num")
    } else {
        None
    }
}

/// Renders `source` as highlighted spans.
pub fn rust(source: &str) -> Html {
    let mut out: Vec<Html> = Vec::new();
    let mut plain = String::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;

    let flush = |plain: &mut String, out: &mut Vec<Html>| {
        if !plain.is_empty() {
            out.push(html! { { std::mem::take(plain) } });
        }
    };

    while i < chars.len() {
        let c = chars[i];
        let start = i;
        let class = if c == '/' && chars.get(i + 1) == Some(&'/') {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            Some("tok-comment")
        } else if c == '"' {
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                i += if chars[i] == '\\' { 2 } else { 1 };
            }
            i = (i + 1).min(chars.len());
            Some("tok-str")
        } else if c.is_alphanumeric() || c == '_' {
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            class_of(&word, chars.get(i).copied())
        } else {
            i += 1;
            None
        };

        let text: String = chars[start..i].iter().collect();
        match class {
            Some(class) => {
                flush(&mut plain, &mut out);
                out.push(html! { <span {class}>{ text }</span> });
            }
            None => plain.push_str(&text),
        }
    }
    flush(&mut plain, &mut out);
    out.into_iter().collect()
}
