use yew::prelude::*;
use yewi::prelude::*;

const SCHEMA: &str = r#"{
  "type": "object",
  "required": ["name", "feed"],
  "properties": {
    "name": { "type": "string", "description": "Shown in the activity log." },
    "feed": { "title": "Feed URL", "type": "string", "format": "uri", "examples": ["https://example.com/feed.xml"] },
    "token": { "title": "Access token", "type": "string", "format": "password" },
    "interval": { "title": "Check every (minutes)", "type": "integer", "minimum": 5, "maximum": 1440, "default": 60 },
    "priority": { "oneOf": [
      { "const": 1, "title": "Low" }, { "const": 2, "title": "Normal" }, { "const": 3, "title": "Urgent" }
    ] },
    "enabled": { "type": "boolean", "default": true, "description": "A paused workflow keeps its history." },
    "labels": { "type": "array", "items": { "type": "string" }, "examples": ["Add labels"] }
  }
}"#;

#[function_component]
pub fn Example() -> Html {
    let schema = use_memo((), |_| Json::parse(SCHEMA).unwrap_or(Json::Null));
    let value = use_state(|| Json::parse(r#"{"name":"Release notes","feed":"","priority":2}"#).unwrap_or(Json::Null));
    let on_change = {
        let value = value.clone();
        Callback::from(move |next| value.set(next))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <SchemaForm {schema} value={(*value).clone()} {on_change} />
            <code role="status">{ value.to_string() }</code>
        </Stack>
    }
}
