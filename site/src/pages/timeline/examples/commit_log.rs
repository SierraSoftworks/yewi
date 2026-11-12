use yew::prelude::*;
use yewi::chrono::{DateTime, TimeZone, Utc};
use yewi::prelude::*;

fn at(day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 1, day, hour, minute, 0).unwrap()
}

fn by(name: &'static str, at: DateTime<Utc>) -> Html {
    html! {
        <>
            <Cluster gap={Space::Xs}><Avatar {name} size={Size::Small} decorative=true />{ name }</Cluster>
            <Timestamp {at} />
        </>
    }
}

#[function_component]
pub fn Example() -> Html {
    let opened = use_state(|| None::<&'static str>);
    let open = |hash: &'static str| {
        let opened = opened.clone();
        Callback::from(move |_: MouseEvent| opened.set(Some(hash)))
    };
    let hash = |hash: &'static str| html! { <InlineCode>{ hash }</InlineCode> };

    html! {
        <Stack gap={Space::Lg}>
            <Timeline label="Revisions">
                <TimelineItem
                    title="Raise the proxy's upstream timeout"
                    meta={by("Grace Hopper", at(15, 11, 25))}
                    aside={html! { <><Pill tone={Tone::Active} active=true>{ "Rolling out" }</Pill>{ hash("9c41e07") }</> }}
                    on_click={open("9c41e07")}
                />
                <TimelineItem
                    title="Restart nginx on the edge hosts"
                    marker={TimelineMarker::Diamond}
                    tone={Tone::Info}
                    tone_label="operator action"
                    meta={by("Ada Lovelace", at(15, 9, 40))}
                    aside={hash("nginx/restart")}
                />
                <TimelineItem
                    title="Pin the kernel on storage hosts"
                    tone={Tone::Ok}
                    meta={by("Ada Lovelace", at(14, 16, 5))}
                    aside={html! { <><Pill tone={Tone::Ok}>{ "Stable" }</Pill>{ hash("4f2a9c1") }</> }}
                    on_click={open("4f2a9c1")}
                />
                <TimelineItem
                    title="Enable HTTP/3 on the public listeners"
                    tone={Tone::Danger}
                    meta={by("Alan Turing", at(13, 14, 5))}
                    aside={html! { <><Pill tone={Tone::Danger}>{ "Rejected" }</Pill>{ hash("b7d3120") }</> }}
                    on_click={open("b7d3120")}
                />
                <TimelineItem
                    title="Rotate the backup credentials"
                    meta={by("Grace Hopper", at(9, 8, 30))}
                    aside={hash("1e0c5aa")}
                    on_click={open("1e0c5aa")}
                />
            </Timeline>
            <span role="status">
                { opened.map_or("No revision opened".to_string(), |hash| format!("Opened {hash}")) }
            </span>
        </Stack>
    }
}
