use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let at = Utc.with_ymd_and_hms(2026, 1, 12, 14, 5, 0).unwrap();

    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Display modes" }}>
            <DescriptionItem term="Auto"><Timestamp {at} /></DescriptionItem>
            <DescriptionItem term="Relative">
                <Timestamp {at} display={TimestampDisplay::Relative} />
            </DescriptionItem>
            <DescriptionItem term="DateTime">
                <Timestamp {at} display={TimestampDisplay::DateTime} />
            </DescriptionItem>
            <DescriptionItem term="Date">
                <Timestamp {at} display={TimestampDisplay::Date} />
            </DescriptionItem>
            <DescriptionItem term="Time">
                <Timestamp {at} display={TimestampDisplay::Time} />
            </DescriptionItem>
        </DescriptionList>
    }
}
