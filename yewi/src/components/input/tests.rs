use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn text_input_renders_a_native_input_with_its_value() {
    let html = render(|| html! { <TextInput value="hello" placeholder="Name" name="name" /> });
    assert!(html.starts_with("<input"));
    assert!(html.contains(r#"value="hello""#));
    assert!(html.contains(r#"placeholder="Name""#));
    assert!(html.contains(r#"name="name""#));
}

#[test]
fn every_kind_sets_a_distinct_native_type() {
    let mut types: Vec<String> = TextInputKind::ALL
        .into_iter()
        .map(|kind| {
            let html = render(move || html! { <TextInput {kind} /> });
            let start = html.find("type=\"").expect("has a type") + 6;
            html[start..start + html[start..].find('"').unwrap()].to_owned()
        })
        .collect();
    types.sort();
    types.dedup();
    assert_eq!(types.len(), TextInputKind::ALL.len());
}

#[test]
fn text_area_renders_its_value_and_rows() {
    let html = render(|| html! { <TextArea value="line one" rows=5 /> });
    assert!(html.starts_with("<textarea"));
    assert!(html.contains("line one"));
    assert!(html.contains(r#"rows="5""#));
}

#[test]
fn states_use_native_attributes() {
    let input = render(|| html! { <TextInput disabled=true readonly=true required=true invalid=true /> });
    let area = render(|| html! { <TextArea disabled=true readonly=true required=true invalid=true /> });
    let number = render(|| html! { <NumberInput disabled=true readonly=true required=true invalid=true /> });
    for html in [input, area, number] {
        for expected in [" disabled", " readonly", " required", r#"aria-invalid="true""#] {
            assert!(html.contains(expected), "{expected} missing from {html}");
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_control() {
    let input = render(|| html! { <TextInput id="a" class="mine" attrs={attrs! { "aria-label" => "Name" }} /> });
    let area = render(|| html! { <TextArea id="a" class="mine" attrs={attrs! { "aria-label" => "Name" }} /> });
    let number = render(|| html! { <NumberInput id="a" class="mine" attrs={attrs! { "aria-label" => "Name" }} /> });
    for html in [input, area, number] {
        assert!(html.contains("mine"));
        let control = &html[html.find("<input").or(html.find("<textarea")).unwrap()..];
        let control = &control[..control.find('>').unwrap()];
        assert!(control.contains(r#"id="a""#), "{control}");
        assert!(control.contains(r#"aria-label="Name""#), "{control}");
    }
}

#[test]
fn number_input_is_a_spinbutton_with_its_range() {
    let html = render(|| html! { <NumberInput value={2.5} min={0.0} max={10.0} /> });
    assert!(html.contains(r#"role="spinbutton""#));
    assert!(html.contains(r#"value="2.5""#));
    assert!(html.contains(r#"aria-valuenow="2.5""#));
    assert!(html.contains(r#"aria-valuemin="0""#));
    assert!(html.contains(r#"aria-valuemax="10""#));
}

#[test]
fn an_empty_number_input_has_no_value() {
    let html = render(|| html! { <NumberInput /> });
    assert!(!html.contains("aria-valuenow"));
    assert!(!html.contains(r#"value="0""#));
}

#[test]
fn the_stepper_can_be_turned_off_and_stops_at_the_limits() {
    let plain = render(|| html! { <NumberInput value={1.0} stepper=false /> });
    assert!(!plain.contains("<button"));

    let html = render(|| html! { <NumberInput value={1.0} min={1.0} max={5.0} /> });
    let decrease = &html[html.find(r#"aria-label="Decrease""#).expect("decrease button")..];
    let (decrease, increase) = decrease.split_at(decrease.find(r#"aria-label="Increase""#).expect("increase button"));
    assert!(decrease.contains(" disabled"));
    assert!(!increase[..increase.find('>').unwrap()].contains(" disabled"));
}

#[test]
fn parse_reads_what_a_user_types() {
    let cases = [
        ("", Some(None)),
        ("  ", Some(None)),
        ("42", Some(Some(42.0))),
        (" -3.5 ", Some(Some(-3.5))),
        ("1.", Some(Some(1.0))),
        ("0,25", Some(Some(0.25))),
        ("1e3", Some(Some(1000.0))),
        ("-", None),
        ("1e", None),
        ("abc", None),
        ("inf", None),
        ("NaN", None),
        ("1.2.3", None),
    ];
    for (text, expected) in cases {
        assert_eq!(parse(text), expected, "parse({text:?})");
    }
}

#[test]
fn stepping_respects_bounds_and_decimal_precision() {
    // (current, steps, step, min, max) => expected
    let cases = [
        ((Some(3.0), 1.0, 1.0, None, None), 4.0),
        ((Some(3.0), -10.0, 1.0, Some(0.0), None), 0.0),
        ((Some(4.0), 1.0, 1.0, None, Some(4.0)), 4.0),
        ((Some(0.1), 1.0, 0.2, None, None), 0.3),
        ((Some(0.25), 1.0, 1.0, None, None), 1.25),
        ((Some(1.0), -1.0, 0.05, Some(0.0), Some(1.0)), 0.95),
        ((None, 1.0, 1.0, None, None), 1.0),
        ((None, -1.0, 1.0, Some(10.0), None), 10.0),
    ];
    for ((current, steps, step, min, max), expected) in cases {
        assert_eq!(stepped(current, steps, step, min, max), expected, "{current:?} {steps:+} x {step}");
    }
}

#[test]
fn numbers_format_without_noise() {
    for (value, expected) in [(3.0, "3"), (-0.0, "0"), (0.5, "0.5"), (1e3, "1000"), (-12.25, "-12.25")] {
        assert_eq!(format(value), expected);
    }
}
