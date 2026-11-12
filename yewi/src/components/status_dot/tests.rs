use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn is_decorative_without_a_label() {
    let html = render(|| html! { <StatusDot tone={Tone::Ok} /> });
    assert!(html.contains(r#"aria-hidden="true""#));
    assert!(!html.contains("role="));
}

#[test]
fn a_label_exposes_it_as_an_image() {
    let html = render(|| html! { <StatusDot tone={Tone::Danger} label="Failing" /> });
    assert!(html.contains(r#"role="img""#));
    assert!(html.contains(r#"aria-label="Failing""#));
    assert!(!html.contains("aria-hidden"));
}

#[test]
fn every_tone_and_the_inherited_colour_are_distinguishable() {
    let mut rendered = vec![render(|| html! { <StatusDot /> })];
    rendered.extend(Tone::ALL.into_iter().map(|tone| render(move || html! { <StatusDot {tone} /> })));
    rendered.push(render(|| html! { <StatusDot active=true /> }));
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn size_id_class_and_attrs_reach_the_root_element() {
    let html = render(|| html! { <StatusDot id="live" class="mine" size="12px" attrs={attrs! { "title" => "Live" }} /> });
    assert!(html.contains(r#"id="live""#));
    assert!(html.contains("mine"));
    assert!(html.contains("12px"));
    assert!(html.contains(r#"title="Live""#));
}
