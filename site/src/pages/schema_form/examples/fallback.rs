use yew::prelude::*;
use yewi::prelude::*;

const SCHEMA: &str = r#"{
  "type": "object",
  "required": ["image"],
  "properties": {
    "image": { "type": "string" },
    "replicas": { "type": "integer", "minimum": 1 },
    "env": {
      "title": "Environment",
      "description": "Variables passed to the container, as a JSON object.",
      "type": "object",
      "additionalProperties": { "type": "string" }
    }
  }
}"#;

#[function_component]
pub fn Example() -> Html {
    let schema = use_memo((), |_| Json::parse(SCHEMA).unwrap_or(Json::Null));
    let value = use_state(|| {
        Json::parse(r#"{"image":"caddy:2.8","replicas":2,"env":{"LOG_LEVEL":"info"}}"#).unwrap_or(Json::Null)
    });
    let saving = use_state(|| false);
    let on_change = {
        let value = value.clone();
        Callback::from(move |next| value.set(next))
    };
    let on_lock = {
        let saving = saving.clone();
        Callback::from(move |on| saving.set(on))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <ToggleSwitch checked={*saving} on_change={on_lock}>{ "Saving" }</ToggleSwitch>
            <SchemaForm {schema} value={(*value).clone()} {on_change} disabled={*saving} />
            <code role="status">{ value.to_string() }</code>
        </Stack>
    }
}
