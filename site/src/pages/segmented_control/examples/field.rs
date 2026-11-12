use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let density = use_state(|| AttrValue::from("comfortable"));
    let on_change = {
        let density = density.clone();
        Callback::from(move |value| density.set(value))
    };

    html! {
        <Field label="Table density" hint="Applies to every table in this workspace." group=true>
            <SegmentedControl value={(*density).clone()} {on_change}>
                <Segment value="comfortable">{ "Comfortable" }</Segment>
                <Segment value="compact">{ "Compact" }</Segment>
            </SegmentedControl>
        </Field>
    }
}
