use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let protocol = use_state(|| AttrValue::from("https"));
    let host = use_state(|| AttrValue::from("hooks.example.com"));
    let unit = use_state(|| AttrValue::from("min"));

    let protocols: Rc<[(AttrValue, AttrValue)]> =
        Rc::from([("https".into(), "https://".into()), ("http".into(), "http://".into())]);
    let units: Rc<[(AttrValue, AttrValue)]> =
        Rc::from([("sec".into(), "seconds".into()), ("min".into(), "minutes".into()), ("hr".into(), "hours".into())]);

    let set = |state: &UseStateHandle<AttrValue>| {
        let state = state.clone();
        Callback::from(move |value: String| state.set(value.into()))
    };

    let protocol_select = html! {
        <InputGroupSelect label="Protocol" options={protocols} value={(*protocol).clone()} on_change={set(&protocol)} />
    };
    let unit_select = html! {
        <InputGroupSelect label="Unit" options={units} value={(*unit).clone()} on_change={set(&unit)} />
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Webhook URL">
                <InputGroup prefix={protocol_select}>
                    <TextInput value={(*host).clone()} on_input={set(&host)} monospace=true />
                </InputGroup>
            </Field>
            <Field label="Retry after">
                <InputGroup suffix={unit_select}>
                    <NumberInput value={5.0} min={1.0} />
                </InputGroup>
            </Field>
            <span role="status">{ format!("{}://{} · retry in {}", *protocol, *host, *unit) }</span>
        </Stack>
    }
}
