use std::rc::Rc;

use yewi_test::render;

use super::*;
use crate::Tone;

fn sites() -> Rc<[MapLayer]> {
    Rc::from([MapLayer::markers("sites", [MapMarker::new("fra", (8.68, 50.11)).label("Frankfurt").tone(Tone::Ok)])])
}

#[test]
fn renders_every_slot_on_the_server() {
    let html = render(|| {
        html! {
            <Map
                layers={sites()}
                toolbar={html! { <MapToolbar><MapToolGroup>{ "tools here" }</MapToolGroup></MapToolbar> }}
                sidebar={html! { <MapPanel title="On the map">{ "list here" }</MapPanel> }}
                overlay={html! { <MapNote>{ "note here" }</MapNote> }}
                panel={html! { "details here" }}
                footer={html! { "playback here" }}
            />
        }
    });
    for expected in
        ["tools here", "list here", "note here", "details here", "playback here", r#"role="toolbar""#, "On the map"]
    {
        assert!(html.contains(expected), "missing {expected:?} in {html}");
    }
}

#[test]
fn zoom_buttons_are_labelled_and_optional() {
    let with = render(|| html! { <Map /> });
    assert!(with.contains(r#"aria-label="Zoom in""#) && with.contains(r#"aria-label="Zoom out""#));

    let without = render(|| html! { <Map controls=false /> });
    assert!(!without.contains("Zoom in"));
}

#[test]
fn a_basemap_that_asks_for_credit_gets_it_as_a_link() {
    let osm = render(|| html! { <Map basemap={Basemap::osm()} /> });
    assert!(osm.contains("OpenStreetMap"));
    assert!(osm.contains(r#"href="https://www.openstreetmap.org/copyright""#));

    let outline = render(|| html! { <Map /> });
    assert!(!outline.contains("<a "));
}

#[test]
fn height_and_passthrough_attributes_reach_the_root() {
    let html = render(|| html! { <Map id="fleet" height="40rem" attrs={crate::attrs! { "data-page" => "map" }} /> });
    assert!(html.contains(r#"id="fleet""#));
    assert!(html.contains("--yi-map-height:40rem"));
    assert!(html.contains(r#"data-page="map""#));
}

#[test]
fn options_describe_the_view_and_the_basemap() {
    let options = |props: MapProps| props.options();

    let outline = options(yew::props!(MapProps { center: LngLat::new(18.5, -33.9), zoom: 9.0, label: "Fleet \"A\"" }));
    assert!(outline.contains(r#""center":[18.5,-33.9]"#));
    assert!(outline.contains(r#""zoom":9"#));
    assert!(outline.contains(r#""kind":"outline""#));
    assert!(outline.contains(r#""bounds":null"#));
    assert!(outline.contains(r#""label":"Fleet \"A\"""#));

    let raster = options(yew::props!(MapProps {
        basemap: Basemap::osm().dark("https://tiles.example/dark/{z}/{x}/{y}.png").max_zoom(17),
        bounds: MapBounds::new((-11.0, 35.0), (20.0, 60.0)),
        engine: MapEngine::at("/vendor/maplibre-gl.mjs").stylesheet("/vendor/maplibre-gl.css"),
    }));
    assert!(raster.contains(r#""tiles":["https://tile.openstreetmap.org/{z}/{x}/{y}.png"]"#));
    assert!(raster.contains(r#""darkTiles":["https://tiles.example/dark/{z}/{x}/{y}.png"]"#));
    assert!(raster.contains(r#""maxZoom":17"#));
    assert!(raster.contains(r#""bounds":[-11,35,20,60]"#));
    assert!(raster.contains(r#""script":"/vendor/maplibre-gl.mjs""#));
    assert!(raster.contains(r#""stylesheet":"/vendor/maplibre-gl.css""#));
}

#[test]
fn markers_become_geojson_points() {
    let layer = MapLayer::markers(
        "sites",
        [
            MapMarker::new("fra", (8.68, 50.11)).label("Frankfurt \"FRA\"").tone(Tone::Warn),
            MapMarker::new("nowhere", (f64::NAN, f64::INFINITY)),
        ],
    );
    assert_eq!(layer.kind, MapLayerKind::Markers);
    assert!(layer.interactive);
    assert!(layer.data.contains(r#""coordinates":[8.68,50.11]"#));
    assert!(layer.data.contains(r#""id":"fra""#));
    assert!(layer.data.contains(r#""label":"Frankfurt \"FRA\"""#));
    assert!(layer.data.contains(r#""tone":"warn""#));
    // Not-a-number is not JSON: such a position is written as zero.
    assert!(layer.data.contains(r#""coordinates":[0,0]"#));
}

#[test]
fn lines_and_areas_become_geojson_shapes() {
    let path = [(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)].map(LngLat::from);

    let line = MapLayer::line("route", path).tone(Tone::Info).inert();
    assert_eq!((line.kind, line.tone, line.interactive), (MapLayerKind::Line, Some(Tone::Info), false));
    assert!(line.data.contains(r#""type":"LineString","coordinates":[[0,0],[10,0],[10,10]]"#));
    assert!(line.data.contains(r#""id":"route""#));

    // A polygon ring ends where it began, whether or not the caller closed it.
    let open = MapLayer::area("zone", path);
    let closed = MapLayer::area("zone", path.into_iter().chain([path[0]]));
    assert!(open.data.contains(r#""type":"Polygon","coordinates":[[[0,0],[10,0],[10,10],[0,0]]]"#));
    assert_eq!(open, closed);
}

#[test]
fn bounds_contain_every_position() {
    let bounds = MapBounds::containing([(8.0, 50.0), (-6.0, 53.0), (18.0, 40.0)].map(LngLat::from));
    assert_eq!(bounds, Some(MapBounds::new((-6.0, 40.0), (18.0, 53.0))));
    assert_eq!(MapBounds::containing([]), None);
}

#[test]
fn a_collapsible_panel_folds_natively() {
    let open = render(|| html! { <MapPanel title="On the map" meta="4" collapsible=true>{ "rows" }</MapPanel> });
    assert!(open.starts_with("<details") && open.contains(" open"));
    assert!(open.contains("<summary") && open.contains("On the map") && open.contains("rows"));

    let folded = render(|| html! { <MapPanel title="On the map" collapsible=true open=false /> });
    assert!(!folded.contains(" open"));
}

#[test]
fn a_fixed_panel_is_a_named_section_with_an_optional_close_button() {
    let plain = render(|| html! { <MapPanel title="Frankfurt">{ "details" }</MapPanel> });
    assert!(plain.starts_with("<section") && plain.contains(r#"aria-label="Frankfurt""#));
    assert!(!plain.contains(r#"aria-label="Close""#));

    let closable = render(|| html! { <MapPanel title="Frankfurt" on_close={Callback::noop()} /> });
    assert!(closable.contains(r#"aria-label="Close""#));
}

#[test]
fn notes_are_announced_and_list_rows_mark_the_selection() {
    let note = render(|| html! { <MapNote tone={Tone::Warn}>{ "Feed delayed" }</MapNote> });
    assert!(note.contains(r#"role="status""#) && note.contains("Feed delayed"));

    let rows = render(|| {
        html! {
            <MapList>
                <MapListItem label="Frankfurt" detail="12 nodes" selected=true />
                <MapListItem label="Dublin" />
            </MapList>
        }
    });
    assert_eq!(rows.matches(r#"aria-current="true""#).count(), 1);
    assert!(rows.contains("12 nodes") && rows.contains("Dublin"));
}
