use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Pill dot=false>{ "UTC" }</Pill>
            <Pill tone={Tone::Ok} dot=false>{ "succeeded" }</Pill>
            <Pill tone={Tone::Warn} dot=false>
                <Icon icon={icons::LOCK} />
                { "held for approval" }
            </Pill>
            <Pill dot=false>
                <StatusDot tone={Tone::Ok} />
                { "3 of 4 healthy" }
            </Pill>
        </Cluster>
    }
}
