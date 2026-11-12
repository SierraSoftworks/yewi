use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

/// One half-hour bucket of a probe's history, from 12:00 yesterday.
#[derive(Clone, PartialEq)]
struct Bucket {
    time: String,
    availability: f64,
    latency_ms: u32,
}

impl Bucket {
    fn tone(&self) -> Tone {
        match self.availability {
            a if a >= 99.9 => Tone::Ok,
            a if a >= 90.0 => Tone::Warn,
            _ => Tone::Danger,
        }
    }

    fn status(&self) -> &'static str {
        match self.tone() {
            Tone::Ok => "Passing",
            Tone::Warn => "Degraded",
            _ => "Failing",
        }
    }
}

fn history() -> Rc<[Bucket]> {
    (0..48)
        .map(|i| {
            let availability = match i {
                17 => 96.4,
                30 | 31 => 41.7,
                32 => 0.0,
                33 => 92.5,
                _ => 100.0,
            };
            let latency_ms = 38 + (i * 7) % 11 + if availability < 100.0 { 900 } else { 0 };
            Bucket { time: format!("{:02}:{:02}", (12 + i / 2) % 24, (i % 2) * 30), availability, latency_ms }
        })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    let history = use_memo((), |_| history());
    let cells: Rc<[HistoryCell]> =
        history.iter().map(|b| HistoryCell::new(b.tone(), format!("{}, {}", b.time, b.status()))).collect();

    let detail = {
        let history = history.clone();
        Callback::from(move |index: usize| {
            let bucket = &history[index];
            HistoryDetail::new(html! {
                <span style="display: grid; grid-template-columns: auto auto; gap: var(--yi-space-2xs) var(--yi-space-lg)">
                    <span style="color: var(--yi-fg-muted)">{ "Latency" }</span>
                    <span>{ format!("{} ms", bucket.latency_ms) }</span>
                    <span style="color: var(--yi-fg-muted)">{ "Availability" }</span>
                    <span>{ format!("{:.1}%", bucket.availability) }</span>
                </span>
            })
            .head(html! { <><StatusDot tone={bucket.tone()} />{ bucket.status() }</> })
            .foot(format!("{} UTC, 30 minutes", bucket.time))
        })
    };

    html! {
        <Stack gap={Space::Xs} attrs={attrs! { "style" => "max-inline-size: 32rem" }}>
            <Cluster justify={Justify::Between}>
                <span>{ "api.example.com" }</span>
                <span style="color: var(--yi-fg-muted)">{ "99.2% over 24 hours" }</span>
            </Cluster>
            <HistoryStrip label="api.example.com, last 24 hours" {cells} {detail} />
        </Stack>
    }
}
