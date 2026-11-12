use yew::prelude::*;
use yewi::prelude::*;

const EVENT: &str = r#"{
  "id": "evt_01JD3Q7",
  "type": "rollout.paused",
  "at": "2026-01-15T11:42:07Z",
  "actor": { "kind": "schedule", "name": "nightly" },
  "hosts": { "done": 2, "total": 31, "failed": [] },
  "stages": ["canary", "staging", "production"]
}"#;

#[function_component]
pub fn Example() -> Html {
    let event = use_memo((), |_| Json::parse(EVENT).unwrap_or(Json::Null));

    html! {
        <Grid min="18rem" gap={Space::Lg}>
            <Stack>
                <strong>{ "Tree" }</strong>
                <JsonView value={event.clone()} open=2 />
            </Stack>
            <Stack>
                <strong>{ "Source" }</strong>
                <JsonView variant={JsonViewVariant::Source} value={event} />
            </Stack>
        </Grid>
    }
}
