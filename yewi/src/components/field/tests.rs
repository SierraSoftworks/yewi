use yewi_test::render;

use super::*;
use crate::{TextInput, attrs, hooks::reset_ids};

/// The value of `name` on the first `<tag ...>` in `html`.
fn attr<'a>(html: &'a str, tag: &str, name: &str) -> Option<&'a str> {
    let start = html.find(&format!("<{tag} "))?;
    let element = &html[start..start + html[start..].find('>')?];
    let value = &element[element.find(&format!(" {name}=\""))? + name.len() + 3..];
    Some(&value[..value.find('"')?])
}

fn render_field(view: fn() -> Html) -> String {
    reset_ids();
    render(view)
}

#[test]
fn the_label_points_at_the_control() {
    let html = render_field(|| html! { <Field label="Email"><TextInput /></Field> });
    assert!(html.contains("Email"));
    let target = attr(&html, "label", "for").expect("label has `for`");
    assert_eq!(attr(&html, "input", "id"), Some(target));
}

#[test]
fn control_id_overrides_the_generated_id() {
    let html = render_field(|| html! { <Field label="Email" control_id="email"><TextInput id="ignored" /></Field> });
    assert_eq!(attr(&html, "label", "for"), Some("email"));
    assert_eq!(attr(&html, "input", "id"), Some("email"));
}

#[test]
fn the_hint_describes_the_control() {
    let html = render_field(|| html! { <Field label="Email" hint="Work address"><TextInput /></Field> });
    let described_by = attr(&html, "input", "aria-describedby").expect("control is described");
    assert_eq!(attr(&html, "p", "id"), Some(described_by));
    assert!(html.contains("Work address"));
    assert!(!html.contains("aria-invalid"));
}

#[test]
fn an_error_replaces_the_hint_and_marks_the_control_invalid() {
    let html =
        render_field(|| html! { <Field label="Email" hint="Work address" error="Not an address"><TextInput /></Field> });
    assert!(html.contains("Not an address"));
    assert!(!html.contains("Work address"));
    assert!(html.contains(r#"role="alert""#));
    assert_eq!(attr(&html, "input", "aria-invalid"), Some("true"));
    assert_eq!(attr(&html, "input", "aria-describedby"), attr(&html, "p", "id"));
}

#[test]
fn required_and_disabled_reach_the_control() {
    let html = render_field(|| html! { <Field label="Name" required=true disabled=true><TextInput /></Field> });
    let input = &html[html.find("<input").unwrap()..];
    assert!(input.contains(" required"));
    assert!(input.contains(" disabled"));
}

#[test]
fn a_hidden_label_is_still_rendered() {
    let html = render_field(|| html! { <Field label="Filter" hide_label=true><TextInput /></Field> });
    assert!(html.contains("Filter"));
    assert!(attr(&html, "label", "for").is_some());
}

#[test]
fn a_group_is_labelled_as_a_whole_and_shares_no_id() {
    let html = render_field(|| {
        html! { <Field label="Range" hint="In ms" group=true><TextInput /><TextInput /></Field> }
    });
    assert!(html.contains(r#"role="group""#));
    assert_eq!(attr(&html, "div", "aria-labelledby"), attr(&html, "span", "id"));
    assert!(!html.contains("<label"));
    assert_eq!(attr(&html, "input", "id"), None);
}

#[test]
fn a_control_outside_a_field_uses_its_own_props() {
    let html = render_field(|| html! { <TextInput id="name" invalid=true /> });
    assert_eq!(attr(&html, "input", "id"), Some("name"));
    assert_eq!(attr(&html, "input", "aria-invalid"), Some("true"));
    assert!(!html.contains("aria-describedby"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render_field(|| {
        html! {
            <Field label="Name" id="name-field" class="mine" attrs={attrs! { "data-row" => "1" }}>
                <TextInput />
            </Field>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="name-field""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-row="1""#));
}
