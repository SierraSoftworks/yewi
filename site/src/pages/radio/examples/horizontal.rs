use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let format = use_state(|| AttrValue::from("json"));
    let on_change = {
        let format = format.clone();
        Callback::from(move |value| format.set(value))
    };

    html! {
        <Stack>
            <Field label="Export format" group=true>
                <RadioGroup
                    value={(*format).clone()}
                    {on_change}
                    orientation={RadioOrientation::Horizontal}
                >
                    <Radio value="json">{ "JSON" }</Radio>
                    <Radio value="csv">{ "CSV" }</Radio>
                    <Radio value="yaml">{ "YAML" }</Radio>
                </RadioGroup>
            </Field>
            <span role="status">{ format!("Format: {}", *format) }</span>
        </Stack>
    }
}
