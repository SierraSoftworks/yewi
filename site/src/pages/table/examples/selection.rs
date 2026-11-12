use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Host {
    name: &'static str,
    agent: &'static str,
}

const HOSTS: [Host; 4] = [
    Host { name: "web-01", agent: "1.4.2" },
    Host { name: "web-02", agent: "1.4.2" },
    Host { name: "db-01", agent: "1.3.9" },
    Host { name: "edge-ams", agent: "1.3.9" },
];

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| Rc::<[Host]>::from(HOSTS));
    let columns = use_columns(|| {
        vec![
            Column::new("host", "Host", |host: &Host| host.name).row_header().sort_text(|host| host.name),
            Column::new("agent", "Agent", |host: &Host| host.agent).sort_text(|host| host.agent),
        ]
    });
    // Rows are known by name, so the selection survives sorting.
    let name = use_memo((), |()| RowFn::new(|host: &Host| AttrValue::from(host.name)));

    let selected = use_state(Selection::default);
    let on_select = {
        let selected = selected.clone();
        Callback::from(move |next: Selection| selected.set(next))
    };
    let clear = {
        let selected = selected.clone();
        Callback::from(move |_| selected.set(Selection::default()))
    };

    html! {
        <Stack>
            <Table<Host>
                label="Hosts"
                {columns}
                rows={(*rows).clone()}
                row_key={(*name).clone()}
                row_label={(*name).clone()}
                selected={(*selected).clone()}
                {on_select}
            />
            <Cluster>
                <Button disabled={selected.is_empty()} on_click={clear}>{ "Clear selection" }</Button>
                <span role="status">{ format!("{} of {} hosts selected", selected.len(), rows.len()) }</span>
            </Cluster>
        </Stack>
    }
}
