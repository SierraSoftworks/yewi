use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Lg}>
            <Cluster gap={Space::Xs} align={Align::Center}>
                <StatusDot />
                { "body text" }
            </Cluster>
            <span style="color: var(--yi-fg-muted)">
                <Cluster gap={Space::Xs} align={Align::Center}>
                    <StatusDot />
                    { "muted text" }
                </Cluster>
            </span>
            <Pill tone={Tone::Warn} dot=false>
                <StatusDot active=true />
                { "held" }
            </Pill>
        </Cluster>
    }
}
