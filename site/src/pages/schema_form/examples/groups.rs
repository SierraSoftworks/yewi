use yew::prelude::*;
use yewi::prelude::*;

const SCHEMA: &str = r#"{
  "type": "object",
  "required": ["name", "listen"],
  "properties": {
    "name": { "type": "string" },
    "listen": {
      "type": "object",
      "required": ["port", "tls"],
      "properties": {
        "port": { "type": "integer", "minimum": 1, "maximum": 65535 },
        "tls": {
          "title": "TLS",
          "type": "object",
          "properties": { "certificate": { "type": "string" }, "require_client": { "title": "Require a client certificate", "type": "boolean", "default": false } }
        }
      }
    },
    "peers": { "type": "array", "items": { "title": "Peer", "type": "string", "format": "uri" } }
  }
}"#;

const VALUE: &str = r#"{
  "name": "gateway",
  "listen": { "port": 8443, "tls": { "certificate": "gateway.pem" } },
  "peers": ["https://a.example.net"]
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
        <Grid min="16rem">
            { for SchemaGroups::ALL.into_iter().map(|groups| html! {
                <Stack>
                    <Text color={TextColor::Muted}>{ format!("{groups:?}") }</Text>
                    <SchemaForm schema={(*schema).clone()} value={(*value).clone()} on_change={on_change.clone()} {groups} />
                </Stack>
            }) }
        </Grid>
    }
}
