use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_title_subtitle_and_body() {
    let html = render(|| html! { <Card title="Hosts" subtitle="31 enrolled">{ "Body text" }</Card> });
    assert!(html.contains("<h2"));
    for text in ["Hosts", "31 enrolled", "Body text"] {
        assert!(html.contains(text), "missing {text}");
    }
}

#[test]
fn a_bare_card_has_no_header() {
    let html = render(|| html! { <Card>{ "Body text" }</Card> });
    assert!(!html.contains("<header"));
    assert!(!html.contains("<footer"));
}

#[test]
fn actions_and_footer_are_rendered_when_given() {
    let html = render(|| {
        html! {
            <Card actions={html! { <button>{ "Edit" }</button> }} footer={html! { <span>{ "Saved" }</span> }}>
                { "Body" }
            </Card>
        }
    });
    assert!(html.contains("<header") && html.contains("Edit"));
    assert!(html.contains("<footer") && html.contains("Saved"));
}

#[test]
fn level_sets_the_heading_element_and_is_clamped() {
    for (level, tag) in [(1, "<h1"), (3, "<h3"), (0, "<h1"), (9, "<h6")] {
        let html = render(move || html! { <Card title="Hosts" {level} /> });
        assert!(html.contains(tag), "level {level} should render {tag}");
    }
}

#[test]
fn every_header_style_is_distinguishable() {
    let rendered: Vec<String> =
        CardHeader::ALL.into_iter().map(|header| render(move || html! { <Card title="Hosts" {header} /> })).collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Card id="hosts" class="mine" attrs={attrs! { "aria-label" => "Hosts" }}>{ "Body" }</Card> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="hosts""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"aria-label="Hosts""#));
}
