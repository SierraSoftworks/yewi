use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

fn browsers() -> Rc<[BarItem]> {
    Rc::from(vec![
        BarItem::labelled("Chrome", 5230.0),
        BarItem::labelled("Safari", 2410.0),
        BarItem::labelled("Firefox", 1180.0),
        BarItem::new("", "Unknown", 96.0).muted(),
    ])
}

fn devices() -> Rc<[BarItem]> {
    Rc::from(vec![
        BarItem::labelled("Desktop", 6105.0).icon(icons::MONITOR),
        BarItem::labelled("Phone", 2530.0).icon(icons::SMARTPHONE),
        BarItem::labelled("Tablet", 281.0).icon(icons::TABLET),
    ])
}

fn countries() -> Rc<[BarItem]> {
    Rc::from(vec![
        BarItem::new("DE", "Germany", 2904.0).glyph("DE"),
        BarItem::new("US", "United States", 2377.0).glyph("US"),
        BarItem::new("IE", "Ireland", 1420.0).glyph("IE"),
        BarItem::new("ZA", "South Africa", 988.0).glyph("ZA"),
        BarItem::new("", "Unknown", 1227.0).muted(),
    ])
}

#[function_component]
pub fn Example() -> Html {
    let (browsers, devices, countries) = (use_memo((), |_| browsers()), use_memo((), |_| devices()), use_memo((), |_| countries()));

    html! {
        <Card attrs={attrs! { "style" => "max-inline-size: 26rem" }}>
            <Tabs label="Audience" variant={TabsVariant::Pill} size={Size::Small}>
                <Tab value="browsers" label="Browsers">
                    <BarList items={(*browsers).clone()} label="Browsers" value_heading="Visitors" show_share=true />
                </Tab>
                <Tab value="devices" label="Devices">
                    <BarList items={(*devices).clone()} label="Devices" value_heading="Visitors" show_share=true />
                </Tab>
                <Tab value="countries" label="Countries">
                    <BarList items={(*countries).clone()} label="Countries" value_heading="Visitors" show_share=true />
                </Tab>
            </Tabs>
        </Card>
    }
}
