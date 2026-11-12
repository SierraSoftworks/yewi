use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let subdomain = use_state(|| AttrValue::from("checkout"));
    let timeout = use_state(|| Some(250.0));

    let set_subdomain = {
        let subdomain = subdomain.clone();
        Callback::from(move |value: String| subdomain.set(value.into()))
    };
    let set_timeout = {
        let timeout = timeout.clone();
        Callback::from(move |value: Option<f64>| timeout.set(value))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Address" hint="A subdomain of example.com.">
                <InputGroup prefix="https://" suffix=".example.com">
                    <TextInput value={(*subdomain).clone()} on_input={set_subdomain} monospace=true />
                </InputGroup>
            </Field>
            <Field label="Timeout" hint="In milliseconds.">
                <InputGroup suffix="ms">
                    <NumberInput value={*timeout} on_input={set_timeout} min={0.0} step={50.0} />
                </InputGroup>
            </Field>
            <Field label="Monthly budget" hint="In US dollars.">
                <InputGroup prefix="$" suffix="USD">
                    <NumberInput value={1200.0} min={0.0} />
                </InputGroup>
            </Field>
            <span role="status">
                { format!("https://{}.example.com, timeout {} ms", *subdomain, timeout.unwrap_or_default()) }
            </span>
        </Stack>
    }
}
