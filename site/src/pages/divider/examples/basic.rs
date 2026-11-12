use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Lg} attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <span>{ "The rollout reached every host in the staging cluster." }</span>
            <Divider />
            <span>{ "Production is waiting for approval." }</span>
        </Stack>
    }
}
