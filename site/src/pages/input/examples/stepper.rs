use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let replicas = use_state(|| Some(3.0));
    let threshold = use_state(|| Some(0.5));

    let set_replicas = {
        let replicas = replicas.clone();
        Callback::from(move |value: Option<f64>| replicas.set(value))
    };
    let set_threshold = {
        let threshold = threshold.clone();
        Callback::from(move |value: Option<f64>| threshold.set(value))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 12rem" }}>
            <Field label="Replicas">
                <NumberInput value={*replicas} on_input={set_replicas} min={1.0} max={5.0} stepper=true />
            </Field>
            <Field label="Error threshold" hint="From 0 to 1, in steps of 0.05.">
                <NumberInput
                    value={*threshold}
                    on_input={set_threshold}
                    min={0.0}
                    max={1.0}
                    step={0.05}
                    stepper=true
                />
            </Field>
            <span role="status">
                { format!("{} replicas, threshold {}", replicas.unwrap_or_default(), threshold.unwrap_or_default()) }
            </span>
        </Stack>
    }
}
