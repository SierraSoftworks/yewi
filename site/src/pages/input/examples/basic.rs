use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let name = use_state(|| AttrValue::from("checkout-api"));
    let saved = use_state(|| AttrValue::from("checkout-api"));

    let on_input = {
        let name = name.clone();
        Callback::from(move |value: String| name.set(value.into()))
    };
    let on_change = {
        let saved = saved.clone();
        Callback::from(move |value: String| saved.set(value.into()))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Service name" hint="Lowercase letters, digits and dashes.">
                <TextInput value={(*name).clone()} {on_input} {on_change} placeholder="my-service" />
            </Field>
            <span role="status">{ format!("Typing: {} · Saved: {}", *name, *saved) }</span>
        </Stack>
    }
}
