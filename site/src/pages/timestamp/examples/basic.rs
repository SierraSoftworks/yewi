use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    // On this site the clock is stopped at 12:00 UTC on Thursday 15 January 2026.
    let events = [
        ("Deployed", Utc.with_ymd_and_hms(2026, 1, 15, 11, 25, 0)),
        ("Built", Utc.with_ymd_and_hms(2026, 1, 15, 8, 40, 0)),
        ("Approved", Utc.with_ymd_and_hms(2026, 1, 13, 14, 5, 0)),
        ("Opened", Utc.with_ymd_and_hms(2025, 11, 28, 9, 30, 0)),
        ("Next rollout", Utc.with_ymd_and_hms(2026, 1, 15, 12, 20, 0)),
    ];

    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Release history" }}>
            { for events.into_iter().map(|(term, at)| html! {
                <DescriptionItem {term}>
                    <Timestamp at={at.unwrap()} />
                </DescriptionItem>
            }) }
        </DescriptionList>
    }
}
