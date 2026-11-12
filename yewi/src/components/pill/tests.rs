use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

/// How many decorative dots the markup holds.
fn dots(html: &str) -> usize {
    html.matches(r#"aria-hidden="true""#).count()
}

#[test]
fn renders_its_label_with_a_decorative_dot() {
    let html = render(|| html! { <Pill tone={Tone::Ok}>{ "healthy" }</Pill> });
    assert!(html.starts_with("<span"));
    assert!(html.contains("healthy"));
    assert_eq!(dots(&html), 1);
}

#[test]
fn the_dot_can_be_left_out() {
    let html = render(|| html! { <Pill dot=false>{ "offline" }</Pill> });
    assert_eq!(dots(&html), 0);
}

#[test]
fn tones_and_emphasis_are_distinguishable() {
    let mut rendered: Vec<String> =
        Tone::ALL.into_iter().map(|tone| render(move || html! { <Pill {tone}>{ "state" }</Pill> })).collect();
    rendered.push(render(|| html! { <Pill bold=true>{ "state" }</Pill> }));
    rendered.push(render(|| html! { <Pill active=true>{ "state" }</Pill> }));
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn href_renders_a_link() {
    let html = render(|| html! { <Pill href="/rollouts/42">{ "rolling" }</Pill> });
    assert!(html.starts_with("<a"));
    assert!(html.contains(r#"href="/rollouts/42""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Pill id="state" class="mine" attrs={attrs! { "title" => "Since 09:00" }}>{ "held" }</Pill> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="state""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"title="Since 09:00""#));
}
