use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Card
            title="Latest rollout"
            subtitle="Started 35 minutes ago by the nightly schedule"
            actions={html! { <Button icon={icons::REFRESH_CW}>{ "Refresh" }</Button> }}
            attrs={attrs! { "style" => "max-inline-size: 28rem" }}
        >
            { "2 of 31 hosts are done. The rollout pauses for approval before it reaches the production stage." }
        </Card>
    }
}
