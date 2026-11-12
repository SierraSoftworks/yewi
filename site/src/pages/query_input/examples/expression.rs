use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

/// Stands in for the app's own parser.
fn problem(query: &str) -> Option<&'static str> {
    if query.matches('"').count() % 2 == 1 {
        Some("Unterminated string: add the closing quote.")
    } else if query.matches('(').count() != query.matches(')').count() {
        Some("Unbalanced brackets.")
    } else if query.trim_end().ends_with(['=', '&', '|']) {
        Some("The expression ends where a value was expected.")
    } else {
        None
    }
}

#[function_component]
pub fn Example() -> Html {
    let applied = use_state(|| r#"browser == "Firefox""#.to_string());
    let draft = use_state(|| (*applied).clone());
    let error = use_state(|| None::<&'static str>);

    let fields = use_memo((), |_| -> Rc<[QueryField]> {
        Rc::from([
            QueryField::new("browser").description("Browser family").values(["Chrome", "Firefox", "Safari", "Edge"]),
            QueryField::new("country").description("ISO country code").values(["DE", "FR", "GB", "US", "ZA"]),
            QueryField::new("path").description("Page path"),
            QueryField::new("bounced").description("Left after one page").values(["true", "false"]),
        ])
    });

    let on_input = {
        let (draft, error) = (draft.clone(), error.clone());
        Callback::from(move |next| {
            error.set(None);
            draft.set(next);
        })
    };
    let on_submit = {
        let (applied, error) = (applied.clone(), error.clone());
        Callback::from(move |query: String| match problem(&query) {
            Some(message) => error.set(Some(message)),
            None => applied.set(query),
        })
    };

    html! {
        <Stack>
            <QueryInput
                syntax={QuerySyntax::Expression}
                value={(*draft).clone()}
                applied={(*applied).clone()}
                error={error.map(AttrValue::from)}
                {on_input}
                {on_submit}
                fields={(*fields).clone()}
                label="Filter query"
                placeholder={r#"Filter, e.g. browser == "Chrome" && country != "DE""#}
            />
            <span role="status">
                { if applied.is_empty() { "No filter applied".to_string() } else { format!("Applied: {}", *applied) } }
            </span>
        </Stack>
    }
}
