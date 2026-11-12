use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    let schema = Json::parse(
        r#"{"type":"object","required":["name"],"properties":{
            "name":{"type":"string"},"token":{"type":"string","format":"password"},
            "retries":{"type":"integer","minimum":0},"enabled":{"type":"boolean"},
            "level":{"enum":["info","warn"]},"tags":{"type":"array","items":{"type":"string"}},
            "hosts":{"type":"array","items":{"type":"object","properties":{"port":{"type":"integer"}}}},
            "extra":{}}}"#,
    )
    .unwrap_or(Json::Null);
    html! { <SchemaForm {schema} /> }
}
