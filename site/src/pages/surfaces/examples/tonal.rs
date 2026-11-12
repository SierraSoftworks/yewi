use yew::prelude::*;
use yewi::prelude::*;

const ROWS: [(&str, &str); 3] = [("Hosts", "2 / 31 done"), ("Started", "35m ago"), ("Stage", "home")];

#[function_component]
pub fn Example() -> Html {
    html! {
        <div class="demo-card demo-card--flush" style="max-inline-size: 22rem">
            <b class="demo-title" style="display: block; padding: var(--yi-space-md) var(--yi-space-lg)">
                { "Latest rollout" }
            </b>
            { for ROWS.into_iter().enumerate().map(|(i, (label, value))| html! {
                <Cluster
                    justify={Justify::Between}
                    attrs={attrs! { "style" => format!(
                        "padding: var(--yi-space-sm) var(--yi-space-lg); background: var(--yi-tone-{})",
                        i % 2 + 1,
                    ) }}
                >
                    <span style="color: var(--yi-fg-subtle)">{ label }</span>
                    <span>{ value }</span>
                </Cluster>
            }) }
        </div>
    }
}
