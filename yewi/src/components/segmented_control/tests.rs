use yewi_test::render;

use super::*;
use crate::{hooks::reset_ids, icons};

fn inputs(html: &str) -> Vec<&str> {
    html.split("<input").skip(1).map(|rest| &rest[..rest.find('>').unwrap()]).collect()
}

fn control(value: &'static str, disabled: bool) -> String {
    reset_ids();
    render(move || {
        html! {
            <SegmentedControl label="Files shown" {value} {disabled}>
                <Segment value="changed">{ "Changed" }</Segment>
                <Segment value="all">{ "All" }</Segment>
                <Segment value="tree" icon={icons::CHECK} label="Tree" disabled=true />
            </SegmentedControl>
        }
    })
}

#[test]
fn is_a_named_radiogroup_of_native_radios_sharing_a_name() {
    let html = control("changed", false);
    assert!(html.contains(r#"role="radiogroup""#));
    assert!(html.contains(r#"aria-label="Files shown""#));
    let inputs = inputs(&html);
    assert_eq!(inputs.len(), 3);
    let name = |input: &str| input.split(r#" name=""#).nth(1).map(|rest| rest[..rest.find('"').unwrap()].to_owned());
    assert!(inputs.iter().all(|input| input.contains(r#"type="radio""#) && name(input) == name(inputs[0])));
    assert!(name(inputs[0]).is_some());
}

#[test]
fn exactly_the_selected_segment_is_checked() {
    for value in ["changed", "all"] {
        let html = control(value, false);
        let checked: Vec<_> = inputs(&html).into_iter().filter(|input| input.contains(" checked")).collect();
        assert_eq!(checked.len(), 1);
        assert!(checked[0].contains(&format!(r#"value="{value}""#)));
    }
}

#[test]
fn disabling_reaches_one_segment_or_all_of_them() {
    let disabled = |html: &str| inputs(html).into_iter().filter(|input| input.contains(" disabled")).count();
    assert_eq!(disabled(&control("all", false)), 1);
    assert_eq!(disabled(&control("all", true)), 3);
}

#[test]
fn an_icon_only_segment_has_an_accessible_name() {
    let html = control("all", false);
    assert!(inputs(&html)[2].contains(r#"aria-label="Tree""#));
    assert!(html.contains("<svg"));
}
