use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let token = use_state(AttrValue::default);
    let on_input = {
        let token = token.clone();
        Callback::from(move |value: String| token.set(value.into()))
    };

    let (hint, error) = match token.len() {
        0 => (Some(html! { "Paste the token from your provider's settings page." }), None),
        1..20 => (None, Some(html! { "A token is at least 20 characters." })),
        _ => (Some(html! { "Looks like a token." }), None),
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="API token" {hint} {error} required=true>
                <SecretInput value={(*token).clone()} {on_input} placeholder="tok_..." />
            </Field>
            <span role="status">{ format!("{} characters entered", token.len()) }</span>
        </Stack>
    }
}
