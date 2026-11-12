use yew::prelude::*;
use yewi::prelude::*;

const TOKEN: &str = "rtk_live_9Zp4mQ2vX7cL1sT8bN5wK3hJ6dF0gR4yU2aE7iO1pM8nB5vC3xZ6lK9jH";

#[function_component]
pub fn Example() -> Html {
    let token = use_state(|| None::<AttrValue>);

    let create = {
        let token = token.clone();
        Callback::from(move |_| token.set(Some(TOKEN.into())))
    };
    let on_dismiss = {
        let token = token.clone();
        Callback::from(move |()| token.set(None))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 36rem" }}>
            if let Some(token) = &*token {
                <SecretReveal title="Token created" {on_dismiss}>
                    <SecretValue label="Token" value={token.clone()} />
                </SecretReveal>
            } else {
                <Cluster>
                    <Button variant={ButtonVariant::Primary} on_click={create}>{ "Create token" }</Button>
                </Cluster>
            }
            <span role="status">
                { if token.is_some() { "Token is on screen" } else { "No token on screen" } }
            </span>
        </Stack>
    }
}
