use yew::prelude::*;
use yewi::prelude::*;

const SCHEMA: &str = r#"{
  "type": "object",
  "required": ["host", "port"],
  "properties": {
    "host": { "type": "string", "description": "The server's public name." },
    "port": { "type": "integer" },
    "window": {
      "title": "Maintenance window",
      "type": "object",
      "required": ["from", "until"],
      "properties": {
        "from": { "title": "From (hour)", "type": "integer", "minimum": 0, "maximum": 23 },
        "until": { "title": "Until (hour)", "type": "integer", "minimum": 0, "maximum": 23 }
      }
    }
  }
}"#;

/// Stands in for the server: it holds every write to the schema and to rules
/// a schema cannot express.
fn check(document: &Json) -> SchemaErrors {
    let text = document.to_string();
    let mut errors = Vec::new();
    if text.contains(r#""host":"""#) || !text.contains(r#""host""#) {
        errors.push(("/host", "A host is required"));
    }
    if text.contains(r#""port":80,"#) {
        errors.push(("/port", "Port 80 is taken by the proxy"));
    }
    if text.contains(r#""from":22,"until":4"#) {
        errors.push(("/window", "The window must start before it ends"));
    }
    errors.into()
}

#[function_component]
pub fn Example() -> Html {
    let schema = use_memo((), |_| Json::parse(SCHEMA).unwrap_or(Json::Null));
    let value = use_state(|| {
        Json::parse(r#"{"host":"","port":80,"window":{"from":22,"until":4}}"#).unwrap_or(Json::Null)
    });
    let errors = use_state(|| check(&value));
    let on_change = {
        let value = value.clone();
        Callback::from(move |next| value.set(next))
    };
    let on_click = {
        let (value, errors) = (value.clone(), errors.clone());
        Callback::from(move |_| errors.set(check(&value)))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 34rem" }}>
            <SchemaForm {schema} value={(*value).clone()} {on_change} errors={(*errors).clone()} layout={FieldLayout::Inline} />
            <Cluster>
                <Button variant={ButtonVariant::Primary} {on_click}>{ "Save" }</Button>
                <span role="status">{ if errors.is_empty() { "Saved" } else { "Not saved" } }</span>
            </Cluster>
        </Stack>
    }
}
