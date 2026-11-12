use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let name = use_state(AttrValue::default);
    let port = use_state(|| Some(8080.0));
    let submitted = use_state(|| None::<String>);

    let set_name = {
        let name = name.clone();
        Callback::from(move |value: String| name.set(value.into()))
    };
    let set_port = {
        let port = port.clone();
        Callback::from(move |value: Option<f64>| port.set(value))
    };
    let onsubmit = {
        let (name, port, submitted) = (name.clone(), port.clone(), submitted.clone());
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            submitted.set(Some(format!("Created {} on port {}", *name, port.unwrap_or_default())));
        })
    };

    let missing = submitted.is_some() && name.is_empty();
    let error = missing.then(|| html! { "A service needs a name." });

    html! {
        <form {onsubmit} novalidate=true style="max-inline-size: 22rem">
            <Stack>
                <Field label="Name" required=true {error}>
                    <TextInput name="name" value={(*name).clone()} on_input={set_name} />
                </Field>
                <Field label="Port">
                    <NumberInput name="port" value={*port} on_input={set_port} min={1.0} max={65535.0} stepper=false />
                </Field>
                <Field label="Description" hint="Optional.">
                    <TextArea name="description" />
                </Field>
                <Cluster>
                    <Button kind={ButtonKind::Submit} variant={ButtonVariant::Primary}>{ "Create service" }</Button>
                </Cluster>
                <span role="status">
                    { if missing { "Not created".to_owned() } else { (*submitted).clone().unwrap_or_default() } }
                </span>
            </Stack>
        </form>
    }
}
