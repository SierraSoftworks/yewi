use yew::prelude::*;
use yewi::prelude::*;

const INPUT: &str = r#"{
  "workflow": "deploy-caddy",
  "trigger": { "kind": "webhook", "id": "3f1c9a7e52b84d06" },
  "attempt": 2,
  "dry_run": false,
  "approved_by": null,
  "targets": ["edge-fra-01", "edge-fra-02"],
  "note": "Rolls back on the first failed health check."
}"#;

#[function_component]
pub fn Example() -> Html {
    let input = use_memo((), |_| Json::parse(INPUT).unwrap_or(Json::Null));

    html! {
        <JsonView
            variant={JsonViewVariant::Source}
            value={input}
            copy_label="Copy input"
            attrs={attrs! { "style" => "max-inline-size: 34rem" }}
        />
    }
}
