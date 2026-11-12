use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Pill tone={Tone::Active} active=true href="#rollout-42">{ "rolling" }</Pill>
            <Pill tone={Tone::Danger} bold=true href="#incident-7">{ "incident open" }</Pill>
        </Cluster>
    }
}
