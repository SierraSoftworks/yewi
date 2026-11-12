use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let window = Utc.with_ymd_and_hms(2026, 1, 20, 2, 0, 0).unwrap();
    let last_run = Utc.with_ymd_and_hms(2026, 1, 15, 11, 25, 0).unwrap();

    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Maintenance" }}>
            <DescriptionItem term="Next window">
                <Timestamp at={window} display={TimestampDisplay::DateTime} utc=true />
            </DescriptionItem>
            <DescriptionItem term="Starts at">
                <Timestamp at={window} display={TimestampDisplay::Time} utc=true />
            </DescriptionItem>
            <DescriptionItem term="Last run">
                <Timestamp at={last_run} utc=true />
            </DescriptionItem>
        </DescriptionList>
    }
}
