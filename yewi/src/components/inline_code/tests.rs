use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_a_code_element_with_its_text() {
    let html = render(|| html! { <InlineCode>{ "cargo xtask ci" }</InlineCode> });
    assert!(html.starts_with("<code"));
    assert!(html.contains("cargo xtask ci"));
}

#[test]
fn markup_in_the_text_is_escaped() {
    let html = render(|| html! { <InlineCode>{ "<Styles />" }</InlineCode> });
    assert!(html.contains("&lt;Styles /&gt;"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html =
        render(|| html! { <InlineCode id="c" class="mine" attrs={attrs! { "translate" => "no" }}>{ "x" }</InlineCode> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="c""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"translate="no""#));
}
