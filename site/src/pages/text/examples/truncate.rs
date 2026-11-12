use yew::prelude::*;
use yewi::prelude::*;

const MESSAGE: &str = "fix(ingress): retry the upstream health probe three times before marking a host \
                       unhealthy, so a single dropped packet no longer pauses the whole rollout";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Text tag={TextTag::Div} truncate=true attrs={attrs! { "title" => MESSAGE }}>{ MESSAGE }</Text>
            <Text tag={TextTag::P} lines={2} color={TextColor::Muted}>{ MESSAGE }</Text>
            <Text tag={TextTag::P} lines={3} color={TextColor::Muted}>{ MESSAGE }</Text>
        </Stack>
    }
}
