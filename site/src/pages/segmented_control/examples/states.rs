use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let range = use_state(|| AttrValue::from("day"));
    let on_change = {
        let range = range.clone();
        Callback::from(move |value| range.set(value))
    };

    html! {
        <Stack align={Align::Start}>
            <SegmentedControl label="Range" value={(*range).clone()} {on_change}>
                <Segment value="day">{ "Day" }</Segment>
                <Segment value="week">{ "Week" }</Segment>
                <Segment value="year" disabled=true>{ "Year" }</Segment>
            </SegmentedControl>
            <SegmentedControl label="Archived range" value="week" disabled=true>
                <Segment value="day">{ "Day" }</Segment>
                <Segment value="week">{ "Week" }</Segment>
                <Segment value="year">{ "Year" }</Segment>
            </SegmentedControl>
            <span role="status">{ format!("Range: {}", *range) }</span>
        </Stack>
    }
}
