use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster attrs={attrs! { "role" => "toolbar", "aria-label" => "Rollout" }}>
            <Button variant={ButtonVariant::Subtle}>{ "Approve" }</Button>
            <Button variant={ButtonVariant::Subtle}>{ "Pause" }</Button>
            <Divider orientation={DividerOrientation::Vertical} />
            <Button variant={ButtonVariant::Subtle} icon={icons::REFRESH_CW}>{ "Refresh" }</Button>
            <Divider orientation={DividerOrientation::Vertical} />
            <span>{ "2 of 31 hosts done" }</span>
        </Cluster>
    }
}
