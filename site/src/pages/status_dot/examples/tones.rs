use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let states = [
        (Tone::Ok, "healthy"),
        (Tone::Active, "rolling"),
        (Tone::Warn, "held"),
        (Tone::Danger, "failing"),
        (Tone::Info, "scheduled"),
        (Tone::Neutral, "offline"),
    ];

    html! {
        <Cluster gap={Space::Lg}>
            { for states.into_iter().map(|(tone, label)| html! {
                <Cluster gap={Space::Xs} align={Align::Center}>
                    <StatusDot {tone} />
                    { label }
                </Cluster>
            }) }
        </Cluster>
    }
}
