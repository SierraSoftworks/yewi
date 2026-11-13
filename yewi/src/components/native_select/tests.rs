use std::rc::Rc;

use yewi_test::render;

use super::*;
use crate::{Field, InputGroup, TextInput, hooks::reset_ids};

/// The opening tag of the first `<tag ...>` in `html`.
fn tag<'a>(html: &'a str, tag: &str) -> &'a str {
    let start = html.find(&format!("<{tag} ")).unwrap_or_else(|| panic!("no <{tag}> in {html}"));
    &html[start..start + html[start..].find('>').unwrap()]
}

fn protocols() -> Rc<[(AttrValue, AttrValue)]> {
    Rc::from([("https".into(), "https://".into()), ("http".into(), "http://".into())])
}

/// The opening tag of the option the server render marks as chosen.
fn selected_option(html: &str) -> &str {
    assert_eq!(html.matches(" selected").count(), 1, "one option is selected in {html}");
    let before = &html[..html.find(" selected").unwrap()];
    &before[before.rfind("<option").expect("selected sits on an option")..]
}

#[test]
fn the_server_render_marks_the_current_option_and_is_named() {
    let html = render(|| html! { <NativeSelect label="Protocol" options={protocols()} value="http" /> });
    assert!(tag(&html, "select").contains(r#"aria-label="Protocol""#));
    assert_eq!(html.matches("<option").count(), 2);
    assert!(selected_option(&html).contains(r#"value="http""#));
}

#[test]
fn states_and_form_wiring_use_native_attributes() {
    let html = render(|| {
        html! { <NativeSelect options={protocols()} name="unit" disabled=true required=true invalid=true id="u" /> }
    });
    let select = tag(&html, "select");
    for expected in [" disabled", " required", r#"name="unit""#, r#"id="u""#, r#"aria-invalid="true""#] {
        assert!(select.contains(expected), "{expected} missing from {select}");
    }
}

#[test]
fn a_placeholder_stands_in_until_a_value_matches() {
    let empty = render(|| html! { <NativeSelect options={protocols()} placeholder="Choose" /> });
    let placeholder = selected_option(&empty);
    assert!(placeholder.contains(r#"value="""#) && placeholder.contains(" disabled"), "{placeholder}");

    let chosen = render(|| html! { <NativeSelect options={protocols()} placeholder="Choose" value="https" /> });
    assert!(selected_option(&chosen).contains(r#"value="https""#));
}

#[test]
fn a_clearable_placeholder_is_a_choice() {
    for (clearable, value) in [(true, ""), (true, "https"), (false, "https")] {
        let html = render(move || {
            html! { <NativeSelect options={protocols()} placeholder="Any" {clearable} {value} /> }
        });
        let placeholder = tag(&html, "option");
        assert!(placeholder.contains(r#"value="""#), "{placeholder}");
        assert_eq!(placeholder.contains(" disabled"), !clearable, "{placeholder}");
        assert!(selected_option(&html).contains(&format!(r#"value="{value}""#)), "{html}");
    }
}

#[test]
fn a_value_outside_the_options_is_kept_as_the_choice() {
    let select = |unknown_label: Option<Callback<AttrValue, AttrValue>>| {
        render(move || {
            let unknown_label = unknown_label.clone();
            html! { <NativeSelect options={protocols()} placeholder="Choose" value="ftp" {unknown_label} /> }
        })
    };
    let plain = select(None);
    assert!(selected_option(&plain).contains(r#"value="ftp""#), "{plain}");
    assert_eq!(plain.matches("<option").count(), 4);

    let labelled = select(Some(Callback::from(|value: AttrValue| format!("{value} (retired)").into())));
    assert!(labelled.contains("ftp (retired)"), "{labelled}");
}

#[test]
fn a_field_labels_and_describes_it() {
    reset_ids();
    let html = render(|| {
        html! {
            <Field label="Protocol" error="Not allowed here">
                <NativeSelect options={protocols()} value="http" />
            </Field>
        }
    });
    let select = tag(&html, "select");
    let id = select.split(r#"id=""#).nth(1).and_then(|rest| rest.split('"').next()).expect("the select has an id");
    assert!(html.contains(&format!(r#"for="{id}""#)), "the label points at the select in {html}");
    for expected in [r#"aria-invalid="true""#, "aria-describedby="] {
        assert!(select.contains(expected), "{expected} missing from {select}");
    }
}

#[test]
fn in_a_group_affix_it_leaves_the_field_wiring_to_the_text_control() {
    reset_ids();
    let html = render(|| {
        let unit = html! { <NativeSelect label="Unit" options={protocols()} /> };
        html! {
            <Field label="Address" error="Taken">
                <InputGroup prefix={unit}><TextInput /></InputGroup>
            </Field>
        }
    });
    let select = tag(&html, "select");
    for unexpected in [" id=", "aria-invalid", "aria-describedby"] {
        assert!(!select.contains(unexpected), "{unexpected} found in {select}");
    }
    assert!(tag(&html, "input").contains(r#"aria-invalid="true""#));
}
