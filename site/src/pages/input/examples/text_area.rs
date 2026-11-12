use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let notes = use_state(|| AttrValue::from("Rolled back after the error rate doubled."));
    let on_input = {
        let notes = notes.clone();
        Callback::from(move |value: String| notes.set(value.into()))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <Field label="Notes" hint={format!("{} of 200 characters", notes.chars().count())}>
                <TextArea value={(*notes).clone()} {on_input} max_length=200 />
            </Field>
            <Field label="Filter expression">
                <TextArea
                    monospace=true
                    rows=4
                    resize={TextAreaResize::None}
                    value={"service == \"checkout-api\"\n  and status >= 500\n  and region in [\"eu-west-1\", \"us-east-1\"]"}
                />
            </Field>
        </Stack>
    }
}
