use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let strategy = use_state(|| AttrValue::from("rolling"));
    let on_change = {
        let strategy = strategy.clone();
        Callback::from(move |value| strategy.set(value))
    };

    html! {
        <Stack>
            <Field label="Deploy strategy" hint="Applies to the next rollout." group=true>
                <RadioGroup value={(*strategy).clone()} {on_change}>
                    <Radio value="rolling">{ "Rolling" }</Radio>
                    <Radio value="blue-green">{ "Blue-green" }</Radio>
                    <Radio value="canary">{ "Canary" }</Radio>
                </RadioGroup>
            </Field>
            <span role="status">{ format!("Strategy: {}", *strategy) }</span>
        </Stack>
    }
}
