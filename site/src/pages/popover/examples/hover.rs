use yew::prelude::*;
use yewi::prelude::*;

/// One bucket of a probe's history: time, whether it passed, latency, availability.
const HISTORY: [(&str, bool, &str, &str); 6] = [
    ("11:00", true, "41 ms", "100%"),
    ("11:10", true, "44 ms", "100%"),
    ("11:20", false, "2.1 s", "62.5%"),
    ("11:30", false, "timeout", "0%"),
    ("11:40", true, "48 ms", "98.3%"),
    ("11:50", true, "42 ms", "100%"),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xxs}>
            { for HISTORY.into_iter().map(|(time, passing, latency, availability)| {
                let tone = if passing { "var(--yi-ok)" } else { "var(--yi-danger)" };
                let status = if passing { "Passing" } else { "Failing" };
                let header = html! {
                    <>
                        <span style={format!("inline-size: 0.5rem; block-size: 0.5rem; border-radius: 50%; background: {tone}")} />
                        { status }
                    </>
                };
                // The layout inside a popover is the caller's.
                let content = html! {
                    <span style="display: grid; grid-template-columns: auto auto; gap: var(--yi-space-2xs) var(--yi-space-lg)">
                        <span style="color: var(--yi-fg-muted)">{ "Latency" }</span>
                        <span>{ latency }</span>
                        <span style="color: var(--yi-fg-muted)">{ "Availability" }</span>
                        <span>{ availability }</span>
                    </span>
                };

                html! {
                    <Popover
                        trigger={PopoverTrigger::Hover}
                        focusable=true
                        {header}
                        {content}
                        footer={format!("15 January 2026, {time} UTC")}
                        attrs={attrs! { "role" => "img", "aria-label" => format!("{time}: {status}") }}
                    >
                        <span style={format!("inline-size: 1.25rem; block-size: 2rem; border-radius: var(--yi-radius-sm); background: {tone}")} />
                    </Popover>
                }
            }) }
        </Cluster>
    }
}
