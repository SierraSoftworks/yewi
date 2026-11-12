use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn a_plain_divider_is_a_native_rule() {
    let html = render(|| html! { <Divider /> });
    assert!(html.starts_with("<hr"));
}

#[test]
fn a_vertical_divider_announces_its_orientation() {
    let html = render(|| html! { <Divider orientation={DividerOrientation::Vertical} /> });
    assert!(html.contains(r#"role="separator""#));
    assert!(html.contains(r#"aria-orientation="vertical""#));
}

#[test]
fn a_label_stays_readable_by_assistive_technology() {
    let html = render(|| html! { <Divider>{ "Yesterday" }</Divider> });
    assert!(html.contains("Yesterday"));
    assert!(!html.contains("role=\"separator\""));
    assert!(!html.contains("aria-hidden"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    for orientation in [DividerOrientation::Horizontal, DividerOrientation::Vertical] {
        let html = render(move || {
            html! { <Divider id="d" class="mine" {orientation} attrs={attrs! { "data-kind" => "rule" }} /> }
        });
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="d""#));
        assert!(root.contains("mine"));
        assert!(root.contains(r#"data-kind="rule""#));
    }
}
