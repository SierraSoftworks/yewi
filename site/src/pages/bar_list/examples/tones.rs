use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let statuses = use_memo((), |_| {
        Rc::<[BarItem]>::from(vec![
            BarItem::labelled("2xx Success", 48_210.0).tone(Tone::Ok),
            BarItem::labelled("3xx Redirect", 5104.0).tone(Tone::Info),
            BarItem::labelled("4xx Client error", 2260.0).tone(Tone::Warn),
            BarItem::labelled("5xx Server error", 318.0).tone(Tone::Danger),
        ])
    });
    let slowest = use_memo((), |_| {
        Rc::<[BarItem]>::from(vec![
            BarItem::labelled("GET /api/v1/search", 1840.0).display("1.84 s").href("#search"),
            BarItem::labelled("POST /api/v1/events", 620.0).display("620 ms").href("#events"),
            BarItem::labelled("GET /api/v1/projects", 95.0).display("95 ms").href("#projects"),
        ])
    });

    html! {
        <Grid min="16rem" gap={Space::Xl}>
            <BarList
                items={(*statuses).clone()}
                label="Responses by status"
                heading="Status"
                value_heading="Responses"
                order={BarListOrder::Given}
            />
            <BarList
                items={(*slowest).clone()}
                label="Slowest endpoints"
                heading="Endpoint"
                value_heading="p95"
                tone={Tone::Warn}
                size={Size::Small}
            />
        </Grid>
    }
}
