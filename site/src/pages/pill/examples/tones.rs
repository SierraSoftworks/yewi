use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Pill tone={Tone::Ok}>{ "healthy" }</Pill>
            <Pill tone={Tone::Active}>{ "rolling" }</Pill>
            <Pill tone={Tone::Warn}>{ "held" }</Pill>
            <Pill tone={Tone::Danger}>{ "failing" }</Pill>
            <Pill tone={Tone::Info}>{ "scheduled" }</Pill>
            <Pill>{ "offline" }</Pill>
        </Cluster>
    }
}
