use yew::prelude::*;
use yewi::chrono::{DateTime, TimeZone, Utc};
use yewi::prelude::*;

#[derive(Properties, PartialEq)]
struct ColumnProps {
    title: AttrValue,
    compact: bool,
}

#[function_component]
fn Column(props: &ColumnProps) -> Html {
    let moments: [DateTime<Utc>; 5] = [
        Utc.with_ymd_and_hms(2026, 1, 15, 11, 25, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 1, 15, 8, 40, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 1, 13, 14, 5, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 1, 3, 9, 30, 0).unwrap(),
        Utc.with_ymd_and_hms(2025, 11, 28, 9, 30, 0).unwrap(),
    ];

    html! {
        <Stack gap={Space::Xs}>
            <strong>{ props.title.clone() }</strong>
            { for moments.into_iter().map(|at| html! { <Timestamp {at} compact={props.compact} /> }) }
        </Stack>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xxl} align={Align::Start}>
            <Column title="Default" compact=false />
            <Column title="Compact" compact=true />
        </Cluster>
    }
}
