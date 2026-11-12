use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let digest = use_state(|| true);
    let on_change = {
        let digest = digest.clone();
        Callback::from(move |value| digest.set(value))
    };

    html! {
        <ToggleSwitch
            checked={*digest}
            {on_change}
            description="One email each Monday with the week's deploys and incidents."
        >
            { "Weekly digest" }
        </ToggleSwitch>
    }
}
