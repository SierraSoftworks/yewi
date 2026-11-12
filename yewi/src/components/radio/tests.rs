use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{Field, hooks::reset_ids};

/// The opening tag of every `<input>` in `html`.
fn inputs(html: &str) -> Vec<&str> {
    html.split("<input").skip(1).map(|rest| &rest[..rest.find('>').unwrap()]).collect()
}

fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let value = &tag[tag.find(&format!(" {name}=\""))? + name.len() + 3..];
    Some(&value[..value.find('"')?])
}

fn group(value: Option<&'static str>) -> String {
    reset_ids();
    render(move || {
        html! {
            <RadioGroup label="Environment" value={value.map(AttrValue::from)}>
                <Radio value="staging">{ "Staging" }</Radio>
                <Radio value="production" description="Needs approval.">{ "Production" }</Radio>
                <Radio value="retired" disabled=true>{ "Retired" }</Radio>
            </RadioGroup>
        }
    })
}

#[test]
fn a_group_is_a_named_radiogroup_of_native_radios() {
    let html = group(None);
    assert!(html.contains(r#"role="radiogroup""#));
    assert!(html.contains(r#"aria-label="Environment""#));
    let inputs = inputs(&html);
    assert_eq!(inputs.len(), 3);
    assert!(inputs.iter().all(|input| input.contains(r#"type="radio""#)));
}

#[test]
fn options_share_one_name_so_the_browser_treats_them_as_a_set() {
    let html = group(None);
    let names: Vec<_> = inputs(&html).into_iter().map(|input| attr(input, "name").expect("has a name")).collect();
    assert!(names.windows(2).all(|pair| pair[0] == pair[1]));
}

#[test]
fn only_the_option_matching_the_value_is_checked() {
    for (value, expected) in [(None, None), (Some("production"), Some("production")), (Some("unknown"), None)] {
        let html = group(value);
        let checked: Vec<_> =
            inputs(&html).into_iter().filter(|input| input.contains(" checked")).map(|i| attr(i, "value")).collect();
        assert_eq!(checked, expected.map(Some).into_iter().collect::<Vec<_>>(), "value {value:?}");
    }
}

#[test]
fn disabling_reaches_one_option_or_all_of_them() {
    let disabled = |html: &str| inputs(html).into_iter().filter(|input| input.contains(" disabled")).count();
    assert_eq!(disabled(&group(None)), 1);

    let html = render(|| {
        html! {
            <RadioGroup label="Environment" disabled=true>
                <Radio value="a">{ "A" }</Radio>
                <Radio value="b">{ "B" }</Radio>
            </RadioGroup>
        }
    });
    assert_eq!(disabled(&html), 2);
}

#[test]
fn a_field_names_and_describes_the_group() {
    reset_ids();
    let html = render(|| {
        html! {
            <Field label="Environment" error="Choose one." required=true group=true>
                <RadioGroup>
                    <Radio value="a">{ "A" }</Radio>
                </RadioGroup>
            </Field>
        }
    });
    let group = &html[html.find(r#"<div"#).unwrap()..];
    let group = &group[group.find(r#"role="radiogroup""#).expect("has a radiogroup") - 200..];
    let tag = &group[group.rfind("<div").unwrap()..];
    let tag = &tag[..tag.find('>').unwrap()];
    assert!(attr(tag, "aria-labelledby").is_some_and(|id| html.contains(&format!(r#"<span id="{id}""#))));
    assert!(attr(tag, "aria-describedby").is_some());
    assert_eq!(attr(tag, "aria-invalid"), Some("true"));
    assert_eq!(attr(tag, "aria-required"), Some("true"));
    assert!(inputs(&html)[0].contains(" required"));
}

#[test]
fn a_radio_works_without_a_group() {
    let html = render(|| html! { <Radio value="row-2" name="row" checked=true label="Select row 2" /> });
    let input = inputs(&html)[0];
    assert!(input.contains(" checked"));
    assert_eq!(attr(input, "name"), Some("row"));
    assert_eq!(attr(input, "aria-label"), Some("Select row 2"));
}
