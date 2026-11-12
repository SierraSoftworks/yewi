use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Host {
    name: &'static str,
    healthy: bool,
    platform: &'static str,
    revision: &'static str,
    memory_gb: u32,
}

const HOSTS: [Host; 4] = [
    Host { name: "web-01", healthy: true, platform: "linux · arm64", revision: "8f3c2d1", memory_gb: 16 },
    Host { name: "web-02", healthy: true, platform: "linux · arm64", revision: "8f3c2d1", memory_gb: 16 },
    Host { name: "db-01", healthy: false, platform: "linux · amd64", revision: "41be07a", memory_gb: 64 },
    Host { name: "edge-ams", healthy: true, platform: "freebsd · amd64", revision: "8f3c2d1", memory_gb: 8 },
];

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| Rc::<[Host]>::from(HOSTS));
    let columns = use_columns(|| {
        vec![
            Column::new("host", "Host", |host: &Host| host.name).row_header(),
            Column::new("state", "State", |host: &Host| {
                if host.healthy {
                    html! { <Pill tone={Tone::Ok}>{ "Healthy" }</Pill> }
                } else {
                    html! { <Pill tone={Tone::Danger}>{ "Degraded" }</Pill> }
                }
            }),
            Column::new("platform", "Platform", |host: &Host| host.platform),
            Column::new("revision", "Revision", |host: &Host| html! { <InlineCode>{ host.revision }</InlineCode> }),
            Column::new("memory", "Memory", |host: &Host| format!("{} GB", host.memory_gb)).numeric(),
        ]
    });

    html! { <Table<Host> label="Hosts" {columns} rows={(*rows).clone()} footer={html! { "4 hosts" }} /> }
}
