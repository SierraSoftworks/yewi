use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    let value = serde_json::json!({ "service": "caddy", "replicas": 3, "ports": [80, 443] });
    html! { <JsonView value={Json::from(value)} /> }
}
