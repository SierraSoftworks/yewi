use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Incident {
    title: &'static str,
    opened: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Loading,
    Loaded,
    Refreshing,
    Empty,
}

#[function_component]
pub fn Example() -> Html {
    let phase = use_state(|| Phase::Loading);
    let incidents = use_memo((), |()| {
        Rc::<[Incident]>::from([
            Incident { title: "Elevated 502s on edge-ams", opened: "Today, 09:14" },
            Incident { title: "Backup upload timing out", opened: "Yesterday, 22:40" },
        ])
    });
    let none = use_memo((), |()| Rc::<[Incident]>::from([]));
    let columns = use_columns(|| {
        vec![
            Column::new("title", "Incident", |incident: &Incident| incident.title).row_header().wrap(),
            Column::new("opened", "Opened", |incident: &Incident| incident.opened).width("12rem"),
        ]
    });

    let rows = match *phase {
        Phase::Loaded | Phase::Refreshing => incidents,
        Phase::Loading | Phase::Empty => none,
    };
    let on_change = {
        let phase = phase.clone();
        Callback::from(move |next: AttrValue| {
            phase.set(match next.as_str() {
                "loaded" => Phase::Loaded,
                "refreshing" => Phase::Refreshing,
                "empty" => Phase::Empty,
                _ => Phase::Loading,
            });
        })
    };
    let value = match *phase {
        Phase::Loading => "loading",
        Phase::Loaded => "loaded",
        Phase::Refreshing => "refreshing",
        Phase::Empty => "empty",
    };

    html! {
        <Stack>
            <SegmentedControl label="Table state" {value} {on_change}>
                <Segment value="loading">{ "Loading" }</Segment>
                <Segment value="loaded">{ "Loaded" }</Segment>
                <Segment value="refreshing">{ "Refreshing" }</Segment>
                <Segment value="empty">{ "Empty" }</Segment>
            </SegmentedControl>
            <Table<Incident>
                label="Open incidents"
                {columns}
                rows={(*rows).clone()}
                loading={matches!(*phase, Phase::Loading | Phase::Refreshing)}
                skeleton_rows=3
                empty={html! {
                    <EmptyState title="No open incidents" icon={icons::CHECK_CIRCLE} tone={Tone::Ok} size={Size::Small}>
                        { "Everything has been quiet for 12 days." }
                    </EmptyState>
                }}
            />
        </Stack>
    }
}
