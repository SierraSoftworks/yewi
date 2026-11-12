use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

const UPDATES: [(Tone, &str, (u32, u32), &str); 4] = [
    (Tone::Warn, "Monitoring", (11, 48), "Traffic is back on the primary region. Error rates are normal; we are watching latency."),
    (Tone::Danger, "Identified", (11, 5), "A certificate on the regional load balancer expired. A replacement is being issued."),
    (Tone::Danger, "Investigating", (10, 32), "Requests to the API from Europe are failing with TLS errors."),
    (Tone::Ok, "Operational", (9, 0), "All probes passing."),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Timeline label="Incident updates">
            { for UPDATES.into_iter().enumerate().map(|(i, (tone, status, (hour, minute), message))| {
                let at = Utc.with_ymd_and_hms(2026, 1, 15, hour, minute, 0).unwrap();
                html! {
                    <TimelineItem
                        {tone}
                        active={i == 0}
                        title={status}
                        meta={html! { <Timestamp {at} display={TimestampDisplay::DateTime} /> }}
                    >
                        { message }
                    </TimelineItem>
                }
            }) }
        </Timeline>
    }
}
