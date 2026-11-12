use yew::prelude::*;
use yewi::prelude::*;

const BEFORE: &str = r#"[server]
listen = "0.0.0.0:8080"
workers = 4

[storage]
path = "/var/lib/buckle"
retention = "30d"

[log]
level = "debug"
"#;

const AFTER: &str = r#"[server]
listen = "0.0.0.0:8443"
workers = 4
tls = true

[storage]
path = "/var/lib/buckle"
retention = "90d"
"#;

#[function_component]
pub fn Example() -> Html {
    let mode = use_state(|| DiffMode::Split);

    let choice = |label: &'static str, value: DiffMode| {
        let mode = mode.clone();
        let pressed = if *mode == value { "true" } else { "false" };
        html! {
            <Button
                size={Size::Small}
                attrs={attrs! { "aria-pressed" => pressed }}
                on_click={Callback::from(move |_| mode.set(value))}
            >
                { label }
            </Button>
        }
    };

    html! {
        <Stack>
            <Cluster>
                { choice("Side by side", DiffMode::Split) }
                { choice("Unified", DiffMode::Unified) }
            </Cluster>
            <Diff title="buckle.toml" mode={*mode} before={BEFORE} after={AFTER} />
        </Stack>
    }
}
