use yew::prelude::*;
use yewi::prelude::*;

const CLIENT_SECRET: &str = "cs_9Zp4mQ2vX7cL1sT8bN5wK3hJ6dF0gR4yU2aE7iO1";
const SIGNING_KEY: &str = "whsec_T8bN5wK3hJ6dF0gR4yU2aE7iO1pM8nB5vC3xZ6lK9jH2qW4e";

#[function_component]
pub fn Example() -> Html {
    let copied = use_state(|| AttrValue::from("Nothing copied yet"));
    let on_copy = |name: &'static str| {
        let copied = copied.clone();
        Callback::from(move |_| copied.set(format!("Copied the {name}").into()))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <SecretValue
                label="Client secret"
                value={CLIENT_SECRET}
                concealed=true
                on_copy={on_copy("client secret")}
            />
            <SecretValue label="Signing key" value={SIGNING_KEY} on_copy={on_copy("signing key")} />
            <span role="status">{ (*copied).clone() }</span>
        </Stack>
    }
}
