use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack align={Align::Start}>
            { for [Size::Small, Size::Medium, Size::Large].into_iter().map(|size| html! {
                <Cluster>
                    <SegmentedControl label="Range" value="week" {size}>
                        <Segment value="day">{ "Day" }</Segment>
                        <Segment value="week">{ "Week" }</Segment>
                        <Segment value="month">{ "Month" }</Segment>
                    </SegmentedControl>
                    <Button {size}>{ "Refresh" }</Button>
                </Cluster>
            }) }
        </Stack>
    }
}
