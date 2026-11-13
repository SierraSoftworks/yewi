use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let breakdown = use_state(|| AttrValue::from("pages"));
    let on_change = {
        let breakdown = breakdown.clone();
        Callback::from(move |value: AttrValue| breakdown.set(value))
    };

    // The tabs choose the data; the list that shows it is the same.
    let items: Rc<[BarItem]> = match breakdown.as_str() {
        "browsers" => Rc::from(vec![
            BarItem::labelled("Firefox", 1840.0),
            BarItem::labelled("Chrome", 1310.0),
            BarItem::labelled("Safari", 720.0),
        ]),
        _ => Rc::from(vec![
            BarItem::labelled("/", 2210.0),
            BarItem::labelled("/docs", 1105.0),
            BarItem::labelled("/pricing", 555.0),
        ]),
    };
    let panel = html! { <BarList label="Visitors" {items} /> };

    html! {
        <Tabs
            label="Break down by"
            selected={(*breakdown).clone()}
            variant={TabsVariant::Pill}
            size={Size::Small}
            {panel}
            {on_change}
        >
            <Tab value="pages" label="Pages" />
            <Tab value="browsers" label="Browsers" />
        </Tabs>
    }
}
