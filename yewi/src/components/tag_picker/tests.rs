use yew::prelude::*;
use yewi_test::render;

use super::{
    values::{add, parse, toggle},
    *,
};
use crate::{Field, SelectOption, attrs};

fn labels() -> SelectOptions {
    [
        SelectOption::new("bug", "Bug"),
        SelectOption::new("docs", "Documentation"),
        SelectOption::new("security", "Security").disabled(true),
    ]
    .into()
}

fn held(values: &[&'static str]) -> Vec<AttrValue> {
    values.iter().map(|value| AttrValue::from(*value)).collect()
}

#[test]
fn shows_each_value_as_a_removable_tag_named_by_its_option() {
    let html = render(|| html! { <TagPicker options={labels()} values={held(&["docs", "wontfix"])} label="Labels" /> });
    assert!(html.contains(r#"role="combobox""#));
    assert!(html.contains(r#"aria-label="Labels""#));
    assert!(html.contains(r#"role="list""#));
    // An option shows its label; a value no option has shows as it is.
    assert!(html.contains(r#"aria-label="Remove Documentation""#));
    assert!(html.contains(r#"aria-label="Remove wontfix""#));
    // The list of options exists only while open.
    assert!(!html.contains(r#"role="listbox""#));
    assert!(!html.contains("Bug"));
}

#[test]
fn the_placeholder_shows_only_while_nothing_is_chosen() {
    let empty = render(|| html! { <TagPicker options={labels()} placeholder="Add labels" /> });
    assert!(empty.contains(r#"placeholder="Add labels""#));
    assert!(!empty.contains(r#"role="list""#));
    let chosen = render(|| html! { <TagPicker options={labels()} values={held(&["bug"])} placeholder="Add labels" /> });
    assert!(!chosen.contains("Add labels"));
}

#[test]
fn name_submits_every_value_in_a_native_form() {
    let html = render(|| html! { <TagPicker options={labels()} values={held(&["bug", "docs"])} name="label" /> });
    assert_eq!(html.matches(r#"type="hidden""#).count(), 2);
    assert_eq!(html.matches(r#"name="label""#).count(), 2);
    for value in ["bug", "docs"] {
        assert!(html.contains(&format!(r#"value="{value}""#)));
    }
}

#[test]
fn a_field_labels_describes_and_invalidates_the_control() {
    let html = render(|| {
        html! { <Field label="Labels" error="Choose a label" required=true><TagPicker options={labels()} /></Field> }
    });
    assert!(html.contains(r#"aria-invalid="true""#));
    assert!(html.contains(r#"aria-required="true""#));
    assert!(html.contains("aria-describedby="));
    let id_at = html.find("for=\"").expect("the label points at a control") + 5;
    let id = &html[id_at..id_at + html[id_at..].find('"').unwrap()];
    assert!(html.contains(&format!(r#"id="{id}""#)));
}

#[test]
fn disabled_reaches_the_input_and_every_remove_button() {
    let html = render(|| html! { <TagPicker options={labels()} values={held(&["bug", "docs"])} disabled=true /> });
    assert_eq!(html.matches(" disabled").count(), 3);
}

#[test]
fn class_and_attrs_reach_the_root_and_id_the_input() {
    let html = render(|| {
        html! { <TagPicker options={labels()} id="labels" class="mine" attrs={attrs! { "data-test" => "x" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-test="x""#));
    assert!(html.contains(r#"id="labels""#));
}

#[test]
fn toggling_adds_at_the_end_and_removes_in_place() {
    let values = held(&["bug", "docs"]);
    assert_eq!(toggle(&values, &"new".into()), held(&["bug", "docs", "new"]));
    assert_eq!(toggle(&values, &"bug".into()), held(&["docs"]));
}

#[test]
fn typed_text_is_split_trimmed_and_matched_to_options() {
    assert_eq!(parse(" a, b ,,\nc ").collect::<Vec<_>>(), ["a", "b", "c"]);

    let (options, values) = (labels(), held(&["bug"]));
    let cases = [
        // New text is added as typed.
        ("wontfix", Some(held(&["bug", "wontfix"]))),
        // An option's label, in any case, adds its value.
        ("documentation", Some(held(&["bug", "docs"]))),
        // What is already there is not added twice, by value or by label.
        ("bug", None),
        ("BUG", None),
        // A disabled option is not reachable by its label.
        ("Security", Some(held(&["bug", "Security"]))),
    ];
    for (typed, expected) in cases {
        assert_eq!(add(&values, &options, typed), expected, "typed {typed:?}");
    }
}
