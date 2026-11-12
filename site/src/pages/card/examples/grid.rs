use yew::prelude::*;
use yewi::prelude::*;

const CLUSTERS: [(&str, &str, &str); 3] = [
    ("Home", "4 hosts", "All hosts are healthy and on the latest revision."),
    ("Staging", "9 hosts", "One host is rebooting after a kernel update."),
    ("Production", "18 hosts", "Healthy."),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="14rem">
            { for CLUSTERS.into_iter().map(|(name, hosts, health)| html! {
                <Card title={name} subtitle={hosts} level=3>{ health }</Card>
            }) }
        </Grid>
    }
}
