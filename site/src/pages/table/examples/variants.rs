use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Release {
    version: &'static str,
    channel: &'static str,
    hosts: u32,
}

const RELEASES: [Release; 4] = [
    Release { version: "2.8.4", channel: "stable", hosts: 212 },
    Release { version: "2.9.0-rc.2", channel: "candidate", hosts: 18 },
    Release { version: "2.9.0-rc.1", channel: "candidate", hosts: 3 },
    Release { version: "2.7.9", channel: "previous", hosts: 41 },
];

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| Rc::<[Release]>::from(RELEASES));
    let columns = use_columns(|| {
        vec![
            Column::new("version", "Version", |release: &Release| release.version).row_header(),
            Column::new("channel", "Channel", |release: &Release| release.channel),
            Column::new("hosts", "Hosts", |release: &Release| release.hosts).numeric(),
        ]
    });

    html! {
        <Grid min="16rem" gap={Space::Xl}>
            { for [("Open", TableVariant::Open), ("Ruled", TableVariant::Ruled), ("Zebra", TableVariant::Zebra)]
                .into_iter()
                .map(|(name, variant)| html! {
                    <Stack gap={Space::Sm}>
                        <Text color={TextColor::Muted} size={TextSize::Sm}>{ name }</Text>
                        <Table<Release>
                            label={format!("Releases, {name}")}
                            {variant}
                            columns={columns.clone()}
                            rows={(*rows).clone()}
                        />
                    </Stack>
                }) }
        </Grid>
    }
}
