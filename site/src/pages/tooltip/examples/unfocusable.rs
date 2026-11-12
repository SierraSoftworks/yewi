use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Lg}>
            <Tooltip tip="Only members of the release team can see this host" focusable=true>
                <Icon icon={icons::LOCK} label="Restricted" />
            </Tooltip>
            <Tooltip tip="deploy-eu-west-1-production-canary-7f3a9c" focusable=true>
                <span class="yi-truncate" style="max-inline-size: 9rem">
                    { "deploy-eu-west-1-production-canary-7f3a9c" }
                </span>
            </Tooltip>
            <Tooltip tip="Two approvals are needed and one has been given" focusable=true>
                <Button variant={ButtonVariant::Primary} disabled=true>{ "Approve" }</Button>
            </Tooltip>
        </Cluster>
    }
}
