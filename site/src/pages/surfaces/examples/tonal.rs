use yew::prelude::*;
use yewi::prelude::*;

const ROWS: [(&str, &str); 3] = [("Hosts", "2 / 31 done"), ("Started", "35m ago"), ("Stage", "home")];

#[function_component]
pub fn Example() -> Html {
    html! {
        <div class="demo-card demo-card--flush" style="max-inline-size: 22rem">
            <b class="demo-title" style="display: block; padding: var(--yw-space-md) var(--yw-space-lg)">
                { "Latest rollout" }
            </b>
            { for ROWS.into_iter().enumerate().map(|(i, (label, value))| html! {
                <Cluster
                    justify={Justify::Between}
                    attrs={attrs! { "style" => format!(
                        "padding: var(--yw-space-sm) var(--yw-space-lg); background: var(--yw-tone-{})",
                        i % 2 + 1,
                    ) }}
                >
                    <span style="color: var(--yw-fg-subtle)">{ label }</span>
                    <span>{ value }</span>
                </Cluster>
            }) }
        </div>
    }
}
