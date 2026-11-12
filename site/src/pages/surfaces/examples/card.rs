use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <div class="demo-card" style="max-inline-size: 22rem">
            <Stack gap={Space::Sm}>
                <b class="demo-title">{ "Latest rollout" }</b>
                <span style="color: var(--yw-fg-muted)">{ "2 of 31 hosts done, started 35 minutes ago." }</span>
            </Stack>
        </div>
    }
}
