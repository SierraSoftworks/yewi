use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    // On this site the clock is stopped at 12:00 UTC on Thursday 15 January 2026.
    let bake_ends = Utc.with_ymd_and_hms(2026, 1, 15, 12, 4, 5).unwrap();
    let token_expires = Utc.with_ymd_and_hms(2026, 1, 15, 13, 2, 30).unwrap();
    let freeze_starts = Utc.with_ymd_and_hms(2026, 1, 17, 15, 4, 5).unwrap();

    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Deadlines" }}>
            <DescriptionItem term="Bake ends in"><Countdown until={bake_ends} /></DescriptionItem>
            <DescriptionItem term="Token expires in"><Countdown until={token_expires} /></DescriptionItem>
            <DescriptionItem term="Freeze starts in"><Countdown until={freeze_starts} /></DescriptionItem>
        </DescriptionList>
    }
}
