use yew::prelude::*;
use yewi_test::render;

use super::*;

/// The text of every token of `kind`, in order.
fn of_kind<'a>(tokens: &[Token<'a>], kind: TokenKind) -> Vec<&'a str> {
    tokens.iter().filter(|token| token.kind == Some(kind)).map(|token| token.text).collect()
}

const SAMPLES: &[(&str, &str)] = &[
    ("rust", "#[derive(Debug)]\npub fn main<'a>() -> Result<(), Error> {\n    let c = 'x'; // done\n    println!(\"{}\", r#\"raw \"text\"\"#);\n}\n"),
    ("javascript", "export const greet = (name) => `Hello ${name}`; /* block */\nif (x !== null) { console.log('hi', 42); }\n"),
    ("typescript", "interface User { readonly id: number; name: string }\n@Component()\nclass View<T> {}\n"),
    ("python", "@app.route(\"/\")\ndef index(request: Request) -> str:\n    \"\"\"Doc\nstring.\"\"\"\n    return f'{request.path}'  # comment\n"),
    ("go", "package main\n\nfunc main() {\n\tfmt.Println(`raw`, 'x', nil)\n}\n"),
    ("bash", "#!/bin/sh\nexport NAME=\"world\"\nif [ -n $NAME ]; then\n  echo \"hello ${NAME}\" | tr a-z A-Z # shout\nfi\ncargo build \\\n  --release\n"),
    ("sql", "SELECT id, count(*) FROM users WHERE name = 'o''neil' -- note\nLIMIT 10;\n"),
    ("css", "@media (width >= 40rem) {\n  .card:hover { color: #1a2b3c; margin: -1px var(--gap); }\n}\n$accent: red; // scss\n"),
    ("html", "<!doctype html>\n<!-- note -->\n<a href=\"/docs\" hidden>Tom &amp; Jerry</a>\n<style>a { color: red; }</style>\n<script>let x = 1;</script>\n"),
    ("json", "{\n  \"name\": \"yewi\",\n  \"count\": 3,\n  \"ok\": true\n}\n"),
    ("toml", "[package]\nname = \"yewi\" # comment\nedition.workspace = true\nbuilt = 2026-01-15T12:00:00Z\n"),
    ("yaml", "# comment\nservice:\n  image: ghcr.io/caddy:2.8\n  ports: [80, 443]\n  note: don't panic\n  enabled: true\n"),
    ("hcl", "job \"docs\" {\n  count = 2 # two\n  image = file(\"x\")\n}\n"),
    ("diff", "--- a/x\n+++ b/x\n@@ -1 +1 @@\n-old\n+new\n same\n"),
    ("unknown-language", "anything at all\n"),
    ("rust", "unterminated \"string and 'c and /* comment"),
    ("html", "<a href=\"unterminated"),
    ("rust", "héllo → 'é' \"ünï\" // çømment"),
    ("bash", ""),
];

#[test]
fn tokens_always_cover_the_source_exactly() {
    for (language, source) in SAMPLES {
        let tokens = tokenize(source, language);
        let joined: String = tokens.iter().map(|token| token.text).collect();
        assert_eq!(&joined, source, "{language}");
        assert!(tokens.iter().all(|token| !token.text.is_empty()), "{language}: empty token");
        // Every prefix must lex without panicking, as it does while someone types.
        for (end, _) in source.char_indices() {
            tokenize(&source[..end], language);
        }
    }
}

