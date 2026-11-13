use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let name = use_state(|| AttrValue::from("  checkout api "));

    let on_input = {
        let name = name.clone();
        Callback::from(move |value: String| name.set(value.into()))
    };
    // Tidied once the user has finished, not under their cursor.
    let on_blur_value = {
        let name = name.clone();
        Callback::from(move |value: String| name.set(value.split_whitespace().collect::<Vec<_>>().join(" ").into()))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Display name" hint="Extra spaces are removed when you leave the field.">
                <TextInput value={(*name).clone()} {on_input} {on_blur_value} />
            </Field>
            <span role="status">{ format!("Name: [{}]", *name) }</span>
        </Stack>
    }
}
