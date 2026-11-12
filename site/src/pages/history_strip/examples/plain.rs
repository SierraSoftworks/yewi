use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

/// A service and its last 24 hours, one character per hour:
/// `o` passing, `w` degraded, `x` failing, `-` no data.
const SERVICES: [(&str, &str); 4] = [
    ("gateway", "oooooooooooooooooooooooo"),
    ("accounts", "oooooowwoooooooooxxooooo"),
    ("search", "------------oooooooooooo"),
    ("billing", "ooooooooooooooooooooowww"),
];

fn cells(hours: &str) -> Rc<[HistoryCell]> {
    hours
        .chars()
        .enumerate()
        .map(|(hour, state)| {
            let (tone, status) = match state {
                'o' => (Tone::Ok, "passing"),
                'w' => (Tone::Warn, "degraded"),
                'x' => (Tone::Danger, "failing"),
                _ => (Tone::Neutral, "no data"),
            };
            HistoryCell::new(tone, format!("{hour:02}:00, {status}"))
        })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <div style="display: grid; grid-template-columns: auto 1fr; gap: var(--yi-space-sm) var(--yi-space-lg); align-items: center; max-inline-size: 24rem">
            { for SERVICES.into_iter().map(|(service, hours)| html! {
                <>
                    <span>{ service }</span>
                    <HistoryStrip label={format!("{service}, last 24 hours")} cells={cells(hours)} size={Size::Small} />
                </>
            }) }
        </div>
    }
}
