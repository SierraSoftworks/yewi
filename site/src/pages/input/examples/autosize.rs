use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let message = use_state(|| AttrValue::from("Deploying the hotfix now."));
    let on_input = {
        let message = message.clone();
        Callback::from(move |value: String| message.set(value.into()))
    };

    html! {
        <Field label="Message" attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <TextArea
                rows=1
                resize={TextAreaResize::Auto}
                value={(*message).clone()}
                {on_input}
                attrs={attrs! { "style" => "--yi-input-max-height: 8rem" }}
            />
        </Field>
    }
}
