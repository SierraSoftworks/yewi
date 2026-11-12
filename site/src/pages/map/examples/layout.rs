use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Select,
    Pin,
}

const SITES: [(&str, &str, &str, f64, f64, Tone); 4] = [
    ("fra", "Frankfurt", "12 nodes", 8.68, 50.11, Tone::Ok),
    ("dub", "Dublin", "8 nodes", -6.26, 53.35, Tone::Ok),
    ("mad", "Madrid", "5 nodes", -3.70, 40.42, Tone::Warn),
    ("arn", "Stockholm", "3 nodes", 18.07, 59.33, Tone::Danger),
];

#[function_component]
pub fn Example() -> Html {
    let map = use_state(|| None::<MapHandle>);
    let tool = use_state(|| Tool::Select);
    let selected = use_state(|| None::<AttrValue>);

    let layers = use_memo((), |_| {
        let markers = SITES.map(|(id, name, _, lng, lat, tone)| MapMarker::new(id, (lng, lat)).label(name).tone(tone));
        Rc::<[MapLayer]>::from([MapLayer::markers("sites", markers)])
    });

    let on_ready = {
        let map = map.clone();
        Callback::from(move |handle| map.set(Some(handle)))
    };
    let on_pick = {
        let selected = selected.clone();
        Callback::from(move |pick: MapPick| selected.set(pick.hits.first().map(|hit| hit.id.clone())))
    };
    let choose = |next: Tool| {
        let tool = tool.clone();
        Callback::from(move |_| tool.set(next))
    };
    let fit = {
        let map = map.clone();
        Callback::from(move |_| {
            if let Some(map) = &*map {
                map.fit_all();
            }
        })
    };
    let close = {
        let selected = selected.clone();
        Callback::from(move |()| selected.set(None))
    };

    let toolbar = html! {
        <MapToolbar>
            <MapToolGroup>
                <MapTool
                    icon={icons::MOUSE_POINTER}
                    label="Select"
                    pressed={*tool == Tool::Select}
                    on_click={choose(Tool::Select)}
                />
                <MapTool
                    icon={icons::MAP_PIN}
                    label="Place a marker"
                    pressed={*tool == Tool::Pin}
                    on_click={choose(Tool::Pin)}
                />
            </MapToolGroup>
            <MapToolGroup>
                <MapTool icon={icons::MAXIMIZE} label="Fit everything" on_click={fit} />
            </MapToolGroup>
            <MapToolGroup>{ format!("{} sites", SITES.len()) }</MapToolGroup>
        </MapToolbar>
    };

    let sidebar = html! {
        <MapPanel title="On the map" meta={SITES.len().to_string()} collapsible=true>
            <MapList>
                { for SITES.iter().map(|(id, name, detail, lng, lat, tone)| {
                    let (map, selected) = (map.clone(), selected.clone());
                    let (id, at) = (AttrValue::from(*id), LngLat::new(*lng, *lat));
                    let is_selected = selected.as_ref() == Some(&id);
                    let on_click = Callback::from(move |_| {
                        if let Some(map) = &*map {
                            map.fly_to(at, None);
                        }
                        selected.set(Some(id.clone()));
                    });
                    html! {
                        <MapListItem label={*name} detail={*detail} tone={*tone} selected={is_selected} {on_click} />
                    }
                }) }
            </MapList>
        </MapPanel>
    };

    let site = selected.as_ref().and_then(|id| SITES.iter().find(|site| site.0 == id.as_str()));
    let panel = site.map(|(_, name, detail, lng, lat, _)| {
        html! {
            <MapPanel title={*name} on_close={close}>
                <p>{ format!("{detail} at {lat:.2}, {lng:.2}.") }</p>
            </MapPanel>
        }
    });

    let overlay = (*tool == Tool::Pin).then(|| html! { <MapNote>{ "Click the map to place a marker." }</MapNote> });

    html! {
        <Map
            center={LngLat::new(-4.0, 50.0)}
            zoom={3.0}
            layers={(*layers).clone()}
            selected={(*selected).clone()}
            {toolbar}
            {sidebar}
            overlay={overlay.unwrap_or_default()}
            panel={panel.unwrap_or_default()}
            {on_ready}
            {on_pick}
            height="30rem"
        />
    }
}
