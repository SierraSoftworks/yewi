use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_title_subtitle_actions_and_body() {
    let html = render(|| {
        html! {
            <Panel title="Schedule" subtitle="Weekly" actions={html! { <button>{ "Edit" }</button> }}>
                { "Body text" }
            </Panel>
        }
    });
    assert!(html.contains("<h3"));
    for text in ["Schedule", "Weekly", "Edit", "Body text"] {
        assert!(html.contains(text), "missing {text}");
    }
}

#[test]
fn a_panel_without_a_heading_is_only_its_body() {
    let html = render(|| html! { <Panel>{ "Body text" }</Panel> });
    assert!(!html.contains("<header"));
    assert!(html.contains("Body text"));
}

#[test]
fn every_fill_is_distinguishable() {
    let rendered: Vec<String> =
        PanelFill::ALL.into_iter().map(|fill| render(move || html! { <Panel {fill}>{ "Body" }</Panel> })).collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn collapsible_uses_native_disclosure_and_is_open_by_default() {
    let html = render(|| html! { <Panel title="Advanced" collapsible=true>{ "Body text" }</Panel> });
    assert!(html.starts_with("<details"));
    assert!(html.contains("<summary"));
    assert!(html[..html.find('>').unwrap()].contains(" open"));
}

#[test]
fn collapsed_starts_closed_but_keeps_its_content_in_the_document() {
    let html = render(|| html! { <Panel title="Advanced" collapsible=true collapsed=true>{ "Body text" }</Panel> });
    assert!(!html[..html.find('>').unwrap()].contains(" open"));
    assert!(html.contains("Body text"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    for collapsible in [false, true] {
        let html = render(move || {
            html! {
                <Panel id="p" class="mine" title="T" {collapsible} attrs={attrs! { "data-section" => "a" }}>
                    { "Body" }
                </Panel>
            }
        });
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="p""#));
        assert!(root.contains("mine"));
        assert!(root.contains(r#"data-section="a""#));
    }
}
