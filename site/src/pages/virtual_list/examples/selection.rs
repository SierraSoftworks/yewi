use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Host {
    name: String,
    zone: &'static str,
    memory_gb: u32,
}

/// 20,000 deterministic hosts.
fn hosts() -> Rc<[Host]> {
    const ZONES: [&str; 3] = ["ams-1", "fra-2", "iad-1"];
    (0..20_000_u32)
        .map(|i| Host { name: format!("node-{}", i + 1), zone: ZONES[(i % 3) as usize], memory_gb: 8 << (i * 5 % 4) })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| hosts());
    let columns = use_columns(|| {
        vec![
            Column::new("host", "Host", |host: &Host| host.name.clone()).row_header().sort_text(|host| &host.name),
            Column::new("zone", "Zone", |host: &Host| host.zone).sort_text(|host| host.zone).width("8rem"),
            Column::new("memory", "Memory", |host: &Host| format!("{} GB", host.memory_gb))
                .sort_by_key(|host| host.memory_gb)
                .numeric()
                .width("8rem"),
        ]
    });
    let name = use_memo((), |()| RowFn::new(|host: &Host| AttrValue::from(host.name.clone())));

    let selected = use_state(Selection::default);
    let on_select = {
        let selected = selected.clone();
        Callback::from(move |next: Selection| selected.set(next))
    };
    let opened = use_state(|| None::<String>);
    let on_row_click = {
        let opened = opened.clone();
        Callback::from(move |host: Host| opened.set(Some(host.name)))
    };

    html! {
        <Stack>
            <VirtualTable<Host>
                label="Hosts"
                variant={TableVariant::Zebra}
                compact=true
                {columns}
                rows={(*rows).clone()}
                row_key={(*name).clone()}
                row_label={(*name).clone()}
                selected={(*selected).clone()}
                {on_select}
                {on_row_click}
                max_height="20rem"
            />
            <span role="status">
                { format!("{} of {} hosts selected", selected.len(), rows.len()) }
                if let Some(opened) = &*opened {
                    { format!(", opened {opened}") }
                }
            </span>
        </Stack>
    }
}
