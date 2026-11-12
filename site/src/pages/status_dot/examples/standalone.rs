use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let hosts = [("pi-1", Tone::Ok, "Healthy"), ("pi-2", Tone::Ok, "Healthy"), ("pi-5", Tone::Danger, "Failing")];

    html! {
        <Stack gap={Space::Lg}>
            <Stack gap={Space::Xs}>
                { for hosts.into_iter().map(|(host, tone, label)| html! {
                    <Cluster gap={Space::Sm} align={Align::Center}>
                        <StatusDot {tone} {label} />
                        <code>{ host }</code>
                    </Cluster>
                }) }
            </Stack>
            <Cluster gap={Space::Md} align={Align::Center}>
                <StatusDot tone={Tone::Ok} label="Healthy, small" size="6px" />
                <StatusDot tone={Tone::Ok} label="Healthy, default" />
                <StatusDot tone={Tone::Ok} label="Healthy, large" size="12px" />
            </Cluster>
        </Stack>
    }
}
