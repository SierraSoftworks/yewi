use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

/// Declared in the order states should sort: problems first.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Offline,
    Degraded,
    Healthy,
}

#[derive(Clone, PartialEq)]
struct Host {
    name: &'static str,
    state: State,
    memory_gb: u32,
    /// Minutes since the agent last reported.
    seen_minutes: u32,
}

const HOSTS: [Host; 5] = [
    Host { name: "web-10", state: State::Healthy, memory_gb: 16, seen_minutes: 1 },
    Host { name: "web-2", state: State::Degraded, memory_gb: 16, seen_minutes: 4 },
    Host { name: "db-1", state: State::Healthy, memory_gb: 64, seen_minutes: 2 },
    Host { name: "Edge-ams", state: State::Offline, memory_gb: 8, seen_minutes: 190 },
    Host { name: "web-1", state: State::Healthy, memory_gb: 32, seen_minutes: 1 },
];

fn state_pill(state: State) -> Html {
    match state {
        State::Healthy => html! { <Pill tone={Tone::Ok}>{ "Healthy" }</Pill> },
        State::Degraded => html! { <Pill tone={Tone::Warn}>{ "Degraded" }</Pill> },
        State::Offline => html! { <Pill tone={Tone::Danger}>{ "Offline" }</Pill> },
    }
}

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| Rc::<[Host]>::from(HOSTS));
    let columns = use_columns(|| {
        vec![
            Column::new("host", "Host", |host: &Host| host.name).row_header().sort_text(|host| host.name),
            Column::new("state", "State", |host: &Host| state_pill(host.state)).sort_by_key(|host| host.state),
            Column::new("memory", "Memory", |host: &Host| format!("{} GB", host.memory_gb))
                .sort_by_key(|host| host.memory_gb)
                .numeric(),
            Column::new("seen", "Last seen", |host: &Host| format!("{} min ago", host.seen_minutes))
                .sort_by_key(|host| host.seen_minutes)
                .numeric(),
        ]
    });

    let sorted_by = use_state(|| String::from("host, ascending"));
    let on_sort = {
        let sorted_by = sorted_by.clone();
        Callback::from(move |sort: Vec<Sort>| {
            sorted_by.set(sort.iter().map(|s| format!("{}, {}", s.column, s.direction.as_str())).collect());
        })
    };

    html! {
        <Stack>
            <Table<Host> label="Hosts" {columns} rows={(*rows).clone()} default_sort={Sort::asc("host")} {on_sort} />
            <span role="status">{ format!("Sorted by {}", *sorted_by) }</span>
        </Stack>
    }
}
