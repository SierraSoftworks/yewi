use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let passed = Utc.with_ymd_and_hms(2026, 1, 15, 11, 58, 0).unwrap();

    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Passed deadlines" }}>
            <DescriptionItem term="Without done">
                <Countdown until={passed} />
            </DescriptionItem>
            <DescriptionItem term="With done">
                <Countdown until={passed} done={html! { <Pill tone={Tone::Warn}>{ "expired" }</Pill> }} />
            </DescriptionItem>
        </DescriptionList>
    }
}
