use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Lg}>
            <Cluster gap={Space::Xs} align={Align::Center}>
                <StatusDot tone={Tone::Active} active=true />
                { "rolling out" }
            </Cluster>
            <Cluster gap={Space::Xs} align={Align::Center}>
                <StatusDot tone={Tone::Danger} active=true />
                { "incident open" }
            </Cluster>
            <Cluster gap={Space::Xs} align={Align::Center}>
                <StatusDot tone={Tone::Ok} />
                { "resolved" }
            </Cluster>
        </Cluster>
    }
}
