use std::rc::Rc;

use yewi_test::render;

use super::*;
use crate::{Field, attrs};

fn fields() -> Rc<[QueryField]> {
    Rc::from([
        QueryField::new("status").description("Probe result").values(["passing", "failing", "unknown"]),
        QueryField::new("region").values(["eu-west-1", "eu north", "us-east-1"]),
        QueryField::new("retries").values(["0", "3"]),
    ])
}

fn labels(completion: &QueryCompletion) -> Vec<&str> {
    completion.items.iter().map(|item| item.label.as_str()).collect()
}

/// Completes `text`, where `|` marks the caret.
fn complete(text: &str, syntax: QuerySyntax) -> (String, QueryCompletion) {
    let caret = text.find('|').expect("mark the caret with |");
    let text = text.replacen('|', "", 1);
    let completion = complete_query(&text, caret, &fields(), syntax, 8);
    (text, completion)
}

/// Accepts the first suggestion and marks where the caret lands.
fn accept(text: &str, syntax: QuerySyntax) -> String {
    let (text, completion) = complete(text, syntax);
    let (mut text, caret) = completion.apply(&text, 0).expect("a suggestion to accept");
    text.insert(caret, '|');
    text
}

#[test]
fn terms_suggest_fields_then_their_values() {
    use QuerySyntax::Terms;
    for (text, expected) in [
        ("|", vec!["status:", "region:", "retries:"]),
        ("re|", vec!["region:", "retries:"]),
        ("ST|", vec!["status:"]),
        ("api status:|", vec!["passing", "failing", "unknown"]),
        ("status:ing|", vec!["passing", "failing"]),
        ("status:fail| api", vec!["failing"]),
        ("region:\"eu n|", vec!["eu north"]),
        ("region:eu| status:passing", vec!["eu-west-1", "eu north"]),
        // Substring matches follow prefix matches.
        ("region:e|", vec!["eu-west-1", "eu north", "us-east-1"]),
        ("status:failing|", vec![]),
        ("owner:|", vec![]),
        ("https://exam|", vec![]),
        ("zzz|", vec![]),
    ] {
        assert_eq!(labels(&complete(text, Terms).1), expected, "{text}");
    }
}

#[test]
fn expressions_suggest_fields_where_an_operand_starts_and_values_after_an_operator() {
    use QuerySyntax::Expression;
    for (text, expected) in [
        ("|", vec!["status", "region", "retries"]),
        ("st|", vec!["status"]),
        ("status == \"passing\" && r|", vec!["region", "retries"]),
        ("!(|", vec!["status", "region", "retries"]),
        ("status == \"passing\" or re|", vec!["region", "retries"]),
        ("status == |", vec!["passing", "failing", "unknown"]),
        ("status==\"fa|", vec!["failing"]),
        ("status != 'pass|'", vec!["passing"]),
        ("region like eu|", vec!["eu-west-1", "eu north"]),
        // Not where an operator or a connective is expected.
        ("status|", vec![]),
        ("status == \"failing\"|", vec![]),
        ("status |", vec![]),
        ("status li|", vec![]),
        ("status == \"passing\" |", vec![]),
        ("owner == |", vec![]),
        ("\"st|", vec![]),
    ] {
        assert_eq!(labels(&complete(text, Expression).1), expected, "{text}");
    }
}

#[test]
fn accepting_a_suggestion_replaces_the_token_at_the_caret() {
    use QuerySyntax::{Expression, Terms};
    for (syntax, text, expected) in [
        (Terms, "sta|", "status:|"),
        (Terms, "api status:fa|", "api status:failing |"),
        (Terms, "status:fa|x api", "status:failing |api"),
        (Terms, "region:\"eu n|", "region:\"eu north\" |"),
        (Terms, "caf\u{e9} st| later", "caf\u{e9} status:| later"),
        (Expression, "sta|", "status |"),
        (Expression, "(sta|)", "(status |)"),
        (Expression, "status == fa|", "status == \"failing\" |"),
        (Expression, "status == \"fa|\"", "status == \"failing\" |"),
        (Expression, "status == \"fa| && retries == 0", "status == \"failing\" |&& retries == 0"),
        (Expression, "retries == |", "retries == 0 |"),
    ] {
        assert_eq!(accept(text, syntax), expected, "{text}");
    }
}

#[test]
fn suggestions_are_capped_and_absent_without_fields() {
    let capped = complete_query("status:", 7, &fields(), QuerySyntax::Terms, 2);
    assert_eq!(labels(&capped), ["passing", "failing"]);
    for syntax in QuerySyntax::ALL {
        assert!(complete_query("st", 2, &[], syntax, 8).items.is_empty());
        // A caret past the end or inside a character is tolerated.
        assert!(complete_query("\u{e9}", 1, &fields(), syntax, 8).items.len() <= 8);
        assert!(!complete_query("st", 99, &fields(), syntax, 8).items.is_empty());
    }
}

#[test]
fn renders_a_closed_combobox_with_its_value() {
    let html = render(|| {
        html! { <QueryInput value="status:failing" label="Filter probes" placeholder="Filter" fields={fields()} /> }
    });
    for expected in [
        r#"role="combobox""#,
        r#"value="status:failing""#,
        r#"aria-label="Filter probes""#,
        r#"placeholder="Filter""#,
        r#"aria-expanded="false""#,
        r#"aria-autocomplete="list""#,
    ] {
        assert!(html.contains(expected), "{expected} missing from {html}");
    }
    // Nothing floats until the input is used.
    assert!(!html.contains("listbox"));
    assert!(!html.contains("popover="));
}

#[test]
fn problems_mark_the_input_invalid() {
    let invalid = render(|| html! { <QueryInput invalid=true /> });
    let error = render(|| html! { <QueryInput error="Unexpected end of input" /> });
    let field = render(|| html! { <Field label="Filter" error="Unknown field"><QueryInput /></Field> });
    for html in [invalid, error, field] {
        assert!(html.contains(r#"aria-invalid="true""#), "{html}");
    }
    let fine = render(|| html! { <QueryInput /> });
    assert!(!fine.contains("aria-invalid"));
}

#[test]
fn joins_a_field_and_passes_through_id_class_and_attrs() {
    let html = render(|| {
        html! {
            <Field label="Filter" hint="All terms must match." control_id="filter" disabled=true>
                <QueryInput class="mine" attrs={attrs! { "data-testid" => "query" }} />
            </Field>
        }
    });
    let input = &html[html.find("<input").expect("an input")..];
    let input = &input[..input.find('>').unwrap()];
    assert!(input.contains(r#"id="filter""#), "{input}");
    assert!(input.contains("aria-describedby="), "{input}");
    assert!(input.contains(" disabled"), "{input}");
    assert!(html.contains("mine"));
    assert!(html.contains(r#"data-testid="query""#));

    let alone = render(|| html! { <QueryInput id="q" name="q" /> });
    assert!(alone.contains(r#"id="q""#) && alone.contains(r#"name="q""#), "{alone}");
}
