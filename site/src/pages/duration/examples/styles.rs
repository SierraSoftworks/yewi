use yew::prelude::*;
use yewi::chrono::TimeDelta;
use yewi::prelude::*;

#[derive(Properties, PartialEq)]
struct ColumnProps {
    title: AttrValue,
    style: DurationStyle,
}

#[function_component]
fn Column(props: &ColumnProps) -> Html {
    let values = [
        TimeDelta::milliseconds(1_400),
        TimeDelta::seconds(45),
        TimeDelta::seconds(125),
        TimeDelta::minutes(62),
        TimeDelta::hours(76),
    ];

    html! {
        <Stack gap={Space::Xs}>
            <strong>{ props.title.clone() }</strong>
            { for values.into_iter().map(|value| html! { <Duration {value} style={props.style} /> }) }
        </Stack>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xxl} align={Align::Start}>
            <Column title="Short" style={DurationStyle::Short} />
            <Column title="Compact" style={DurationStyle::Compact} />
            <Column title="Long" style={DurationStyle::Long} />
        </Cluster>
    }
}
