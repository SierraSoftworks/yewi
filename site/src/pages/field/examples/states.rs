use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Project name" required=true>
                <TextInput value="buckle" />
            </Field>
            <Field label="Region" hint="Set by your organisation." disabled=true>
                <TextInput value="eu-west-1" />
            </Field>
        </Stack>
    }
}
