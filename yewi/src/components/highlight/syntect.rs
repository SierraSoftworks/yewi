//! The `highlight-syntect` engine: syntect's grammars, mapped onto the same
//! token kinds, and so the same colours, as the built-in lexers.

use std::sync::LazyLock;

use syntect::{
    parsing::{ParseState, Scope, ScopeStack, SyntaxSet},
    util::LinesWithEndings,
};

use super::{TokenKind, lexer::Spans};

static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);

/// Scope prefixes and the kind they give a token. The innermost scope of a
/// token is tried first, and for each scope the first matching row wins. A
/// row with `None` stops the search and leaves the token plain.
static KINDS: LazyLock<Vec<(Scope, Option<TokenKind>)>> = LazyLock::new(|| {
    use TokenKind::*;
    [
        ("comment", Some(Comment)),
        ("constant.numeric", Some(Number)),
        ("constant", Some(Literal)),
        ("string", Some(String)),
        ("keyword.operator", None),
        ("keyword", Some(Keyword)),
        ("storage", Some(Keyword)),
        ("variable.language", Some(Keyword)),
        ("entity.name.function.decorator", Some(Meta)),
        ("variable.annotation", Some(Meta)),
        ("meta.annotation", Some(Meta)),
        ("meta.preprocessor", Some(Meta)),
        ("support.macro", Some(Meta)),
        ("entity.name.section", Some(Meta)),
        ("markup.heading", Some(Meta)),
        ("meta.diff", Some(Meta)),
        ("entity.name.function", Some(Function)),
        ("support.function", Some(Function)),
        ("variable.function", Some(Function)),
        ("entity.name.tag", Some(Tag)),
        ("entity.other.attribute-name", Some(Property)),
        ("support.type.property-name", Some(Property)),
        ("entity.name", Some(Type)),
        ("support.type", Some(Type)),
        ("support.class", Some(Type)),
        ("support.constant", Some(Literal)),
        ("markup.inserted", Some(Inserted)),
        ("markup.deleted", Some(Deleted)),
    ]
    .into_iter()
    .filter_map(|(prefix, kind)| Some((Scope::new(prefix).ok()?, kind)))
    .collect()
});

/// The scopes that mark a key in data: current grammars use the first, the
/// bundled JSON grammar a dictionary that is not inside its own value.
static KEYS: LazyLock<Vec<Scope>> = LazyLock::new(|| {
    ["meta.mapping.key", "meta.structure.dictionary", "meta.structure.dictionary.value"]
        .into_iter()
        .filter_map(|prefix| Scope::new(prefix).ok())
        .collect()
});

fn in_key(scopes: &[Scope]) -> bool {
    let [mapping_key, dictionary, value] = KEYS.as_slice() else { return false };
    scopes.iter().any(|scope| mapping_key.is_prefix_of(*scope))
        || scopes.iter().rev().find(|scope| dictionary.is_prefix_of(**scope)).is_some_and(|scope| !value.is_prefix_of(*scope))
}

fn kind(stack: &ScopeStack) -> Option<TokenKind> {
    let scopes = stack.as_slice();
    let found = scopes
        .iter()
        .rev()
        .find_map(|scope| KINDS.iter().find(|(prefix, _)| prefix.is_prefix_of(*scope)).map(|(_, kind)| *kind))
        .flatten();
    // A quoted key is a string inside a key: colour it as the key.
    if found == Some(TokenKind::String) && in_key(scopes) { Some(TokenKind::Property) } else { found }
}

/// Lexes `source` with the grammar named `language`. `None` when syntect has
/// no such grammar or cannot parse the text, so the caller can fall back.
pub(super) fn lex(source: &str, language: &str) -> Option<Spans> {
    let syntax = SYNTAXES.find_syntax_by_token(language.trim())?;
    let mut state = ParseState::new(syntax);
    let mut stack = ScopeStack::new();
    let mut out = Spans::new();
    let mut push = |start: usize, end: usize, kind: Option<TokenKind>| match out.last_mut() {
        _ if start >= end => {}
        Some(last) if last.2 == kind => last.1 = end,
        _ => out.push((start, end, kind)),
    };

    let mut offset = 0;
    for line in LinesWithEndings::from(source) {
        let mut at = 0;
        for (position, op) in state.parse_line(line, &SYNTAXES).ok()? {
            push(offset + at, offset + position, kind(&stack));
            at = at.max(position);
            stack.apply(&op).ok()?;
        }
        push(offset + at, offset + line.len(), kind(&stack));
        offset += line.len();
    }
    Some(out)
}

/// The display name of the grammar named `language`.
pub(super) fn label(language: &str) -> Option<String> {
    SYNTAXES.find_syntax_by_token(language.trim()).map(|syntax| syntax.name.clone())
}
