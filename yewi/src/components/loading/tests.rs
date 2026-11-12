use yewi_test::render;

use super::*;
use crate::{Size, attrs};

#[test]
fn loading_is_a_status_with_a_default_label() {
    let html = render(|| html! { <Loading /> });
    assert!(html.contains(r#"role="status""#));
    assert!(html.contains("Loading"));
}

#[test]
fn a_hidden_label_is_still_rendered_for_assistive_technology() {
    let html = render(|| html! { <Loading label="Loading hosts" label_hidden=true /> });
    assert!(html.contains("Loading hosts"));
}

#[test]
fn the_spinner_is_decorative() {
    let html = render(|| html! { <Spinner /> });
    assert!(html.contains(r#"aria-hidden="true""#));
    assert!(!html.contains("role="));
}

#[test]
fn layouts_sizes_and_delay_are_distinguishable() {
    let rendered = [
        render(|| html! { <Loading /> }),
        render(|| html! { <Loading layout={LoadingLayout::Block} /> }),
        render(|| html! { <Loading size={Size::Small} /> }),
        render(|| html! { <Loading size={Size::Large} /> }),
        render(|| html! { <Loading delay=true /> }),
    ];
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn an_idle_overlay_shows_only_its_content() {
    let html = render(|| html! { <LoadingOverlay>{ "Three hosts" }</LoadingOverlay> });
    assert!(html.contains("Three hosts"));
    assert!(!html.contains("aria-busy"));
    assert!(!html.contains(" inert"));
    assert!(!html.contains(r#"role="status""#));
}

#[test]
fn an_active_overlay_is_busy_and_makes_its_content_inert() {
    let html = render(|| html! { <LoadingOverlay active=true label="Refreshing">{ "Three hosts" }</LoadingOverlay> });
    assert!(html.contains(r#"aria-busy="true""#));
    assert!(html.contains(" inert"));
    assert!(html.contains("Three hosts"));
    assert!(html.contains(r#"role="status""#));
    assert!(html.contains("Refreshing"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let roots = [
        render(|| html! { <Spinner id="x" class="mine" attrs={attrs! { "data-test" => "1" }} /> }),
        render(|| html! { <Loading id="x" class="mine" attrs={attrs! { "data-test" => "1" }} /> }),
        render(|| html! { <LoadingOverlay id="x" class="mine" attrs={attrs! { "data-test" => "1" }} /> }),
    ];
    for html in roots {
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="x""#));
        assert!(root.contains("mine"));
        assert!(root.contains(r#"data-test="1""#));
    }
}
