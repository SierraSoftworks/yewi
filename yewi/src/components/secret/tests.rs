use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{Field, attrs};

const TOKEN: &str = "rtk_live_9Zp4mQ2vX7cL";

/// The opening tag of the first `<element` in `html`.
fn tag<'a>(html: &'a str, element: &str) -> &'a str {
    let start = html.find(&format!("<{element}")).unwrap_or_else(|| panic!("no <{element}> in {html}"));
    &html[start..start + html[start..].find('>').unwrap()]
}

/// The value of `name` in an opening tag.
fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let start = tag.find(&format!(r#" {name}=""#))? + name.len() + 3;
    Some(&tag[start..start + tag[start..].find('"')?])
}

#[test]
fn the_input_is_masked_until_revealed() {
    let cases = [(None, "password", "false"), (Some(false), "password", "false"), (Some(true), "text", "true")];
    for (revealed, kind, pressed) in cases {
        let html = render(move || html! { <SecretInput value={TOKEN} {revealed} /> });
        assert_eq!(attr(tag(&html, "input"), "type"), Some(kind), "revealed={revealed:?}");
        assert_eq!(attr(tag(&html, "button"), "aria-pressed"), Some(pressed), "revealed={revealed:?}");
    }
}

#[test]
fn the_input_keeps_browsers_from_helping_with_a_secret() {
    let html = render(|| html! { <SecretInput /> });
    let input = tag(&html, "input");
    assert_eq!(attr(input, "autocomplete"), Some("off"));
    assert_eq!(attr(input, "spellcheck"), Some("false"));

    let html = render(|| html! { <SecretInput autocomplete="current-password" /> });
    assert_eq!(attr(tag(&html, "input"), "autocomplete"), Some("current-password"));
}

#[test]
fn the_actions_are_named_buttons_that_do_not_submit_a_form() {
    let html = render(|| html! { <SecretInput generator=true /> });
    assert_eq!(html.matches("<button").count(), 2);
    assert_eq!(html.matches(r#"type="button""#).count(), 2);
    assert!(html.contains(r#"aria-label="Show secret""#));
    assert!(html.contains(r#"aria-label="Generate a new secret""#));
}

#[test]
fn the_actions_are_opt_in_or_out() {
    let plain = render(|| html! { <SecretInput /> });
    assert_eq!(plain.matches("<button").count(), 1);

    let write_only = render(|| html! { <SecretInput revealable=false /> });
    assert!(!write_only.contains("<button"));
}

#[test]
fn states_use_native_attributes() {
    let html = render(|| html! { <SecretInput disabled=true readonly=true required=true invalid=true generator=true /> });
    let input = tag(&html, "input");
    for expected in [" disabled", " readonly", " required", r#"aria-invalid="true""#] {
        assert!(input.contains(expected), "{expected} missing from {input}");
    }
    // The input and both of its buttons.
    assert_eq!(html.matches(" disabled").count(), 3);
}

#[test]
fn a_read_only_secret_can_be_revealed_but_not_regenerated() {
    let html = render(|| html! { <SecretInput value={TOKEN} readonly=true generator=true /> });
    let reveal = tag(&html, "button");
    assert!(!reveal.contains(" disabled"));
    let generate = &html[html.find(r#"aria-label="Generate"#).expect("generator")..];
    assert!(generate[..generate.find('>').unwrap()].contains(" disabled"));
}

#[test]
fn id_class_and_attrs_reach_the_control() {
    let html = render(|| html! { <SecretInput id="a" class="mine" attrs={attrs! { "aria-label" => "Token" }} /> });
    assert!(html.contains("mine"));
    let input = tag(&html, "input");
    assert_eq!(attr(input, "id"), Some("a"));
    assert_eq!(attr(input, "aria-label"), Some("Token"));
}

#[test]
fn the_input_joins_a_field() {
    let html = render(|| {
        html! {
            <Field label="Token" error={html! { "Too short" }} required=true>
                <SecretInput />
            </Field>
        }
    });
    let input = tag(&html, "input");
    let id = attr(input, "id").expect("input has an id");
    assert!(html.contains(&format!(r#"for="{id}""#)));
    assert!(attr(input, "aria-describedby").is_some());
    assert!(input.contains(r#"aria-invalid="true""#));
    assert!(input.contains(" required"));
}

#[test]
fn generated_secrets_are_unpadded_base64url() {
    let cases: [(&[u8], &str); 6] = [
        (b"", ""),
        (b"f", "Zg"),
        (b"fo", "Zm8"),
        (b"foo", "Zm9v"),
        (b"foobar", "Zm9vYmFy"),
        (&[0xfb, 0xff, 0xfe], "-__-"),
    ];
    for (bytes, expected) in cases {
        assert_eq!(encode_base64url(bytes), expected, "{bytes:?}");
    }
    assert_eq!(encode_base64url(&[0u8; 32]).len(), 43);
}

#[test]
fn a_value_is_labelled_and_copyable() {
    let html = render(|| html! { <SecretValue label="Token" value={TOKEN} /> });
    assert!(html.contains(TOKEN));
    assert!(html.contains(">Token<"));
    assert_eq!(html.matches("<button").count(), 1);
    let copy = tag(&html, "button");
    assert_eq!(attr(copy, "aria-label"), Some("Copy"));
    let described_by = attr(copy, "aria-describedby").expect("copy button has a description");
    assert!(html.contains(&format!(r#"id="{described_by}""#)));
}

#[test]
fn a_concealed_value_stays_out_of_the_markup() {
    let html = render(|| html! { <SecretValue label="Token" value={TOKEN} concealed=true /> });
    assert!(!html.contains(TOKEN));
    assert!(html.contains("Hidden"));
    assert!(html.contains(r#"aria-label="Show value""#));
    assert!(html.contains(r#"aria-pressed="false""#));
    assert!(html.contains(r#"aria-label="Copy""#));
}

#[test]
fn the_reveal_is_a_named_region_with_a_warning() {
    let html = render(|| {
        html! {
            <SecretReveal title="Token created" aside={html! { <p>{ "QR" }</p> }}>
                <SecretValue label="Token" value={TOKEN} />
            </SecretReveal>
        }
    });
    let labelled_by = attr(tag(&html, "section"), "aria-labelledby").expect("region is named");
    assert!(html.contains(&format!(r#"<h3 id="{labelled_by}""#)));
    assert!(html.contains("Token created"));
    assert!(html.contains("only time"));
    assert!(html.contains(TOKEN));
    assert!(html.contains("QR"));
    assert!(!html.contains("Done"));
}

#[test]
fn the_reveal_takes_a_custom_warning_level_and_dismiss_button() {
    let html = render(|| {
        html! {
            <SecretReveal
                title="Key rotated"
                level=2
                warning={html! { "The old key stops working in an hour." }}
                on_dismiss={Callback::noop()}
                dismiss_label="I have saved it"
            />
        }
    });
    assert!(html.contains("<h2"));
    assert!(html.contains("The old key stops working in an hour."));
    assert!(!html.contains("only time"));
    assert!(html.contains("I have saved it"));
}
