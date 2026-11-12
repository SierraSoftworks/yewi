use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let head = html! {
        <>
            <Icon icon={icons::CIRCLE_CHECK} attrs={attrs! { "style" => "color: var(--yi-ok)" }} />
            { "Deployed" }
        </>
    };
    let content = html! {
        <>
            { "Version 2.14.0 is serving all traffic in " }
            <strong>{ "eu-west-1" }</strong>
            { ". " }
            <a href="#release">{ "Release notes" }</a>
        </>
    };

    html! {
        <Cluster>
            <Popover {head} {content} foot="Thursday 15 January 2026, 11:42 UTC">
                <Button icon={icons::ROCKET}>{ "Deployment" }</Button>
            </Popover>
            <Popover content="Only members of the release team can change this." placement={Placement::Right}>
                <Button
                    class="yi-button--icon-only"
                    variant={ButtonVariant::Subtle}
                    icon={icons::INFO}
                    attrs={attrs! { "aria-label" => "About permissions" }}
                />
            </Popover>
        </Cluster>
    }
}
