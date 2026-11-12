use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let items = use_memo((), |_| {
        Rc::<[BarItem]>::from(vec![
            BarItem::labelled("/", 4812.0),
            BarItem::labelled("/pricing", 2140.0),
            BarItem::labelled("/docs/getting-started", 1986.0),
            BarItem::labelled("/blog/shipping-faster-with-fewer-dashboards", 913.0),
            BarItem::labelled("/changelog", 402.0),
            BarItem::labelled("/about", 127.0),
        ])
    });

    html! {
        <div style="max-inline-size: 28rem">
            <BarList
                items={(*items).clone()}
                label="Top pages"
                heading="Page"
                value_heading="Visitors"
                show_share=true
            />
        </div>
    }
}
