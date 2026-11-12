use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

const TRACK: [(f64, f64); 4] = [(-6.26, 53.35), (-0.13, 51.51), (4.90, 52.37), (13.40, 52.52)];

fn vessel(fix: usize) -> MapLayer {
    MapLayer::markers("vessel", [MapMarker::new("v1", TRACK[fix]).label("Aurora").tone(Tone::Active)])
}

#[function_component]
pub fn Example() -> Html {
    let map = use_state(|| None::<MapHandle>);
    let fix = use_state(|| 0_usize);
    let view = use_state(|| None::<MapView>);

    // The layer the page declares. Later fixes go straight to the engine.
    let layers = use_memo((), |_| Rc::<[MapLayer]>::from([vessel(0)]));

    let on_ready = {
        let map = map.clone();
        Callback::from(move |handle| map.set(Some(handle)))
    };
    let on_move = {
        let view = view.clone();
        Callback::from(move |moved: MapView| view.set(Some(moved)))
    };
    let advance = {
        let (map, fix) = (map.clone(), fix.clone());
        Callback::from(move |_| {
            let next = (*fix + 1) % TRACK.len();
            if let Some(map) = &*map {
                map.set_data("vessel", &vessel(next).data);
            }
            fix.set(next);
        })
    };
    let follow = {
        let (map, fix) = (map.clone(), fix.clone());
        Callback::from(move |_| {
            if let Some(map) = &*map {
                map.fly_to(TRACK[*fix], Some(5.0));
            }
        })
    };

    html! {
        <Stack>
            <Cluster>
                <Button on_click={advance}>{ "Next fix" }</Button>
                <Button on_click={follow}>{ "Centre on Aurora" }</Button>
            </Cluster>
            <Map center={LngLat::new(3.0, 52.0)} zoom={3.6} layers={(*layers).clone()} {on_ready} {on_move} height="20rem" />
            <span role="status">
                { format!("Fix {} of {}", *fix + 1, TRACK.len()) }
                { match *view {
                    Some(view) => format!(", centred on {:.1}, {:.1} at zoom {:.1}", view.center.lat, view.center.lng, view.zoom),
                    None => String::new(),
                } }
            </span>
        </Stack>
    }
}
