use yew::prelude::*;
use yewi::prelude::*;

const DETAIL: &str = r#"{
  "action": "host.approve",
  "target": { "host": "edge-fra-02", "rollout": 1842 },
  "changes": { "state": ["pending", "approved"], "approved_at": "2026-01-15T11:58:31Z" },
  "client": { "ip": "185.12.64.20", "agent": "buckle-cli/0.9.3" }
}"#;

#[function_component]
pub fn Example() -> Html {
    let detail = use_memo((), |_| Json::parse(DETAIL).unwrap_or(Json::Null));

    html! {
        <Card
            title="Audit entry 20417"
            subtitle="Recorded 2 minutes ago"
            attrs={attrs! { "style" => "max-inline-size: 30rem" }}
        >
            <JsonView value={detail} open=3 plain=true copy=false />
        </Card>
    }
}
