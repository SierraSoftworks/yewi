use yewi_test::render;

use super::*;
use crate::attrs;

/// The opening tag of the first `<button>` in `html`.
fn button_tag(html: &str) -> &str {
    let start = html.find("<button").expect("no button rendered");
    &html[start..start + html[start..].find('>').unwrap()]
}

/// The value of `name` in an opening tag.
fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let start = tag.find(&format!(r#" {name}=""#))? + name.len() + 3;
    Some(&tag[start..start + tag[start..].find('"')?])
}

#[test]
fn shows_the_value_beside_a_named_copy_button() {
    let html = render(|| html! { <Copyable value="4f9a2c1e" /> });
    assert!(html.contains("4f9a2c1e"));
    assert_eq!(attr(button_tag(&html), "aria-label"), Some("Copy"));
    assert_eq!(attr(button_tag(&html), "type"), Some("button"));
}

#[test]
fn can_show_something_shorter_than_it_copies() {
    let html = render(|| html! { <Copyable value="4f9a2c1e8b7d">{ "4f9a2c1" }</Copyable> });
    assert!(html.contains(">4f9a2c1<"));
    assert!(!html.contains(">4f9a2c1e8b7d<"));
}

#[test]
fn the_button_is_described_by_the_value_it_copies() {
    let html = render(|| html! { <Copyable value="4f9a2c1e" /> });
    let described_by = attr(button_tag(&html), "aria-describedby").expect("no description");
    assert!(html.contains(&format!(r#"id="{described_by}""#)));
}

#[test]
fn every_variant_is_distinguishable() {
    let rendered: Vec<String> = CopyableVariant::ALL
        .into_iter()
        .map(|variant| render(move || html! { <Copyable {variant} value="abc" /> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn a_truncated_value_stays_readable_as_a_tooltip() {
    let html = render(|| html! { <Copyable truncate=true value="https://example.com/a/very/long/path" /> });
    assert!(html.contains(r#"title="https://example.com/a/very/long/path""#));
}

#[test]
fn labels_can_be_replaced() {
    let html = render(|| html! { <Copyable value="abc" label="Kopieren" /> });
    assert_eq!(attr(button_tag(&html), "aria-label"), Some("Kopieren"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Copyable value="abc" id="sha" class="mine" attrs={attrs! { "data-row" => "7" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="sha""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-row="7""#));
}

#[test]
fn an_icon_only_copy_button_is_named_by_its_label() {
    let html = render(|| html! { <CopyButton value="abc" label="Copy token" /> });
    assert!(html.starts_with("<button"));
    assert_eq!(attr(button_tag(&html), "aria-label"), Some("Copy token"));
}

#[test]
fn a_labelled_copy_button_is_named_by_its_text() {
    let html = render(|| html! { <CopyButton value="abc">{ "Copy command" }</CopyButton> });
    assert!(html.contains("Copy command"));
    assert_eq!(attr(button_tag(&html), "aria-label"), None);
}

#[test]
fn a_disabled_copy_button_uses_the_native_attribute() {
    let html = render(|| html! { <CopyButton value="abc" disabled=true /> });
    assert!(button_tag(&html).contains(" disabled"));
}

#[test]
fn copy_button_attrs_reach_the_button() {
    let html = render(|| html! { <CopyButton value="abc" id="copy" attrs={attrs! { "data-kind" => "sha" }} /> });
    assert_eq!(attr(button_tag(&html), "id"), Some("copy"));
    assert_eq!(attr(button_tag(&html), "data-kind"), Some("sha"));
}

#[test]
fn starts_idle_so_server_and_client_agree() {
    let html = render(|| html! { <Copyable value="abc" /> });
    assert!(!html.contains("data-yi-state"));
    assert_eq!(CopyState::default().as_attr(), None);
}
