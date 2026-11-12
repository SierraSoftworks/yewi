use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Text tag={TextTag::P} attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            { "The image " }
            <InlineCode>{ "ghcr.io/sierrasoftworks/caddy-ingress@sha256:3f9a1c2b81e07d09c4e5a" }</InlineCode>
            { " was pulled from the registry and verified." }
        </Text>
    }
}
