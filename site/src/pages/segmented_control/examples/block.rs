use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let diff = use_state(|| AttrValue::from("combined"));
    let on_change = {
        let diff = diff.clone();
        Callback::from(move |value| diff.set(value))
    };

    html! {
        <div style="max-inline-size: 20rem">
            <SegmentedControl label="Diff view" value={(*diff).clone()} {on_change} block=true>
                <Segment value="combined">{ "Combined" }</Segment>
                <Segment value="target">{ "By target" }</Segment>
            </SegmentedControl>
        </div>
    }
}
