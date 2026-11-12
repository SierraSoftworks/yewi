use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_inline_text_by_default() {
    let html = render(|| html! { <Text>{ "3 hosts" }</Text> });
    assert!(html.starts_with("<span"));
    assert!(html.contains("3 hosts"));
}

#[test]
fn tag_selects_the_element() {
    for (tag, element) in [
        (TextTag::Span, "<span"),
        (TextTag::P, "<p"),
        (TextTag::Div, "<div"),
        (TextTag::Strong, "<strong"),
        (TextTag::Em, "<em"),
        (TextTag::Small, "<small"),
    ] {
        let html = render(move || html! { <Text {tag}>{ "x" }</Text> });
        assert!(html.starts_with(element), "{tag:?} rendered {html}");
    }
}

/// Renders one `Text` per value and checks no two look alike.
fn assert_distinct(rendered: Vec<String>) {
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn every_size_color_and_weight_is_distinguishable() {
    assert_distinct(TextSize::ALL.into_iter().map(|size| render(move || html! { <Text {size}>{ "x" }</Text> })).collect());
    assert_distinct(
        TextColor::ALL.into_iter().map(|color| render(move || html! { <Text {color}>{ "x" }</Text> })).collect(),
    );
    assert_distinct(
        TextWeight::ALL.into_iter().map(|weight| render(move || html! { <Text {weight}>{ "x" }</Text> })).collect(),
    );
}

#[test]
fn tone_wins_over_color() {
    let toned = render(|| html! { <Text tone={Tone::Danger} color={TextColor::Muted}>{ "x" }</Text> });
    let tone_only = render(|| html! { <Text tone={Tone::Danger}>{ "x" }</Text> });
    assert_eq!(toned, tone_only);
}

#[test]
fn lines_sets_the_clamp_and_truncate_overrides_it() {
    let clamped = render(|| html! { <Text lines={3}>{ "x" }</Text> });
    assert!(clamped.contains("--yi-text-lines:3"));

    let truncated = render(|| html! { <Text lines={3} truncate=true>{ "x" }</Text> });
    assert!(!truncated.contains("--yi-text-lines"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| html! { <Text id="t" class="mine" attrs={attrs! { "title" => "Full" }}>{ "x" }</Text> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="t""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"title="Full""#));
}
