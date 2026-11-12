use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let replicas = use_state(|| Some(3.0));
    let on_input = {
        let replicas = replicas.clone();
        Callback::from(move |value: Option<f64>| replicas.set(value))
    };

    let status = match *replicas {
        Some(count) => format!("Replicas: {count}"),
        None => "Replicas: not set".to_owned(),
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 12rem" }}>
            <Field label="Replicas" hint="Between 1 and 20.">
                <NumberInput value={*replicas} {on_input} min={1.0} max={20.0} />
            </Field>
            <span role="status">{ status }</span>
        </Stack>
    }
}
