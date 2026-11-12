use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

/// The last runs of a nightly job: day of January, outcome, duration.
const RUNS: [(u32, Tone, &str); 12] = [
    (3, Tone::Ok, "4m 12s"),
    (4, Tone::Ok, "4m 02s"),
    (5, Tone::Ok, "4m 31s"),
    (6, Tone::Danger, "0m 48s"),
    (7, Tone::Ok, "4m 09s"),
    (8, Tone::Ok, "4m 15s"),
    (9, Tone::Warn, "9m 57s"),
    (10, Tone::Ok, "4m 20s"),
    (11, Tone::Ok, "4m 06s"),
    (12, Tone::Ok, "4m 11s"),
    (13, Tone::Ok, "4m 18s"),
    (14, Tone::Active, "running"),
];

fn outcome(tone: Tone) -> &'static str {
    match tone {
        Tone::Ok => "Succeeded",
        Tone::Warn => "Slow",
        Tone::Danger => "Failed",
        Tone::Active => "Running",
        _ => "Skipped",
    }
}

#[function_component]
pub fn Example() -> Html {
    let selected = use_state(|| None::<u32>);
    let cells: Rc<[HistoryCell]> =
        RUNS.into_iter().map(|(day, tone, _)| HistoryCell::new(tone, format!("{day} January, {}", outcome(tone)))).collect();

    let detail = Callback::from(|index: usize| {
        let (day, tone, duration) = RUNS[index];
        HistoryDetail::new(format!("Duration {duration}"))
            .head(html! { <><StatusDot {tone} />{ outcome(tone) }</> })
            .foot(format!("{day} January 2026, 02:00 UTC"))
    });
    let on_select = {
        let selected = selected.clone();
        Callback::from(move |index: usize| selected.set(Some(RUNS[index].0)))
    };

    html! {
        <Stack gap={Space::Sm} attrs={attrs! { "style" => "max-inline-size: 24rem" }}>
            <Cluster justify={Justify::Between} align={Align::Center}>
                <span>{ "nightly-backup" }</span>
                <HistoryStrip
                    label="nightly-backup, last 12 runs"
                    variant={HistoryStripVariant::Discrete}
                    size={Size::Large}
                    {cells}
                    {detail}
                    {on_select}
                    attrs={attrs! { "style" => "inline-size: 12rem" }}
                />
            </Cluster>
            <span role="status" style="color: var(--yi-fg-muted)">
                { selected.map_or_else(|| "No run selected".to_string(), |day| format!("Selected the run of {day} January")) }
            </span>
        </Stack>
    }
}
