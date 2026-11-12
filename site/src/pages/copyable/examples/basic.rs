use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let copied = use_state(|| AttrValue::from("Nothing copied yet"));
    let on_copy = {
        let copied = copied.clone();
        Callback::from(move |value: AttrValue| copied.set(format!("Copied {value}").into()))
    };

    html! {
        <Stack>
            <p>
                { "Deployed commit " }
                <Copyable value="4f9a2c1e8b7d6a5f4e3d2c1b0a9f8e7d6c5b4a39" on_copy={on_copy.clone()}>
                    { "4f9a2c1" }
                </Copyable>
                { " to " }
                <Copyable value="edge-fra-02" mono=false {on_copy} />
                { " 35 minutes ago." }
            </p>
            <span role="status">{ (*copied).clone() }</span>
        </Stack>
    }
}
