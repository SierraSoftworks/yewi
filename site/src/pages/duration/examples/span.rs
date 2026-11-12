use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let started = Utc.with_ymd_and_hms(2026, 1, 15, 9, 12, 30).unwrap();
    let finished = Utc.with_ymd_and_hms(2026, 1, 15, 9, 31, 4).unwrap();

    html! {
        <span>
            { "Rollout finished " }
            <Timestamp at={finished} />
            { " after " }
            <Duration since={started} until={finished} style={DurationStyle::Long} />
            { "." }
        </span>
    }
}
