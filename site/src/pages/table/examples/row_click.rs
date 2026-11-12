use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Workload {
    name: &'static str,
    cluster: &'static str,
    replicas: u32,
}

const WORKLOADS: [Workload; 3] = [
    Workload { name: "caddy", cluster: "eu-west-1", replicas: 6 },
    Workload { name: "postgres", cluster: "eu-west-1", replicas: 3 },
    Workload { name: "grafana", cluster: "us-east-2", replicas: 1 },
];

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| Rc::<[Workload]>::from(WORKLOADS));
    let columns = use_columns(|| {
        vec![
            // The link is the accessible way in; the row click is a larger target for the mouse.
            Column::new("name", "Workload", |workload: &Workload| {
                html! { <Link href="#clickable-rows">{ workload.name }</Link> }
            })
            .row_header(),
            Column::new("cluster", "Cluster", |workload: &Workload| workload.cluster),
            Column::new("replicas", "Replicas", |workload: &Workload| workload.replicas).numeric(),
        ]
    });

    let opened = use_state(|| None::<&'static str>);
    let on_row_click = {
        let opened = opened.clone();
        Callback::from(move |workload: Workload| opened.set(Some(workload.name)))
    };

    html! {
        <Stack>
            <Table<Workload> label="Workloads" {columns} rows={(*rows).clone()} {on_row_click} />
            <span role="status">{ opened.map_or("Nothing opened".into(), |name| format!("Opened {name}")) }</span>
        </Stack>
    }
}
