use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let secret = use_state(AttrValue::default);
    let saved = use_state(|| 0usize);

    let on_input = {
        let secret = secret.clone();
        Callback::from(move |value: String| secret.set(value.into()))
    };
    let on_change = {
        let saved = saved.clone();
        Callback::from(move |value: String| saved.set(value.len()))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 30rem" }}>
            <Field label="Webhook secret" hint="Enter the same value in the provider's webhook settings.">
                <SecretInput
                    value={(*secret).clone()}
                    {on_input}
                    {on_change}
                    generator=true
                    monospace=true
                    placeholder="A long random string"
                />
            </Field>
            <span role="status">{ format!("Saved a secret of {} characters", *saved) }</span>
        </Stack>
    }
}
