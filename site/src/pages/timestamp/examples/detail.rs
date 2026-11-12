use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let signed = Utc.with_ymd_and_hms(2026, 1, 15, 11, 25, 42).unwrap();

    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Signature" }}>
            <DescriptionItem term="Signed by">{ "release-bot" }</DescriptionItem>
            <DescriptionItem term="Signed at">
                <TimestampDetail at={signed} />
            </DescriptionItem>
        </DescriptionList>
    }
}
