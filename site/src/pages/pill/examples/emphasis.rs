use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Cluster>
                <Pill tone={Tone::Active} active=true>{ "rolling" }</Pill>
                <Pill tone={Tone::Danger} active=true>{ "failing" }</Pill>
            </Cluster>
            <Cluster>
                <Pill tone={Tone::Ok} bold=true>{ "healthy" }</Pill>
                <Pill tone={Tone::Active} bold=true>{ "rolling" }</Pill>
                <Pill tone={Tone::Warn} bold=true>{ "held" }</Pill>
                <Pill tone={Tone::Danger} bold=true active=true>{ "failing" }</Pill>
                <Pill tone={Tone::Info} bold=true>{ "scheduled" }</Pill>
                <Pill bold=true>{ "offline" }</Pill>
            </Cluster>
        </Stack>
    }
}
