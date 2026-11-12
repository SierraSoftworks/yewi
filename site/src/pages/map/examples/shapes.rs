use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let layers = use_memo((), |_| {
        let coverage = [(-10.5, 36.0), (3.5, 36.0), (3.5, 44.0), (-10.5, 44.0)].map(LngLat::from);
        let route = [(-6.26, 53.35), (-0.13, 51.51), (2.35, 48.86), (8.68, 50.11), (18.07, 59.33)].map(LngLat::from);
        let stops = [
            MapMarker::new("dub", (-6.26, 53.35)).label("Dublin"),
            MapMarker::new("arn", (18.07, 59.33)).label("Stockholm"),
        ];

        Rc::<[MapLayer]>::from([
            MapLayer::area("iberia", coverage).tone(Tone::Warn),
            MapLayer::line("backbone", route).tone(Tone::Info),
            MapLayer::markers("stops", stops).tone(Tone::Info),
        ])
    });

    // `bounds` frames the data whatever size the map turns out to be.
    let bounds = MapBounds::new((-11.0, 35.0), (20.0, 60.0));

    html! { <Map {bounds} layers={(*layers).clone()} height="22rem" /> }
}
