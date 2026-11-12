use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let breadcrumbs = html! {
        <a href="#rollouts">
            <Icon icon={icons::ARROW_LEFT} />
            { " All rollouts" }
        </a>
    };
    let actions = html! {
        <>
            <Button>{ "Pause" }</Button>
            <Button variant={ButtonVariant::Primary}>{ "Approve" }</Button>
        </>
    };

    html! {
        <PageHeader
            title="Rollout 4f2a91c"
            {breadcrumbs}
            meta={html! { <span>{ "Waiting for approval" }</span> }}
            subtitle={html! { "Bump grafana to 12.3 and rotate the loki retention policy." }}
            {actions}
        />
    }
}
