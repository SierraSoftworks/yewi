use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let layout = use_state(|| AttrValue::from("list"));
    let align = use_state(|| AttrValue::from("left"));
    let set = |state: &UseStateHandle<AttrValue>| {
        let state = state.clone();
        Callback::from(move |value| state.set(value))
    };

    html! {
        <Stack>
            <Cluster>
                <SegmentedControl label="Layout" value={(*layout).clone()} on_change={set(&layout)}>
                    <Segment value="list" icon={icons::LIST}>{ "List" }</Segment>
                    <Segment value="grid" icon={icons::GRID_2X2}>{ "Grid" }</Segment>
                    <Segment value="columns" icon={icons::COLUMNS}>{ "Columns" }</Segment>
                </SegmentedControl>
                <SegmentedControl label="Alignment" value={(*align).clone()} on_change={set(&align)}>
                    <Segment value="left" icon={icons::ALIGN_LEFT} label="Left" />
                    <Segment value="center" icon={icons::ALIGN_CENTER} label="Centre" />
                    <Segment value="right" icon={icons::ALIGN_RIGHT} label="Right" />
                </SegmentedControl>
            </Cluster>
            <span role="status">{ format!("Layout: {}, aligned {}", *layout, *align) }</span>
        </Stack>
    }
}
