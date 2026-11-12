use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Card title="edge-01" subtitle="Enrolled 14 months ago" attrs={attrs! { "style" => "max-inline-size: 30rem" }}>
            <Panel
                title="System"
                subtitle="NixOS 25.11"
                actions={html! { <Button size={Size::Small}>{ "Reboot" }</Button> }}
            >
                { "Kernel 6.12, up for 18 days." }
            </Panel>
            <Panel
                title="Workloads"
                subtitle="3 running"
                actions={html! { <Button size={Size::Small}>{ "Restart all" }</Button> }}
            >
                { "grafana, loki and tempo are healthy." }
            </Panel>
            <Panel title="Notes">{ "Rack 4, upper shelf. Shares a power strip with edge-02." }</Panel>
        </Card>
    }
}
