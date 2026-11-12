use yew::prelude::*;
use yewi::prelude::*;

const BODIES: [&str; 2] =
    [r#"{"status": "ok", "took_ms": 42, "items": [1, 2, 3]}"#, r#"{"status": "ok", "took_ms": 42, "items": [1, 2,"#];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Lg} attrs={attrs! { "style" => "max-inline-size: 34rem" }}>
            { for BODIES.into_iter().map(|body| html! { <Response {body} /> }) }
        </Stack>
    }
}

#[derive(Properties, PartialEq)]
struct ResponseProps {
    body: &'static str,
}

#[function_component]
fn Response(props: &ResponseProps) -> Html {
    let parsed = use_memo(props.body, |body| Json::parse(body));

    match &*parsed {
        Ok(value) => html! { <JsonView value={value.clone()} /> },
        Err(error) => html! {
            <Alert tone={Tone::Danger} title="The response is not valid JSON">
                { error.to_string() }
            </Alert>
        },
    }
}
