use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Release {
    version: &'static str,
    channel: &'static str,
    hosts: u32,
}

const RELEASES: [Release; 3] = [
    Release { version: "2026.1.3", channel: "stable", hosts: 212 },
    Release { version: "2026.1.4", channel: "candidate", hosts: 18 },
    Release { version: "2026.2.0", channel: "nightly", hosts: 3 },
];

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| Rc::<[Release]>::from(RELEASES));
    let columns = use_columns(|| {
        vec![
            Column::new("version", "Version", |release: &Release| release.version).row_header(),
            Column::new("channel", "Channel", |release: &Release| release.channel),
            Column::new("hosts", "Hosts", |release: &Release| release.hosts.to_string()).numeric(),
        ]
    });

    html! {
        <Card title="Releases" subtitle="What each channel is running" flush=true attrs={attrs! { "style" => "max-inline-size: 32rem" }}>
            <Table<Release> label="Releases" {columns} rows={(*rows).clone()} footer={html! { "3 releases" }} />
        </Card>
    }
}
