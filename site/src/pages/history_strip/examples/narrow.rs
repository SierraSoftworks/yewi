use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    // 96 quarter-hours; the failure is recent, the warning long ago.
    let cells: Rc<[HistoryCell]> = (0..96)
        .map(|i| {
            let tone = match i {
                5..=7 => Tone::Warn,
                88..=90 => Tone::Danger,
                _ => Tone::Ok,
            };
            HistoryCell::new(tone, format!("{:02}:{:02}", i / 4, (i % 4) * 15))
        })
        .collect();

    html! {
        <Stack gap={Space::Md}>
            { for ["100%", "14rem", "7rem"].into_iter().map(|width| html! {
                <HistoryStrip
                    label="Last 24 hours"
                    cells={cells.clone()}
                    attrs={attrs! { "style" => format!("inline-size: {width}") }}
                />
            }) }
        </Stack>
    }
}
