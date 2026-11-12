use yew::prelude::*;
use yewi::prelude::*;

// What `schemars` derives for a settings struct with an optional, internally
// tagged enum, a nested struct and a `Vec` of structs.
const SCHEMA: &str = r##"{
  "type": "object",
  "required": ["retry"],
  "properties": {
    "area": {
      "description": "Where the feed is looking. Leave it out to cover everything.",
      "anyOf": [{ "$ref": "#/$defs/Area" }, { "type": "null" }]
    },
    "retry": {
      "type": "object",
      "required": ["attempts"],
      "properties": {
        "attempts": { "type": "integer", "minimum": 0, "maximum": 10, "default": 3 },
        "backoff": { "title": "Back off", "type": "boolean", "default": true }
      }
    },
    "stations": {
      "description": "Receivers to read from.",
      "type": "array",
      "items": {
        "title": "Station",
        "type": "object",
        "required": ["host", "port"],
        "properties": { "host": { "type": "string" }, "port": { "type": "integer", "default": 30005 } }
      }
    }
  },
  "$defs": { "Area": { "oneOf": [
    {
      "type": "object",
      "required": ["kind", "lat", "lon", "radius_km"],
      "properties": {
        "kind": { "const": "circle" },
        "lat": { "title": "Latitude", "type": "number", "minimum": -90, "maximum": 90 },
        "lon": { "title": "Longitude", "type": "number", "minimum": -180, "maximum": 180 },
        "radius_km": { "title": "Radius (km)", "type": "number", "default": 50 }
      }
    },
    {
      "title": "Bounding box",
      "type": "object",
      "required": ["kind", "lat", "lon", "north", "east"],
      "properties": {
        "kind": { "const": "bbox" },
        "lat": { "title": "South", "type": "number" },
        "lon": { "title": "West", "type": "number" },
        "north": { "type": "number" },
        "east": { "type": "number" }
      }
    }
  ] } }
}"##;

const VALUE: &str = r#"{
  "area": { "kind": "circle", "lat": 51.5, "lon": -0.12, "radius_km": 50 },
  "retry": { "attempts": 3 },
  "stations": [{ "host": "rx1.example.net", "port": 30005 }]
}"#;

#[function_component]
pub fn Example() -> Html {
    let schema = use_memo((), |_| Json::parse(SCHEMA).unwrap_or(Json::Null));
    let value = use_state(|| Json::parse(VALUE).unwrap_or(Json::Null));
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
