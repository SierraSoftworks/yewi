use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let view = use_state(|| AttrValue::from("changed"));
    let on_change = {
        let view = view.clone();
        Callback::from(move |value| view.set(value))
    };

    html! {
        <Stack>
            <SegmentedControl label="Files shown" value={(*view).clone()} {on_change}>
                <Segment value="changed">{ "Changed" }</Segment>
                <Segment value="all">{ "All" }</Segment>
            </SegmentedControl>
            <span role="status">
                { if *view == "changed" { "Showing 4 changed files" } else { "Showing all 37 files" } }
            </span>
        </Stack>
    }
}
