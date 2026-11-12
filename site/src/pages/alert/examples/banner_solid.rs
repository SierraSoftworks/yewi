use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Banner variant={BannerVariant::Solid} tone={Tone::Ok} size={Size::Large}>
                { "All services are operating normally" }
            </Banner>
            <Banner variant={BannerVariant::Solid} tone={Tone::Warn} size={Size::Large}>
                { "Some services are degraded" }
            </Banner>
            <Banner variant={BannerVariant::Solid} tone={Tone::Danger}>
                { "Major outage: 4 services are unavailable" }
            </Banner>
            <Banner variant={BannerVariant::Solid} tone={Tone::Neutral} size={Size::Small}>
                { "Demo data. Nothing here is live." }
            </Banner>
        </Stack>
    }
}
