use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{Field, attrs, hooks::reset_ids};

/// The opening tag of the first `<input>` in `html`.
fn input(html: &str) -> &str {
    let start = html.find("<input").expect("renders an input");
    &html[start..start + html[start..].find('>').unwrap()]
}

#[test]
fn renders_a_native_checkbox_inside_its_label() {
    let html = render(|| html! { <Checkbox name="tls" value="yes">{ "Require TLS" }</Checkbox> });
    assert!(html.starts_with("<label"));
    assert!(html.contains("Require TLS"));
    let input = input(&html);
    assert!(input.contains(r#"type="checkbox""#));
    assert!(input.contains(r#"name="tls""#));
    assert!(input.contains(r#"value="yes""#));
    assert!(!input.contains(" checked"));
}

#[test]
fn states_use_native_attributes() {
    let html = render(|| html! { <Checkbox checked=true disabled=true required=true invalid=true /> });
    for expected in [" checked", " disabled", " required", r#"aria-invalid="true""#] {
        assert!(input(&html).contains(expected), "{expected} missing from {html}");
    }
}

#[test]
fn a_description_describes_the_input() {
    reset_ids();
    let html = render(|| html! { <Checkbox description="Applies to new deploys.">{ "Canary" }</Checkbox> });
    assert!(html.contains("Applies to new deploys."));
    let id = input(&html).split(r#"aria-describedby=""#).nth(1).expect("is described");
    let id = &id[..id.find('"').unwrap()];
    assert!(html.contains(&format!(r#"<span id="{id}""#)));
}

#[test]
fn a_bare_checkbox_takes_its_name_from_label() {
    let html = render(|| html! { <Checkbox label="Select row" id="row-1" class="mine" attrs={attrs! { "data-row" => "1" }} /> });
    assert!(input(&html).contains(r#"aria-label="Select row""#));
    assert!(input(&html).contains(r#"id="row-1""#));
    assert!(html.contains("mine"));
    assert!(html.contains(r#"data-row="1""#));
    assert!(!html.contains("yi-choice__text"));
}

#[test]
fn a_field_supplies_id_description_and_state() {
    reset_ids();
    let html = render(|| {
        html! {
            <Field label="Terms" error="Accept the terms to continue." required=true>
                <Checkbox>{ "I accept" }</Checkbox>
            </Field>
        }
    });
    let target = html.split(r#" for=""#).nth(1).expect("field label has `for`");
    let target = &target[..target.find('"').unwrap()];
    let input = input(&html);
    assert!(input.contains(&format!(r#"id="{target}""#)));
    assert!(input.contains(r#"aria-invalid="true""#));
    assert!(input.contains("aria-describedby"));
    assert!(input.contains(" required"));
}

#[test]
fn described_by_joins_only_the_ids_present() {
    let (a, b) = (AttrValue::from("a"), AttrValue::from("b"));
    assert_eq!(described_by([None, None]), None);
    assert_eq!(described_by([Some(&a), None]).as_deref(), Some("a"));
    assert_eq!(described_by([Some(&a), Some(&b)]).as_deref(), Some("a b"));
}
