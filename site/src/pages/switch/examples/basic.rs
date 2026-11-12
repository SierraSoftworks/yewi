use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let paused = use_state(|| false);
    let on_change = {
        let paused = paused.clone();
        Callback::from(move |value| paused.set(value))
    };

    html! {
        <Stack>
            <ToggleSwitch checked={*paused} {on_change}>{ "Pause deploys" }</ToggleSwitch>
            <span role="status">{ if *paused { "Deploys are paused" } else { "Deploys are running" } }</span>
        </Stack>
    }
}
