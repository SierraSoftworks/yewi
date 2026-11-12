use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let exact = html! {
        <>
            { "Thursday 15 January 2026, 13:00:00" }
            <br />
            <span style="color: var(--yi-fg-muted)">{ "12:00:00 UTC" }</span>
        </>
    };
    let blocked = html! {
        <>
            <strong>{ "Blocked" }</strong>
            { " by a failed health check. " }
            <a href="#runbook">{ "Open the runbook" }</a>
        </>
    };

    html! {
        <Cluster gap={Space::Lg}>
            <Tooltip tip={exact} focusable=true>
                <span>{ "35 minutes ago" }</span>
            </Tooltip>
            <Tooltip tip={blocked} placement={Placement::BottomStart}>
                <Button variant={ButtonVariant::Danger} icon={icons::CIRCLE_ALERT}>{ "Rollout blocked" }</Button>
            </Tooltip>
        </Cluster>
    }
}
