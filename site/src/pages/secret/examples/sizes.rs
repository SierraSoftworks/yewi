use yew::prelude::*;
use yewi::prelude::*;

const KEY: &str = "sk_live_51NqT8vX2mZ7";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <SecretInput size={Size::Small} value={KEY} attrs={attrs! { "aria-label" => "Small" }} />
            <SecretInput value={KEY} generator=true attrs={attrs! { "aria-label" => "Medium" }} />
            <SecretInput size={Size::Large} value={KEY} attrs={attrs! { "aria-label" => "Large" }} />
            <SecretInput value={KEY} revealed=true mono=true attrs={attrs! { "aria-label" => "Monospace" }} />
        </Stack>
    }
}
