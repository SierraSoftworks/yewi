use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Entry {
    line: u32,
    tone: Tone,
    message: &'static str,
    source: &'static str,
}

/// 10,000 deterministic log entries.
fn entries() -> Rc<[Entry]> {
    const MESSAGES: [(Tone, &str); 5] = [
        (Tone::Ok, "Health check passed"),
        (Tone::Ok, "Rollout step completed"),
        (Tone::Info, "Configuration reloaded"),
        (Tone::Warn, "Retrying upstream after timeout"),
        (Tone::Danger, "Probe failed: connection refused"),
    ];
    const SOURCES: [&str; 4] = ["web-01", "web-02", "db-01", "edge-ams"];
    (0..10_000_u32)
        .map(|i| {
            let (tone, message) = MESSAGES[(i * 7 % 5) as usize];
            Entry { line: i + 1, tone, message, source: SOURCES[(i * 3 % 4) as usize] }
        })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    let items = use_memo((), |()| entries());
    let render = use_memo((), |()| {
        RowFn::new(|entry: &Entry| {
            html! {
                <Cluster justify={Justify::Between} nowrap=true>
                    <Cluster gap={Space::Sm} nowrap=true>
                        <StatusDot tone={entry.tone} />
                        <Text truncate=true>{ entry.message }</Text>
                    </Cluster>
                    <Text color={TextColor::Subtle} size={TextSize::Sm} tabular=true>
                        { format!("{} · line {}", entry.source, entry.line) }
                    </Text>
                </Cluster>
            }
        })
    });

    html! {
        <VirtualList<Entry>
            label="Log entries"
            items={(*items).clone()}
            render={(*render).clone()}
            row_height=36.0
            max_height="20rem"
            ruled=true
        />
    }
}
