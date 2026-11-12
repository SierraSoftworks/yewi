use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{Field, FieldLayout, hooks::reset_ids};

fn input(html: &str) -> &str {
    let start = html.find("<input").expect("renders an input");
    &html[start..start + html[start..].find('>').unwrap()]
}

#[test]
fn is_a_native_checkbox_exposed_as_a_switch() {
    let html = render(|| html! { <ToggleSwitch name="alerts">{ "Email alerts" }</ToggleSwitch> });
    assert!(html.starts_with("<label"));
    assert!(html.contains("Email alerts"));
    let input = input(&html);
    assert!(input.contains(r#"type="checkbox""#));
    assert!(input.contains(r#"role="switch""#));
    assert!(input.contains(r#"name="alerts""#));
}

#[test]
fn reflects_its_state() {
    for (checked, disabled) in [(false, false), (true, false), (true, true)] {
        let html = render(move || html! { <ToggleSwitch {checked} {disabled} label="Alerts" /> });
        let input = input(&html);
        assert_eq!(input.contains(" checked"), checked);
        assert_eq!(input.contains(" disabled"), disabled);
        assert!(input.contains(r#"aria-label="Alerts""#));
    }
}

#[test]
fn a_field_labels_and_describes_it() {
    reset_ids();
    let html = render(|| {
        html! {
            <Field label="Maintenance mode" hint="Pauses all deploys." layout={FieldLayout::Inline}>
                <ToggleSwitch description="Visible to everyone." />
            </Field>
        }
    });
    let target = html.split(r#" for=""#).nth(1).expect("field label has `for`");
    let target = &target[..target.find('"').unwrap()];
    let input = input(&html);
    assert!(input.contains(&format!(r#"id="{target}""#)));
    assert!(input.contains("aria-describedby"));
    assert!(html.contains("Visible to everyone."));
}
