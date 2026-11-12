use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let hosts = use_state(|| vec![AttrValue::from("api.example.net")]);
    let on_change = {
        let hosts = hosts.clone();
        Callback::from(move |values: Vec<AttrValue>| hosts.set(values))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 24rem" }}>
            <Field label="Allowed hosts" hint="Press Enter or type a comma after each host">
                <TagPicker values={(*hosts).clone()} {on_change} allow_custom=true mono=true placeholder="example.net" />
            </Field>
            <span role="status">{ format!("{} allowed: {}", hosts.len(), hosts.join(" ")) }</span>
        </Stack>
    }
}
