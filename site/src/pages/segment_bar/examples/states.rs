use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

/// A stage's hosts: done, in flight, held, failed, skipped.
const STAGES: [(&str, [f64; 5]); 4] = [
    ("canary", [4.0, 0.0, 0.0, 0.0, 0.0]),
    ("eu-west", [31.0, 6.0, 0.0, 1.0, 2.0]),
    ("us-east", [8.0, 4.0, 12.0, 0.0, 0.0]),
    ("ap-south", [0.0, 0.0, 0.0, 0.0, 0.0]),
];

const STATES: [(Tone, &str); 5] = [
    (Tone::Ok, "done"),
    (Tone::Active, "in flight"),
    (Tone::Warn, "held"),
    (Tone::Danger, "failed"),
    (Tone::Neutral, "skipped"),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <div style="display: grid; grid-template-columns: auto 1fr auto; gap: var(--yi-space-sm) var(--yi-space-lg); align-items: center; max-inline-size: 30rem">
            { for STAGES.into_iter().map(|(stage, counts)| {
                let segments: Rc<[Segment]> = STATES
                    .into_iter()
                    .zip(counts)
                    .map(|((tone, label), count)| Segment::new(count, tone, label))
                    .collect();
                html! {
                    <>
                        <span>{ stage }</span>
                        <SegmentBar label={stage} {segments} total={40.0} rest_label="to go" />
                        <span style="color: var(--yi-fg-muted); font-variant-numeric: tabular-nums">
                            { format!("{} / 40", counts[0]) }
                        </span>
                    </>
                }
            }) }
        </div>
    }
}
