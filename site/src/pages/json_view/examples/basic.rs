use yew::prelude::*;
use yewi::prelude::*;

const FACTS: &str = r#"{
  "hostname": "edge-fra-02",
  "os": { "family": "debian", "release": "13.2", "kernel": "6.12.48-amd64" },
  "cpu": { "model": "AMD EPYC 7443P", "cores": 24, "flags": ["avx2", "aes", "sha_ni"] },
  "memory_mb": 65536,
  "virtual": false,
  "maintenance_window": null,
  "interfaces": [
    { "name": "eth0", "addresses": ["185.12.64.7", "2a01:4f8:c17::7"], "mtu": 1500 },
    { "name": "wg0", "addresses": ["10.44.0.7"], "mtu": 1420 }
  ],
  "labels": { "role": "edge", "region": "eu-central", "tier": "production" }
}"#;

#[function_component]
pub fn Example() -> Html {
    let facts = use_memo((), |_| Json::parse(FACTS).unwrap_or(Json::Null));

    html! {
        <JsonView value={facts} name="facts" attrs={attrs! { "style" => "max-inline-size: 34rem" }} />
    }
}
