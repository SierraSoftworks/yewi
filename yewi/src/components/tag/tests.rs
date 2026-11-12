use yewi_test::render;

use super::*;
use crate::attrs;

/// The value of `attr` on the first element that has it.
fn attr<'a>(html: &'a str, attr: &str) -> Option<&'a str> {
    let start = html.find(&format!(" {attr}=\""))? + attr.len() + 3;
    Some(&html[start..start + html[start..].find('"')?])
}

#[test]
fn renders_its_label_and_no_button_by_default() {
    let html = render(|| html! { <Tag>{ "linux" }</Tag> });
    assert!(html.contains("linux"));
    assert!(!html.contains("<button"));
}

#[test]
fn name_is_shown_before_the_value() {
    let html = render(|| html! { <Tag name="env">{ "production" }</Tag> });
    assert!(html.find("env").unwrap() < html.find("production").unwrap());
}

#[test]
fn removable_tags_name_their_button_after_the_tag() {
    let html = render(|| html! { <Tag name="env" on_remove={Callback::noop()}>{ "production" }</Tag> });
    let button = &html[html.find("<button").expect("a remove button")..];
    assert!(button.contains(r#"type="button""#));
    assert_eq!(attr(button, "aria-label"), Some("Remove"));

    // The name is the button's own label followed by the tag's text.
    let labelled_by: Vec<&str> = attr(button, "aria-labelledby").unwrap().split(' ').collect();
    assert_eq!(labelled_by.len(), 2);
    assert_eq!(attr(button, "id"), Some(labelled_by[0]));
    let text = &html[html.find(&format!(r#"id="{}""#, labelled_by[1])).expect("the labelling element")..];
    assert!(text.contains("env") && text.contains("production"));
}

#[test]
fn each_tag_gets_its_own_ids() {
    let html = render(|| {
        html! {
            <>
                <Tag on_remove={Callback::noop()}>{ "a" }</Tag>
                <Tag on_remove={Callback::noop()}>{ "b" }</Tag>
            </>
        }
    });
    let names: Vec<_> =
        html.match_indices(" aria-labelledby=").map(|(at, _)| attr(&html[at..], "aria-labelledby")).collect();
    assert_eq!(names.len(), 2);
    assert_ne!(names[0], names[1]);
}

#[test]
fn disabled_uses_the_native_attribute_on_the_button() {
    let html = render(|| html! { <Tag disabled=true on_remove={Callback::noop()}>{ "locked" }</Tag> });
    assert!(html[html.find("<button").unwrap()..].contains(" disabled"));
}

#[test]
fn href_links_the_label() {
    let html = render(|| html! { <Tag href="/hosts?os=linux">{ "linux" }</Tag> });
    assert!(html.contains(r#"href="/hosts?os=linux""#));
}

#[test]
fn tones_are_distinguishable() {
    let rendered: Vec<String> =
        Tone::ALL.into_iter().map(|tone| render(move || html! { <Tag {tone}>{ "tag" }</Tag> })).collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| html! { <Tag id="os" class="mine" attrs={attrs! { "title" => "Operating system" }}>{ "linux" }</Tag> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="os""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"title="Operating system""#));
}
