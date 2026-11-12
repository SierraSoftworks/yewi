use yew::prelude::*;
use yewi::chrono::{TimeZone, Utc};
use yewi::prelude::*;

const UPDATES: [(Tone, &str, (u32, u32), &str); 4] = [
    (Tone::Danger, "Investigating", (10, 32), "Requests to the API from Europe are failing with TLS errors."),
    (Tone::Danger, "Identified", (11, 5), "A certificate on the regional load balancer expired."),
    (Tone::Warn, "Monitoring", (11, 48), "Traffic is back on the primary region."),
    (Tone::Ok, "Resolved", (11, 58), "Latency is back to normal."),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Timeline orientation={TimelineOrientation::Horizontal} label="API incident, oldest first">
            { for UPDATES.into_iter().map(|(tone, status, (hour, minute), message)| {
                let at = Utc.with_ymd_and_hms(2026, 1, 15, hour, minute, 0).unwrap();
                html! {
                    <TimelineItem
                        {tone}
                        title={status}
                        meta={html! { <Timestamp {at} display={TimestampDisplay::DateTime} tooltip=false /> }}
                    >
                        { message }
                    </TimelineItem>
                }
            }) }
        </Timeline>
    }
}
