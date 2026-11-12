use yew::prelude::*;
use yewi::chrono::{DateTime, TimeDelta, Utc};
use yewi::prelude::*;

pub fn view() -> Html {
    let at = DateTime::<Utc>::from(Now::system());
    html! {
        <>
            <Timestamp {at} />
            <Duration value={TimeDelta::seconds(125)} />
            <Countdown until={at + TimeDelta::minutes(5)} />
        </>
    }
}
