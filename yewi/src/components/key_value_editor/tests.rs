use yewi_test::render;

use super::*;
use crate::{Field, hooks::reset_ids};

fn editor(rows: &[(&str, &str)], errors: Vec<(usize, AttrValue)>) -> String {
    reset_ids();
    let rows: Vec<KeyValue> = rows.iter().map(|row| KeyValue::from(*row)).collect();
    render(move || html! { <KeyValueEditor label="Labels" row_label="label" rows={rows.clone()} errors={errors.clone()} /> })
}

/// The opening tag of every `<input>` in `html`.
fn inputs(html: &str) -> Vec<&str> {
    html.split("<input").skip(1).map(|rest| &rest[..rest.find('>').unwrap()]).collect()
}

#[test]
fn each_row_is_a_named_key_and_value_with_a_way_to_remove_it() {
    let html = editor(&[("env", "prod"), ("tier", "web")], vec![]);
    assert!(html.contains(r#"aria-label="Labels""#));
    let inputs = inputs(&html);
    assert_eq!(inputs.len(), 4);
    for (input, (name, value)) in inputs.iter().zip([
        ("Key, label 1", "env"),
        ("Value, label 1", "prod"),
        ("Key, label 2", "tier"),
        ("Value, label 2", "web"),
    ]) {
        assert!(input.contains(&format!(r#"aria-label="{name}""#)), "{input}");
        assert!(input.contains(&format!(r#"value="{value}""#)), "{input}");
    }
    assert!(html.contains("Remove label 1") && html.contains("Remove label 2"));
}

#[test]
fn no_rows_leaves_only_the_add_button() {
    let html = editor(&[], vec![]);
    assert!(inputs(&html).is_empty());
    assert!(html.contains("Add row"));
}

#[test]
fn an_error_marks_and_describes_only_its_row() {
    let html = editor(&[("env", "prod"), ("", "web")], vec![(1, "A label needs a key.".into())]);
    assert!(html.contains("A label needs a key."));
    let invalid: Vec<bool> = inputs(&html).iter().map(|input| input.contains(r#"aria-invalid="true""#)).collect();
    assert_eq!(invalid, [false, false, true, true]);
    assert!(inputs(&html)[2].contains("aria-describedby="));
    assert!(!inputs(&html)[0].contains("aria-describedby="));
}

#[test]
fn the_add_button_stops_at_max_rows() {
    let add_disabled = |max_rows: Option<usize>| {
        let html = render(move || html! { <KeyValueEditor rows={vec![KeyValue::new("a", "b")]} {max_rows} /> });
        let button = &html[html.rfind("<button").unwrap()..];
        button[..button.find('>').unwrap()].contains(" disabled")
    };
    assert!(!add_disabled(None));
    assert!(!add_disabled(Some(2)));
    assert!(add_disabled(Some(1)));
}

#[test]
fn inside_a_field_the_inputs_keep_their_own_names_and_ids() {
    reset_ids();
    let html = render(|| {
        html! {
            <Field label="Labels" hint="Decide the rollout phase.">
                <KeyValueEditor rows={vec![KeyValue::new("env", "prod")]} />
            </Field>
        }
    });
    assert!(html.contains("aria-labelledby="));
    assert!(inputs(&html).iter().all(|input| !input.contains(" id=")), "{html}");
}

#[test]
fn blank_rows_are_recognisable() {
    for (row, blank) in [(("", ""), true), ((" ", "\t"), true), (("env", ""), false), (("", "prod"), false)] {
        assert_eq!(KeyValue::from(row).is_blank(), blank, "{row:?}");
    }
}
