use std::rc::Rc;

use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{Button, Field, MenuButton, MenuItem, Select, SelectOption, SelectOptions, TextInput, attrs, hooks::reset_ids};

/// The opening tag of the first `<tag ...>` in `html`.
fn tag<'a>(html: &'a str, tag: &str) -> &'a str {
    let start = html.find(&format!("<{tag} ")).unwrap_or_else(|| panic!("no <{tag}> in {html}"));
    &html[start..start + html[start..].find('>').unwrap()]
}

fn protocols() -> Rc<[(AttrValue, AttrValue)]> {
    Rc::from([("https".into(), "https://".into()), ("http".into(), "http://".into())])
}

#[test]
fn affixes_surround_the_control_in_reading_order() {
    let html = render(|| {
        html! {
            <InputGroup prefix="https://" suffix={html! { <Button>{ "Check" }</Button> }}>
                <TextInput value="example.com" />
            </InputGroup>
        }
    });
    let position = |needle: &str| html.find(needle).unwrap_or_else(|| panic!("{needle} missing from {html}"));
    assert!(position("https://") < position("<input"));
    assert!(position("<input") < position("Check"));
    assert!(html.contains("<button"));
}

#[test]
fn a_group_without_affixes_renders_only_the_control() {
    let html = render(|| html! { <InputGroup><TextInput /></InputGroup> });
    assert_eq!(html.matches("<span").count(), 0);
    assert_eq!(html.matches("<input").count(), 1);
}

#[test]
fn id_class_and_attrs_reach_the_group() {
    let html = render(|| {
        html! { <InputGroup id="g" class="mine" attrs={attrs! { "data-row" => "1" }}><TextInput /></InputGroup> }
    });
    let root = tag(&html, "div");
    for expected in [r#"id="g""#, "mine", r#"data-row="1""#] {
        assert!(root.contains(expected), "{expected} missing from {root}");
    }
}

#[test]
fn the_control_keeps_its_field_wiring_inside_a_group() {
    reset_ids();
    let html = render(|| {
        html! {
            <Field label="Domain" error="Taken">
                <InputGroup prefix="https://"><TextInput /></InputGroup>
            </Field>
        }
    });
    let input = tag(&html, "input");
    let label = tag(&html, "label");
    let id = &input[input.find(" id=\"").expect("input has an id") + 5..];
    assert!(label.contains(&format!(r#"for="{}""#, &id[..id.find('"').unwrap()])));
    assert!(input.contains(r#"aria-invalid="true""#));
}

#[test]
fn select_marks_the_current_option_and_is_named() {
    let html = render(|| html! { <InputGroupSelect label="Protocol" options={protocols()} value="http" /> });
    assert!(tag(&html, "select").contains(r#"aria-label="Protocol""#));
    assert_eq!(html.matches("<option").count(), 2);
    assert_eq!(html.matches(" selected").count(), 1);
    let selected = &html[..html.find(" selected").unwrap()];
    assert!(selected.rfind("<option").is_some_and(|start| selected[start..].contains(r#"value="http""#)));
}

#[test]
fn select_states_use_native_attributes() {
    let html = render(|| {
        html! { <InputGroupSelect label="Unit" options={protocols()} name="unit" disabled=true id="u" /> }
    });
    let select = tag(&html, "select");
    for expected in [" disabled", r#"name="unit""#, r#"id="u""#] {
        assert!(select.contains(expected), "{expected} missing from {select}");
    }
}

#[test]
fn search_input_is_a_named_native_search_box() {
    let html = render(|| html! { <SearchInput value="api" placeholder="Search services" name="q" /> });
    let input = tag(&html, "input");
    for expected in [
        r#"type="search""#,
        r#"value="api""#,
        r#"placeholder="Search services""#,
        r#"name="q""#,
        r#"aria-label="Search""#,
    ] {
        assert!(input.contains(expected), "{expected} missing from {input}");
    }
    assert!(tag(&html, "button").contains(r#"aria-label="Clear search""#));
}

#[test]
fn search_input_labels_are_configurable() {
    let html = render(|| html! { <SearchInput label="Search hosts" clear_label="Reset" /> });
    assert!(tag(&html, "input").contains(r#"aria-label="Search hosts""#));
    assert!(tag(&html, "button").contains(r#"aria-label="Reset""#));
}

#[test]
fn a_field_names_the_search_input() {
    reset_ids();
    let html = render(|| html! { <Field label="Find a service"><SearchInput /></Field> });
    let input = tag(&html, "input");
    assert!(!input.contains("aria-label"), "{input}");
    assert!(input.contains(" id=\""));
}

#[test]
fn search_input_states_reach_the_markup() {
    let html = render(|| {
        html! { <SearchInput loading=true disabled=true id="s" attrs={attrs! { "data-scope" => "all" }} /> }
    });
    assert!(tag(&html, "div").contains(r#"aria-busy="true""#));
    let input = tag(&html, "input");
    for expected in [" disabled", r#"id="s""#, r#"data-scope="all""#] {
        assert!(input.contains(expected), "{expected} missing from {input}");
    }

    let idle = render(|| html! { <SearchInput /> });
    assert!(!idle.contains("aria-busy"));
}

#[test]
fn search_input_suffix_follows_the_clear_button() {
    let html = render(|| html! { <SearchInput suffix={html! { <Button>{ "Go" }</Button> }} /> });
    assert!(html.find("Clear search").unwrap() < html.find("Go").unwrap());
}

#[test]
fn select_and_menu_button_render_inside_affixes() {
    reset_ids();
    let html = render(|| {
        let options = SelectOptions::from([SelectOption::new("https", "https://")]);
        let menu = html! { <MenuButton label="Insert"><MenuItem>{ "Version" }</MenuItem></MenuButton> };
        html! {
            <InputGroup prefix={html! { <Select label="Protocol" {options} value="https" /> }} suffix={menu}>
                <TextInput />
            </InputGroup>
        }
    });
    let position = |needle: &str| html.find(needle).unwrap_or_else(|| panic!("{needle} missing from {html}"));
    assert!(position(r#"role="combobox""#) < position("<input"));
    assert!(position("<input") < position("Insert"));
}

#[test]
fn a_control_in_an_affix_does_not_take_the_field_wiring() {
    reset_ids();
    let html = render(|| {
        let options = SelectOptions::from([SelectOption::new("https", "https://")]);
        html! {
            <Field label="Webhook URL" error="Unreachable">
                <InputGroup prefix={html! { <Select label="Protocol" {options} value="https" /> }}>
                    <TextInput />
                </InputGroup>
            </Field>
        }
    });
    let label = tag(&html, "label");
    let target = &label[label.find(" for=\"").expect("label has `for`") + 6..];
    let target = &target[..target.find('"').unwrap()];
    assert_eq!(html.matches(&format!(r#" id="{target}""#)).count(), 1, "{html}");
    assert!(tag(&html, "input").contains(&format!(r#" id="{target}""#)));
    assert!(!tag(&html, "button").contains("aria-invalid"));
}
