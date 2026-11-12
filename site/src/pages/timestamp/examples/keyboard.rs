use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let expires = Utc.with_ymd_and_hms(2026, 1, 15, 15, 30, 0).unwrap();

    html! {
        <Cluster>
            <Button>{ "Renew certificate" }</Button>
            <span>
                { "Expires " }
                <Timestamp at={expires} focusable=true />
            </span>
        </Cluster>
    }
}
