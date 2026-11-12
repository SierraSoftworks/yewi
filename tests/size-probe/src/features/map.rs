use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    let layers: Rc<[MapLayer]> =
        Rc::from([MapLayer::markers("sites", [MapMarker::new("cpt", (18.42, -33.92)).label("Cape Town")])]);
    let toolbar = html! {
        <MapToolbar>
            <MapToolGroup><MapTool icon={icons::MAXIMIZE} label="Fit everything" /></MapToolGroup>
        </MapToolbar>
    };
    let sidebar = html! { <MapPanel title="On the map" collapsible=true>{ "Cape Town" }</MapPanel> };
    html! { <Map {layers} {toolbar} {sidebar} overlay={html! { <MapNote>{ "Ready" }</MapNote> }} /> }
}
