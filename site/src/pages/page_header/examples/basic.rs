use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let actions = html! {
        <>
            <Button icon={icons::REFRESH_CW}>{ "Refresh" }</Button>
            <Button variant={ButtonVariant::Primary} icon={icons::PLUS}>{ "New rollout" }</Button>
        </>
    };

    html! {
        <PageHeader
            title="Rollouts"
            subtitle={html! { "Every change to the fleet, newest first." }}
            {actions}
        />
    }
}
