use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Banner bold=true tone={Tone::Ok} size={Size::Large}>
                { "All services are operating normally" }
            </Banner>
            <Banner bold=true tone={Tone::Warn} size={Size::Large}>
                { "Some services are degraded" }
            </Banner>
            <Banner bold=true tone={Tone::Danger}>
                { "Major outage: 4 services are unavailable" }
            </Banner>
            <Banner bold=true tone={Tone::Neutral} size={Size::Small}>
                { "Demo data. Nothing here is live." }
            </Banner>
        </Stack>
    }
}
