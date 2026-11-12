use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <h1>{ "Rollouts" }</h1>
            <h2>{ "Latest rollout" }</h2>
            <h3>{ "Stage: home" }</h3>
            <h4>{ "Hosts in this stage" }</h4>
            <h5>{ "Health checks" }</h5>
            <p style="color: var(--yw-fg-muted)">{ "Commits and operator actions, newest first." }</p>
        </Stack>
    }
}
