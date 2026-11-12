use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let until = Utc.with_ymd_and_hms(2026, 1, 15, 12, 4, 5).unwrap();

    html! {
        <Stack>
            <span>
                <Pill tone={Tone::Active} active=true>{ "baking" }</Pill>
                { " " }
                <Countdown {until} />
            </span>
            <span>
                { "Promotes to the next ring in " }
                <Countdown {until} format={CountdownFormat::Units} />
                { "." }
            </span>
        </Stack>
    }
}
