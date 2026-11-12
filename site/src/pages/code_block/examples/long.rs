use yew::prelude::*;
use yewi::prelude::*;

const LOG: &str = r#"2026-01-15T11:58:02Z INFO  rollout{job="docs" region="fra"}: placed 4 of 4 allocations in 1.8s, waiting for health checks to pass
2026-01-15T11:58:09Z WARN  rollout{job="docs" region="fra"}: allocation 7f3a9c1e is slow to start: image pull took 6.2s (caddy:2.8-alpine)
2026-01-15T11:58:14Z INFO  rollout{job="docs" region="fra"}: all allocations healthy, promoting canaries and draining the previous version
2026-01-15T11:58:15Z INFO  rollout{job="docs" region="ams"}: placed 4 of 4 allocations in 1.1s, waiting for health checks to pass
2026-01-15T11:58:21Z INFO  rollout{job="docs" region="ams"}: all allocations healthy, promoting canaries and draining the previous version
2026-01-15T11:58:21Z INFO  rollout{job="docs"}: finished in 19.4s
"#;

#[function_component]
pub fn Example() -> Html {
    let wrap = use_state(|| false);
    let toggle = {
        let wrap = wrap.clone();
        Callback::from(move |_| wrap.set(!*wrap))
    };
    let actions = html! {
        <Button size={Size::Small} variant={ButtonVariant::Subtle} pressed={*wrap} on_click={toggle}>
            { "Wrap" }
        </Button>
    };

    html! {
        <Stack>
            <CodeBlock code={LOG} filename="rollout.log" wrap={*wrap} line_numbers=true {actions} />
            <CodeBlock
                code={LOG.repeat(3)}
                filename="rollout.log"
                attrs={attrs! { "style" => "--yi-code-block-max-height: 9rem" }}
            />
        </Stack>
    }
}
