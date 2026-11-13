use yewi_test::render;

use super::{
    options::{edge, filter, step, typeahead},
    *,
};
use crate::{Field, attrs};

fn regions() -> SelectOptions {
    [
        SelectOption::new("eu-west-1", "Ireland").description("eu-west-1").group("Europe"),
        SelectOption::new("eu-central-1", "Frankfurt").group("Europe"),
        SelectOption::new("us-gov-1", "GovCloud").group("Americas").disabled(true),
        SelectOption::new("us-east-1", "Virginia").group("Americas"),
        SelectOption::new("sa-east-1", "Ireland Street").group("Americas"),
    ]
    .into()
}

#[test]
fn select_shows_the_chosen_label_as_a_collapsed_combobox() {
    let html = render(|| html! { <Select options={regions()} value="us-east-1" label="Region" /> });
    assert!(html.contains(r#"role="combobox""#));
    assert!(html.contains(r#"aria-expanded="false""#));
    assert!(html.contains(r#"aria-label="Region""#));
    assert!(html.contains("Virginia"));
    // The list exists only while open.
    assert!(!html.contains(r#"role="listbox""#));
    assert!(!html.contains("Frankfurt"));
}

#[test]
fn select_shows_the_placeholder_when_no_option_matches() {
    for value in [None, Some(AttrValue::from("nowhere"))] {
        let html = render(move || html! { <Select options={regions()} value={value.clone()} placeholder="Choose a region" /> });
        assert!(html.contains("Choose a region"));
    }
}

#[test]
fn combobox_shows_the_chosen_label_in_a_text_input() {
    let html = render(|| html! { <Combobox options={regions()} value="eu-west-1" placeholder="Search" /> });
    assert!(html.contains("<input"));
    assert!(html.contains(r#"role="combobox""#));
    assert!(html.contains(r#"aria-autocomplete="list""#));
    assert!(html.contains(r#"value="Ireland""#));
    assert!(html.contains(r#"placeholder="Search""#));
}

#[test]
fn combobox_shows_custom_text_only_when_allowed() {
    let custom = render(|| html! { <Combobox options={regions()} value="moon-1" allow_custom=true /> });
    assert!(custom.contains(r#"value="moon-1""#));
    let strict = render(|| html! { <Combobox options={regions()} value="moon-1" /> });
    assert!(!strict.contains("moon-1"));
}

#[test]
fn name_submits_the_value_in_a_native_form() {
    let select = render(|| html! { <Select options={regions()} value="us-east-1" name="region" /> });
    let combobox = render(|| html! { <Combobox options={regions()} value="us-east-1" name="region" /> });
    for html in [select, combobox] {
        assert!(html.contains(r#"type="hidden""#));
        assert!(html.contains(r#"name="region""#));
        assert!(html.contains(r#"value="us-east-1""#));
    }
}

#[test]
fn a_field_labels_describes_and_invalidates_the_control() {
    let select = render(|| {
        html! { <Field label="Region" error="Choose a region" required=true><Select options={regions()} /></Field> }
    });
    let combobox = render(|| {
        html! { <Field label="Region" error="Choose a region" required=true><Combobox options={regions()} /></Field> }
    });
    for html in [select, combobox] {
        assert!(html.contains(r#"aria-invalid="true""#));
        assert!(html.contains("aria-describedby="));
        let id_at = html.find("for=\"").expect("the label points at a control") + 5;
        let id = &html[id_at..id_at + html[id_at..].find('"').unwrap()];
        assert!(html.contains(&format!(r#"id="{id}""#)));
    }
}

#[test]
fn disabled_uses_the_native_attribute() {
    let select = render(|| html! { <Select options={regions()} disabled=true /> });
    let combobox = render(|| html! { <Combobox options={regions()} disabled=true /> });
    for html in [select, combobox] {
        assert!(html.contains(" disabled"));
    }
}

#[test]
fn class_and_attrs_reach_the_root_and_id_the_control() {
    let html = render(|| {
        html! { <Select options={regions()} id="region" class="mine" attrs={attrs! { "data-test" => "x" }} /> }
    });
    assert!(html.starts_with("<div"));
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-test="x""#));
    assert!(html.contains(r#"id="region""#));
}

#[test]
fn stepping_skips_disabled_options_and_stops_at_the_ends() {
    let options = regions();
    let order: Vec<usize> = (0..options.len()).collect();
    let cases = [
        (None, 1, Some(0)),
        (None, -1, Some(4)),
        (Some(1), 1, Some(3)),
        (Some(3), -1, Some(1)),
        (Some(4), 1, Some(4)),
        (Some(0), -1, Some(0)),
        (Some(0), 10, Some(4)),
        (Some(4), -10, Some(0)),
    ];
    for (from, delta, expected) in cases {
        assert_eq!(step(&options, &order, from, delta), expected, "from {from:?} by {delta}");
    }
    assert_eq!(step(&options, &[2], None, 1), None);
    assert_eq!(edge(&options, &[2, 3, 4], false), Some(3));
}

#[test]
fn typing_jumps_by_prefix_and_cycles_on_a_repeated_letter() {
    let options = regions();
    let cases = [
        ("f", None, Some(1)),
        ("V", Some(0), Some(3)),
        ("i", Some(0), Some(4)),
        ("ii", Some(4), Some(0)),
        ("ireland s", Some(0), Some(4)),
        ("ire", Some(0), Some(0)),
        // Disabled options are never landed on.
        ("g", None, None),
        ("x", Some(1), None),
    ];
    for (typed, from, expected) in cases {
        assert_eq!(typeahead(&options, typed, from), expected, "typed {typed:?} from {from:?}");
    }
}

#[test]
fn filtering_matches_every_term_and_caps_what_is_shown() {
    let options = regions();
    let cases = [
        ("", 10, vec![0, 1, 2, 3, 4], 5),
        ("IRE", 10, vec![0, 4], 2),
        ("street ireland", 10, vec![4], 1),
        ("eu-west", 10, vec![0], 1),
        ("zzz", 10, vec![], 0),
        ("", 2, vec![0, 1], 5),
    ];
    for (query, limit, shown, total) in cases {
        assert_eq!(filter(&options, query, limit), (shown, total), "query {query:?}");
    }
}

#[test]
fn the_cut_off_line_is_english_unless_more_label_is_given() {
    assert!(more_label(None, 50, 2000).contains("first 50 of 2000"));
    let custom = Callback::from(|(shown, total): (usize, usize)| AttrValue::from(format!("{shown} von {total}")));
    assert_eq!(more_label(Some(&custom), 50, 2000), "50 von 2000");
}
