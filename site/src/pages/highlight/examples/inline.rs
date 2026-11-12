use yew::prelude::*;
use yewi::prelude::*;

const QUERY: &str = "SELECT service, count(*) AS errors
FROM events
WHERE status >= 500 -- server errors only
GROUP BY service;";

const BOX: &str = "margin: 0; padding: var(--yi-space-md); border-radius: var(--yi-radius-control); \
                   background: var(--yi-code-bg); overflow-x: auto";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <p>
                { "Read the flag with " }
                <Highlight language="rust" code={r#"std::env::var("LOG_LEVEL")"#} />
                { ", or set it for one run: " }
                <Highlight language="sh" code="LOG_LEVEL=debug cargo run" />
                { "." }
            </p>
            <pre style={BOX}><Highlight language="sql" code={QUERY} /></pre>
        </Stack>
    }
}