// The built-in lexers. With syntect on, most of these languages are its job.
#[cfg(not(feature = "highlight-syntect"))]
#[test]
fn recognises_the_main_token_kinds_of_each_language() {
    use TokenKind::*;
    let cases: &[(&str, &str, TokenKind, &[&str])] = &[
        ("rust", SAMPLES[0].1, Keyword, &["fn", "let"]),
        ("rust", SAMPLES[0].1, Comment, &["// done"]),
        ("rust", SAMPLES[0].1, String, &["'x'", "\"{}\"", "r#\"raw \"text\"\"#"]),
        ("rust", SAMPLES[0].1, Meta, &["#[derive(Debug)]", "'a", "println!"]),
        ("rust", SAMPLES[0].1, Type, &["Result", "Error"]),
        ("rs", "let n = 1_000u32 + 0.5;", Number, &["1_000u32", "0.5"]),
        ("javascript", SAMPLES[1].1, String, &["`Hello ${name}`", "'hi'"]),
        ("javascript", SAMPLES[1].1, Comment, &["/* block */"]),
        ("javascript", SAMPLES[1].1, Literal, &["null"]),
        ("ts", SAMPLES[2].1, Keyword, &["interface", "readonly", "class"]),
        ("ts", SAMPLES[2].1, Meta, &["@Component"]),
        ("python", SAMPLES[3].1, String, &["\"/\"", "\"\"\"Doc\nstring.\"\"\"", "f'{request.path}'"]),
        ("python", SAMPLES[3].1, Comment, &["# comment"]),
        ("go", SAMPLES[4].1, String, &["`raw`", "'x'"]),
        ("sh", SAMPLES[5].1, Function, &["echo", "tr", "cargo"]),
        ("sh", SAMPLES[5].1, Variable, &["NAME", "$NAME"]),
        ("sh", SAMPLES[5].1, Comment, &["#!/bin/sh", "# shout"]),
        ("SQL", SAMPLES[6].1, Keyword, &["SELECT", "FROM", "WHERE", "LIMIT"]),
        ("sql", SAMPLES[6].1, Comment, &["-- note"]),
        ("scss", SAMPLES[7].1, Property, &["color", "margin"]),
        ("scss", SAMPLES[7].1, Number, &["40rem", "#1a2b3c", "1px"]),
        ("scss", SAMPLES[7].1, Keyword, &["@media"]),
        ("html", SAMPLES[8].1, Tag, &["a", "a", "style", "style", "script", "script"]),
        ("html", SAMPLES[8].1, Property, &["href", "hidden", "color"]),
        ("html", SAMPLES[8].1, Comment, &["<!-- note -->"]),
        ("html", SAMPLES[8].1, Keyword, &["let"]),
        ("json", SAMPLES[9].1, Property, &["\"name\"", "\"count\"", "\"ok\""]),
        ("json", SAMPLES[9].1, String, &["\"yewi\""]),
        ("toml", SAMPLES[10].1, Meta, &["[package]"]),
        ("toml", SAMPLES[10].1, Property, &["name", "edition.workspace"]),
        ("toml", SAMPLES[10].1, Number, &["2026-01-15T12:00:00Z"]),
        ("yml", SAMPLES[11].1, Property, &["service", "image", "ports", "note", "enabled"]),
        ("nomad", SAMPLES[12].1, Keyword, &["job"]),
        ("nomad", SAMPLES[12].1, Property, &["count", "image"]),
        ("diff", SAMPLES[13].1, Inserted, &["+new"]),
        ("diff", SAMPLES[13].1, Deleted, &["-old"]),
    ];
    for (language, source, kind, expected) in cases {
        let found = of_kind(&tokenize(source, language), *kind);
        for text in *expected {
            assert!(found.contains(text), "{language}: {text:?} is not {kind:?}; those are {found:?}");
        }
    }
    // An apostrophe in prose does not open a string.
    assert_eq!(of_kind(&tokenize(SAMPLES[11].1, "yaml"), String), Vec::<&str>::new());
}

#[test]
fn languages_are_found_by_name_alias_and_case() {
    for language in Language::ALL {
        assert_eq!(Language::from_name(language.name()), Some(language));
        assert_eq!(Language::from_name(&language.name().to_uppercase()), Some(language));
        assert!(language_label(language.name()).is_some());
    }
    for (alias, language) in [("rs", Language::Rust), ("yml", Language::Yaml), ("sh", Language::Bash), (" TSX ", Language::TypeScript)]
    {
        assert_eq!(Language::from_name(alias), Some(language));
    }
    assert_eq!(Language::from_name("klingon"), None);
    assert_eq!(language_label("klingon"), None);
}

#[test]
fn renders_the_code_with_coloured_tokens_and_escapes_it() {
    let html = render(|| html! { <Highlight language="rust" code={"let a = \"<b>\"; // x"} /> });
    assert!(html.starts_with("<code"));
    assert!(html.contains(TokenKind::Keyword.class()));
    assert!(html.contains(TokenKind::Comment.class()));
    assert!(html.contains("&lt;b&gt;") && !html.contains("<b>"));
}

#[test]
fn an_unknown_language_renders_plain_text() {
    let html = render(|| html! { <Highlight language="klingon" code="let a = 1;" /> });
    assert!(html.contains("let a = 1;"));
    assert!(!html.contains("<span"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Highlight id="snippet" class="mine" attrs={crate::attrs! { "data-x" => "1" }} code="x" /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="snippet""#) && root.contains("mine") && root.contains(r#"data-x="1""#));
}

#[cfg(feature = "highlight-syntect")]
#[test]
fn syntect_lexes_what_it_knows_and_the_built_in_lexers_the_rest() {
    use TokenKind::*;
    let cases: &[(&str, &str, TokenKind, &str)] = &[
        ("rust", SAMPLES[0].1, Keyword, "fn"),
        ("rust", SAMPLES[0].1, Comment, "// done"),
        ("rust", SAMPLES[0].1, String, "raw"),
        ("java", "class A { int n = 42; /* c */ }", Keyword, "class"),
        ("java", "class A { int n = 42; /* c */ }", Number, "42"),
        ("c", "#include <stdio.h>\nint main(void) { return 0; }", Keyword, "return"),
        ("json", SAMPLES[9].1, Property, "name"),
        ("diff", SAMPLES[13].1, Inserted, "+new"),
        // Not among syntect's default grammars.
        ("toml", SAMPLES[10].1, Property, "name"),
        ("hcl", SAMPLES[12].1, Keyword, "job"),
    ];
    for (language, source, kind, text) in cases {
        let found = of_kind(&tokenize(source, language), *kind);
        assert!(found.iter().any(|token| token.contains(text)), "{language}: {text:?} is not {kind:?}; those are {found:?}");
    }
    assert_eq!(language_label("java").as_deref(), Some("Java"));
}
