use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{attrs, icons};

#[test]
fn renders_a_native_button_with_its_label() {
    let html = render(|| html! { <Button>{ "Save" }</Button> });
    assert!(html.starts_with("<button"));
    assert!(html.contains(r#"type="button""#));
    assert!(html.contains("Save"));
}

#[test]
fn href_renders_a_link() {
    let html = render(|| html! { <Button href="/docs">{ "Docs" }</Button> });
    assert!(html.starts_with("<a"));
    assert!(html.contains(r#"href="/docs""#));
}

#[test]
fn every_variant_is_distinguishable() {
    let rendered: Vec<String> = ButtonVariant::ALL
        .into_iter()
        .map(|variant| render(move || html! { <Button {variant}>{ "Go" }</Button> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn disabled_uses_the_native_attribute() {
    let html = render(|| html! { <Button disabled=true>{ "Save" }</Button> });
    assert!(html.contains(" disabled"));
}

#[test]
fn loading_is_announced_and_stays_focusable() {
    let html = render(|| html! { <Button loading=true>{ "Save" }</Button> });
    assert!(html.contains(r#"aria-busy="true""#));
    assert!(html.contains(r#"aria-disabled="true""#));
    assert!(!html.contains(" disabled"));
}

#[test]
fn an_inert_link_cannot_be_followed() {
    let html = render(|| html! { <Button href="/docs" disabled=true>{ "Docs" }</Button> });
    assert!(!html.contains("href="));
    assert!(html.contains(r#"aria-disabled="true""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! {
            <Button id="save" class="mine" attrs={attrs! { "aria-describedby" => "hint" }} icon={icons::CHECK}>
                { "Save" }
            </Button>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="save""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"aria-describedby="hint""#));
}
