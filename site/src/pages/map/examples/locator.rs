use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let layers = use_memo((), |_| {
        Rc::<[MapLayer]>::from([MapLayer::markers("office", [MapMarker::new("cpt", (18.42, -33.92)).label("Cape Town")])])
    });

    html! {
        <Map
            center={LngLat::new(21.0, -32.5)}
            zoom={3.8}
            layers={(*layers).clone()}
            interactive=false
            controls=false
            scale=false
            label="Office location: Cape Town"
            attrs={attrs! { "style" => "--yi-map-height: 14rem; max-inline-size: 24rem" }}
        />
    }
}
