use yewi_test::{render, render_hydratable};

use super::*;
use crate::attrs;

const DOC: &str = r#"{
  "service": "caddy",
  "replicas": 3,
  "healthy": true,
  "owner": null,
  "ports": [80, 443],
  "meta": {"region": "eu-west-1", "tags": {"tier": "edge"}},
  "nodes": [{"id": "fra-01"}, {"id": "fra-02"}]
}"#;

fn doc() -> Json {
    Json::parse(DOC).unwrap()
}

/// How many times `needle` occurs in `html`.
fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

// ---- Json ------------------------------------------------------------------

#[test]
fn parses_every_kind_of_value() {
    for (text, expected) in [
        ("null", Json::Null),
        (" true ", Json::Bool(true)),
        ("false", Json::Bool(false)),
        ("-12.50e+3", Json::Number("-12.50e+3".into())),
        ("18446744073709551616", Json::Number("18446744073709551616".into())),
        (r#""a\"b\\c\n\u00e9\ud83d\ude00""#, Json::from("a\"b\\c\né😀")),
        ("[]", Json::Array([].into())),
        ("{}", Json::Object([].into())),
        ("[1, [true]]", Json::Array([Json::Number("1".into()), Json::Array([Json::Bool(true)].into())].into())),
    ] {
        assert_eq!(Json::parse(text), Ok(expected), "{text}");
    }
}

#[test]
fn objects_keep_document_order() {
    let Json::Object(entries) = Json::parse(r#"{"z": 1, "a": 2, "m": 3}"#).unwrap() else { panic!("not an object") };
    let keys: Vec<&str> = entries.iter().map(|(key, _)| key.as_str()).collect();
    assert_eq!(keys, ["z", "a", "m"]);
}

#[test]
fn rejects_malformed_documents_with_a_position() {
    for text in ["", "{", "[1,]", "{\"a\" 1}", "{a: 1}", "01", "1.", "\"open", "\"\\x\"", "nul", "1 2", "\"a\nb\""] {
        let error = Json::parse(text).expect_err(text);
        assert!(error.offset <= text.len(), "{text}: {error}");
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn refuses_runaway_nesting_instead_of_overflowing_the_stack() {
    let deep = "[".repeat(100_000);
    assert!(Json::parse(&deep).is_err());
}

#[test]
fn pretty_and_compact_output_round_trip() {
    let value = doc();
    assert_eq!(Json::parse(&value.to_pretty()), Ok(value.clone()));
    assert_eq!(Json::parse(&value.to_string()), Ok(value.clone()));
    assert!(value.to_pretty().contains("\n  \"service\": \"caddy\","));
    assert!(!value.to_string().contains(char::is_whitespace));
}

#[test]
fn strings_are_escaped_on_output() {
    let value = Json::from("tab\there \"quoted\" \u{1}");
    assert_eq!(value.to_string(), r#""tab\there \"quoted\" \u0001""#);
    assert_eq!(Json::parse(&value.to_string()), Ok(value));
}

#[cfg(feature = "serde_json")]
#[test]
fn converts_from_serde_json() {
    let value: serde_json::Value = serde_json::from_str(DOC).unwrap();
    let converted = Json::from(&value);
    // serde_json may order keys differently; compare through its own parser.
    let back: serde_json::Value = serde_json::from_str(&converted.to_pretty()).unwrap();
    assert_eq!(back, value);
}

// ---- JsonView --------------------------------------------------------------

#[test]
fn the_tree_shows_first_level_values_and_folds_the_rest() {
    let html = render(|| html! { <JsonView value={doc()} /> });
    for shown in ["service", "\"caddy\"", "replicas", "healthy", "true", "null", "443", "meta", "nodes"] {
        assert!(html.contains(shown), "missing {shown}");
    }
    // Nested containers are a summary line with a hint of what they hold;
    // anything deeper is not rendered at all.
    assert!(html.contains("region: \"eu-west-1\", tags: {…}"));
    for folded in ["edge", "fra-02\"}", "tier"] {
        assert!(!html.contains(folded), "{folded} should be folded away");
    }
    assert_eq!(count(&html, "<details"), 3);
    assert_eq!(count(&html, "<details open"), 1);
}

#[test]
fn open_sets_how_many_levels_start_expanded() {
    for (open, expanded, visible, hidden) in
        [(0, 0, Some("caddy"), Some("eu-west-1")), (2, 3, Some("tier"), None), (9, 6, Some("edge"), None)]
    {
        let html = render(move || html! { <JsonView value={doc()} {open} /> });
        assert_eq!(count(&html, "<details open"), expanded, "open={open}");
        if let Some(text) = visible {
            assert!(html.contains(text), "open={open} should show {text}");
        }
        if let Some(text) = hidden {
            assert!(!html.contains(text), "open={open} should hide {text}");
        }
    }
}

#[test]
fn long_containers_are_shown_a_page_at_a_time() {
    let text = format!("[{}]", (0..250).map(|i| format!("\"item-{i}\"")).collect::<Vec<_>>().join(","));
    let value = Json::parse(&text).unwrap();

    let paged = render({
        let value = value.clone();
        move || html! { <JsonView value={value.clone()} page_size=100 /> }
    });
    assert!(paged.contains("item-99") && !paged.contains("item-100"));

    assert!(paged.contains("Show 100 more") && paged.contains("150 hidden"));

    let all = render(move || html! { <JsonView value={value.clone()} page_size=0 /> });
    assert!(all.contains("item-249") && !all.contains("Show"));
}

#[test]
fn the_source_variant_prints_the_whole_document() {
    let html = render(|| html! { <JsonView variant={JsonViewVariant::Source} value={doc()} /> });
    assert!(html.contains("<pre"));
    assert_eq!(count(&html, "<details"), 0);
    for shown in ["\"service\"", "eu-west-1", "fra-02", "edge"] {
        assert!(html.contains(shown), "missing {shown}");
    }
}

#[test]
fn untrusted_text_is_escaped() {
    let value = Json::parse(r#"{"<script>alert(1)</script>": "<img src=x onerror=alert(1)>"}"#).unwrap();
    for variant in JsonViewVariant::ALL {
        let value = value.clone();
        let html = render(move || html! { <JsonView {variant} value={value.clone()} /> });
        assert!(!html.contains("<script>") && !html.contains("<img"), "{variant:?}");
    }
}

#[test]
fn awkward_keys_are_quoted_in_the_tree() {
    let value = Json::parse(r#"{"": 1, "two words": 2, "plain": 3}"#).unwrap();
    let html = render(move || html! { <JsonView value={value.clone()} /> });
    assert!(html.contains(">\"\"<") && html.contains(">\"two words\"<") && html.contains(">plain<"));
}

#[test]
fn the_copy_button_is_named_and_optional() {
    let html = render(|| html! { <JsonView value={doc()} copy_label="Copy payload" /> });
    assert!(html.contains(r#"aria-label="Copy payload""#));

    let html = render(|| html! { <JsonView value={doc()} copy=false /> });
    assert!(!html.contains("<button"));
}

#[test]
fn a_height_cap_makes_a_named_focusable_region() {
    let html = render(|| html! { <JsonView value={doc()} name="facts" max_height="12rem" /> });
    assert!(html.contains(r#"role="region""#) && html.contains(r#"tabindex="0""#));
    assert!(html.contains(r#"aria-label="facts""#));
    assert!(html.contains("max-block-size:12rem"));

    let html = render(|| html! { <JsonView value={doc()} /> });
    assert!(!html.contains("tabindex") && !html.contains("role="));
}

#[test]
fn scalar_and_empty_documents_render_on_one_line() {
    for (text, shown) in [("42", "42"), ("\"hi\"", "\"hi\""), ("{}", "{}"), ("[]", "[]"), ("null", "null")] {
        for variant in JsonViewVariant::ALL {
            let value = Json::parse(text).unwrap();
            let html = render(move || html! { <JsonView {variant} value={value.clone()} /> });
            assert!(html.contains(shown), "{text} as {variant:?}");
            assert_eq!(count(&html, "<details"), 0);
        }
    }
}

#[test]
fn every_variant_renders_hydratable_markup() {
    for variant in JsonViewVariant::ALL {
        let html = render_hydratable(move || html! { <JsonView {variant} value={doc()} open=2 /> });
        assert!(html.contains("caddy"), "{variant:?}");
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <JsonView value={doc()} id="facts" class="mine" plain=true attrs={attrs! { "data-host" => "fra-01" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="facts""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-host="fra-01""#));
}
