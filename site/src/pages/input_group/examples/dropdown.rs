use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let protocol = use_state(|| AttrValue::from("https"));
    let host = use_state(|| AttrValue::from("hooks.example.com"));
    let unit = use_state(|| AttrValue::from("min"));

    let protocols = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("https", "https://").description("Encrypted. Required in production."),
            SelectOption::new("http", "http://").description("Plain text, for local testing."),
        ])
    });
    let units: Rc<[(AttrValue, AttrValue)]> =
        Rc::from([("sec".into(), "seconds".into()), ("min".into(), "minutes".into()), ("hr".into(), "hours".into())]);

    let set_protocol = {
        let protocol = protocol.clone();
        Callback::from(move |value: AttrValue| protocol.set(value))
    };
    let set_host = {
        let host = host.clone();
        Callback::from(move |value: String| host.set(value.into()))
    };
    let set_unit = {
        let unit = unit.clone();
        Callback::from(move |value: String| unit.set(value.into()))
    };

    let protocol_select = html! {
        <Select label="Protocol" options={protocols} value={(*protocol).clone()} on_change={set_protocol} />
    };
    let unit_select = html! {
        <InputGroupSelect label="Unit" options={units} value={(*unit).clone()} on_change={set_unit} />
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Retry after">
                <InputGroup suffix={unit_select}>
                    <NumberInput value={5.0} min={1.0} stepper=false />
                </InputGroup>
            </Field>
            <Field label="Webhook URL">
                <InputGroup prefix={protocol_select}>
                    <TextInput value={(*host).clone()} on_input={set_host} monospace=true />
                </InputGroup>
            </Field>
            <span role="status">{ format!("{}://{} · retry in {}", *protocol, *host, *unit) }</span>
        </Stack>
    }
}
