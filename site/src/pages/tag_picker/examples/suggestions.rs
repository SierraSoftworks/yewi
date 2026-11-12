use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let topics = use_state(|| vec![AttrValue::from("rust")]);
    let on_change = {
        let topics = topics.clone();
        Callback::from(move |values: Vec<AttrValue>| topics.set(values))
    };
    let options = use_memo((), |_| {
        ["rust", "yew", "wasm", "css", "accessibility", "testing"]
            .into_iter()
            .map(SelectOption::plain)
            .collect::<SelectOptions>()
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 24rem" }}>
            <Field label="Topics">
                <TagPicker {options} values={(*topics).clone()} {on_change} allow_custom=true placeholder="Add topics" />
            </Field>
            <span role="status">{ format!("Topics: {}", topics.join(", ")) }</span>
        </Stack>
    }
}
