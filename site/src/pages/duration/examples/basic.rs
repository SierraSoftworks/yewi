use yew::prelude::*;
use yewi::chrono::TimeDelta;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let steps = [
        ("Health check", TimeDelta::milliseconds(230)),
        ("Lint", TimeDelta::milliseconds(1_400)),
        ("Unit tests", TimeDelta::seconds(45)),
        ("Build", TimeDelta::seconds(125)),
        ("Integration tests", TimeDelta::minutes(62)),
        ("Soak", TimeDelta::hours(76)),
    ];

    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Pipeline steps" }}>
            { for steps.into_iter().map(|(term, value)| html! {
                <DescriptionItem {term}><Duration {value} /></DescriptionItem>
            }) }
        </DescriptionList>
    }
}
