use yewi_test::render;

use super::*;

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

#[test]
fn labels_a_language_by_any_of_its_names() {
    assert_eq!(language_label("rs").as_deref(), Some("Rust"));
    assert_eq!(language_label("klingon"), None);
}
