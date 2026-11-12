use yewi_test::render;

use super::*;
use crate::attrs;

const SOURCE: &str = "fn main() {\n    println!(\"<hi>\");\n}\n";

/// The text between tags, as a reader would see it.
fn text_of(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => text.push(c),
            _ => {}
        }
    }
    text.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&amp;", "&")
}

#[test]
fn shows_the_code_escaped_in_a_pre_that_the_keyboard_can_scroll() {
    let html = render(|| html! { <CodeBlock code={SOURCE} language="rust" /> });
    assert!(html.contains("<pre") && html.contains("<code"));
    assert!(html.contains(r#"tabindex="0""#));
    assert!(!html.contains("<hi>"));
    assert!(text_of(&html).contains(SOURCE.trim_end()));
}

#[test]
fn the_header_names_the_file_and_language_and_offers_a_copy() {
    let html = render(|| html! { <CodeBlock code={SOURCE} language="rs" filename="src/main.rs" /> });
    assert!(html.contains("src/main.rs"));
    assert!(html.contains(r#"aria-label="Copy code""#));
    // The lexer's name for the language when highlighting is built in, otherwise the text as given.
    let label = if cfg!(feature = "highlight") { "Rust" } else { "rs" };
    assert!(text_of(&html).contains(label), "{html}");
}

#[test]
fn nothing_to_show_means_no_header() {
    let html = render(|| html! { <CodeBlock code={SOURCE} copy=false /> });
    assert!(!html.contains("<button"));
    assert!(!html.contains("__header"));

    let with_actions = render(|| {
        html! { <CodeBlock code={SOURCE} copy=false actions={html! { <button>{ "Run" }</button> }} /> }
    });
    assert!(with_actions.contains("Run"));
}

#[test]
fn line_mode_has_one_element_per_line_and_keeps_the_text() {
    let lines = |html: &str| html.matches("yi-code-block__line").count() - html.matches("__line--marked").count();
    let plain = render(|| html! { <CodeBlock code={"a\n\nb\n"} /> });
    assert_eq!(lines(&plain), 0);

    let numbered = render(|| html! { <CodeBlock code={"a\n\nb\n"} line_numbers=true /> });
    assert_eq!(lines(&numbered), 3);
    assert_eq!(text_of(&render(|| html! { <CodeBlock code={"a\n\nb\n"} line_numbers=true copy=false /> })), "a\n\nb");
}

#[test]
fn marks_the_listed_lines_and_ignores_nonsense() {
    for (spec, expected) in [("2", 1), ("1, 3-4", 3), ("2-9", 3), ("x, 0, 7-5, ", 0), ("", 0)] {
        let html = render(move || html! { <CodeBlock code={"a\nb\nc\nd"} marked={spec} /> });
        assert_eq!(html.matches("__line--marked").count(), expected, "{spec:?}");
    }
}

#[test]
fn a_forced_theme_is_set_on_the_block_alone() {
    let dark = render(|| html! { <CodeBlock code={SOURCE} theme={Theme::Dark} /> });
    assert!(dark.contains(r#"data-theme="dark""#));
    let following = render(|| html! { <CodeBlock code={SOURCE} /> });
    assert!(!following.contains("data-theme"));
}

#[cfg(feature = "highlight")]
#[test]
fn colours_known_languages_and_leaves_unknown_ones_plain() {
    let rust = render(|| html! { <CodeBlock code={SOURCE} language="rust" copy=false /> });
    assert!(rust.contains(crate::TokenKind::Keyword.class()));
    let unknown = render(|| html! { <CodeBlock code={SOURCE} language="klingon" copy=false /> });
    assert!(!unknown.contains("<span class=\"yi-tok"));
    assert!(text_of(&unknown).contains("klingon"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <CodeBlock id="sample" class="mine" attrs={attrs! { "aria-label" => "Sample" }} code={SOURCE} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="sample""#) && root.contains("mine") && root.contains(r#"aria-label="Sample""#));
}
