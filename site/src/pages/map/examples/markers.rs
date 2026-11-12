use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let selected = use_state(|| None::<AttrValue>);

    let layers = use_memo((), |_| {
        Rc::<[MapLayer]>::from([MapLayer::markers(
            "sites",
            [
                MapMarker::new("fra", (8.68, 50.11)).label("Frankfurt").tone(Tone::Ok),
                MapMarker::new("dub", (-6.26, 53.35)).label("Dublin").tone(Tone::Ok),
                MapMarker::new("mad", (-3.70, 40.42)).label("Madrid").tone(Tone::Warn),
                MapMarker::new("arn", (18.07, 59.33)).label("Stockholm").tone(Tone::Danger),
                MapMarker::new("mxp", (9.19, 45.46)).label("Milan"),
            ],
        )])
    });

    let on_pick = {
        let selected = selected.clone();
        Callback::from(move |pick: MapPick| selected.set(pick.hits.first().map(|hit| hit.id.clone())))
    };

    html! {
        <Stack>
            <Map
                center={LngLat::new(8.68, 50.11)}
                zoom={3.4}
                layers={(*layers).clone()}
                selected={(*selected).clone()}
                {on_pick}
                height="22rem"
            />
            <span role="status">
                { match &*selected {
                    Some(id) => format!("Selected {id}"),
                    None => "Nothing selected".to_owned(),
                } }
            </span>
        </Stack>
    }
}
