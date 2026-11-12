#![cfg(feature = "map")]
//! An interactive map with floating chrome: a toolbar over the top, a
//! sidebar and a detail panel over the corners, notes and a footer bar.
//!
//! The map itself is [MapLibre GL](https://maplibre.org), bundled with yewi
//! and fetched the first time a map is shown, so enabling the `map` feature
//! adds almost nothing to the wasm binary and nothing to pages without a
//! map. It draws into a shadow root: the server renders the frame and every
//! slot, and the engine fills in the picture once the page is interactive.
//!
//! Yew is not in the hot path. Give the map GeoJSON through
//! [`MapProps::layers`] for data that changes with the page, and push
//! high-rate updates straight to the engine with [`MapHandle::set_data`].

mod chrome;
mod data;

use std::rc::Rc;

use yew::prelude::*;

pub use self::chrome::*;
pub use self::data::*;
use crate::{Attrs, Icon, icons};

#[derive(Properties, PartialEq)]
pub struct MapProps {
    /// Where the map looks first. Changing it later moves the camera.
    #[prop_or(LngLat::new(0.0, 20.0))]
    pub center: LngLat,
    /// The zoom level to start at: 0 is the whole world, 10 a city.
    #[prop_or(1.0)]
    pub zoom: f64,
    /// Fits the view to these bounds instead of `center` and `zoom`.
    #[prop_or_default]
    pub bounds: Option<MapBounds>,
    /// What is drawn underneath the layers.
    #[prop_or_default]
    pub basemap: Basemap,
    /// The data on the map, bottom layer first.
    #[prop_or_default]
    pub layers: Rc<[MapLayer]>,
    /// The `id` of the feature to highlight.
    #[prop_or_default]
    pub selected: Option<AttrValue>,
    /// Whether the map can be panned and zoomed.
    #[prop_or(true)]
    pub interactive: bool,
    /// Shows the zoom buttons.
    #[prop_or(true)]
    pub controls: bool,
    /// Shows the scale bar.
    #[prop_or(true)]
    pub scale: bool,
    /// The accessible name of the map canvas.
    #[prop_or(AttrValue::Static("Map"))]
    pub label: AttrValue,
    /// The block size as a CSS length. Defaults to `--yi-map-height`.
    #[prop_or_default]
    pub height: Option<AttrValue>,
    /// Tools laid over the top edge: usually a [`MapToolbar`].
    #[prop_or_default]
    pub toolbar: Html,
    /// A list laid over the start corner: usually a [`MapPanel`]. It is the
    /// way in for anyone who cannot use a canvas with a pointer.
    #[prop_or_default]
    pub sidebar: Html,
    /// Notes under the toolbar: usually [`MapNote`]s.
    #[prop_or_default]
    pub overlay: Html,
    /// Details of the selection, laid over the end corner.
    #[prop_or_default]
    pub panel: Html,
    /// A bar along the bottom edge, for playback or a legend.
    #[prop_or_default]
    pub footer: Html,
    /// Where the map engine is loaded from.
    #[prop_or_default]
    pub engine: MapEngine,
    /// Fired once the map has been drawn, with the handle that controls it.
    #[prop_or_default]
    pub on_ready: Callback<MapHandle>,
    /// Fired on a click, with the features under it.
    #[prop_or_default]
    pub on_pick: Callback<MapPick>,
    /// Fired when the camera comes to rest.
    #[prop_or_default]
    pub on_move: Callback<MapView>,
    /// Fired when the map could not be started. The message is also shown
    /// over the map.
    #[prop_or_default]
    pub on_error: Callback<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

impl MapProps {
    fn options(&self) -> String {
        let mut out = String::from(r#"{"engine":"#);
        self.engine.write_json(&mut out);
        out.push_str(r#","basemap":"#);
        self.basemap.write_json(&mut out);
        out.push_str(r#","center":["#);
        json_number(&mut out, self.center.lng);
        out.push(',');
        json_number(&mut out, self.center.lat);
        out.push_str(r#"],"zoom":"#);
        json_number(&mut out, self.zoom);
        out.push_str(r#","bounds":"#);
        match self.bounds {
            Some(bounds) => {
                out.push('[');
                for (index, value) in
                    [bounds.south_west.lng, bounds.south_west.lat, bounds.north_east.lng, bounds.north_east.lat]
                        .into_iter()
                        .enumerate()
                {
                    if index > 0 {
                        out.push(',');
                    }
                    json_number(&mut out, value);
                }
                out.push(']');
            }
            None => out.push_str("null"),
        }
        out.push_str(&format!(r#","interactive":{},"scale":{},"label":"#, self.interactive, self.scale));
        json_string(&mut out, &self.label);
        out.push('}');
        out
    }
}

/// An interactive map with slots for the chrome laid over it.
#[function_component]
pub fn Map(props: &MapProps) -> Html {
    let host = use_node_ref();
    let handle = use_state(|| None::<MapHandle>);
    let error = use_state(|| None::<AttrValue>);

    // The engine calls back for as long as it lives; it always reaches the
    // callbacks of the latest render.
    let listeners = use_mut_ref(|| (Callback::noop(), Callback::noop()));
    *listeners.borrow_mut() = (props.on_pick.clone(), props.on_move.clone());

    // What the camera was last told, so that only a change moves it.
    let view = use_mut_ref(|| None::<(LngLat, f64, Option<MapBounds>)>);

    // The engine: started once mounted, restarted when what it was built
    // with changes.
    {
        let (host, handle, error, listeners, view) =
            (host.clone(), handle.clone(), error.clone(), listeners.clone(), view.clone());
        let (on_ready, on_error) = (props.on_ready.clone(), props.on_error.clone());
        let options = props.options();
        let identity =
            (props.engine.clone(), props.basemap.clone(), props.interactive, props.scale, props.label.clone());
        use_effect_with(identity, move |_| {
            *view.borrow_mut() = None;
            let running = engine::start(
                &host,
                &options,
                listeners,
                Callback::from({
                    let handle = handle.clone();
                    move |started: Result<MapHandle, AttrValue>| match started {
                        Ok(started) => {
                            handle.set(Some(started.clone()));
                            on_ready.emit(started);
                        }
                        Err(message) => {
                            error.set(Some(message.clone()));
                            on_error.emit(message);
                        }
                    }
                }),
            );
            move || {
                running.stop();
                handle.set(None);
            }
        });
    }

    // Layers and selection follow the props.
    {
        let host = host.clone();
        use_effect_with(((*handle).clone(), props.layers.clone()), move |(handle, layers)| {
            if let Some(handle) = handle {
                handle.set_layers(layers);
                handle.settle(&host);
            }
        });
    }
    use_effect_with(((*handle).clone(), props.selected.clone()), |(handle, selected)| {
        if let Some(handle) = handle {
            handle.select(selected.as_deref());
        }
    });
    use_effect_with(((*handle).clone(), props.center, props.zoom, props.bounds), move |(handle, center, zoom, bounds)| {
        let Some(handle) = handle else { return };
        let next = (*center, *zoom, *bounds);
        let previous = view.borrow_mut().replace(next);
        match (previous, bounds) {
            (None, _) => {}
            (Some(previous), _) if previous == next => {}
            (Some(_), Some(bounds)) => handle.fit(*bounds),
            (Some(_), None) => handle.fly_to(*center, Some(*zoom)),
        }
    });

    let zoom_by = |delta: f64| {
        let handle = (*handle).clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(handle) = &handle {
                handle.zoom_by(delta);
            }
        })
    };

    let style = props.height.as_ref().map(|height| format!("--yi-map-height:{height}"));

    props.attrs.apply(html! {
        <div id={props.id.clone()} class={classes!("yi-map", props.class.clone())} {style}>
            <div ref={host} class="yi-map__canvas" />
            <div class="yi-map__overlay">
                <div class="yi-map__toolbar">{ props.toolbar.clone() }</div>
                <div class="yi-map__notes">
                    if let Some(message) = &*error {
                        <p class="yi-map__error" role="alert">{ message.clone() }</p>
                    }
                    { props.overlay.clone() }
                </div>
                <div class="yi-map__sidebar">{ props.sidebar.clone() }</div>
                <div class="yi-map__panel">{ props.panel.clone() }</div>
                <div class="yi-map__footer">{ props.footer.clone() }</div>
                <div class="yi-map__controls">
                    if props.controls {
                        <div class="yi-map__zoom" role="group" aria-label="Zoom">
                            <button type="button" class="yi-map__zoom-button" aria-label="Zoom in" onclick={zoom_by(1.0)}>
                                <Icon icon={icons::PLUS} />
                            </button>
                            <button type="button" class="yi-map__zoom-button" aria-label="Zoom out" onclick={zoom_by(-1.0)}>
                                <Icon icon={icons::MINUS} />
                            </button>
                        </div>
                    }
                </div>
            </div>
            if let Some(credit) = props.basemap.credit() {
                <div class="yi-map__attribution">
                    if let Some(href) = &credit.href {
                        <a href={href.clone()} target="_blank" rel="noopener">{ credit.text.clone() }</a>
                    } else {
                        { credit.text.clone() }
                    }
                </div>
            }
        </div>
    })
}

/// Controls a map that has been drawn. Handed out by [`MapProps::on_ready`];
/// cheap to clone and keep in state.
///
/// Off the web every method does nothing.
#[derive(Clone)]
pub struct MapHandle(Rc<engine::Inner>);

impl PartialEq for MapHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl std::fmt::Debug for MapHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MapHandle")
    }
}

impl MapHandle {
    /// Moves the camera to `center`, keeping the zoom unless one is given.
    pub fn fly_to(&self, center: impl Into<LngLat>, zoom: Option<f64>) {
        self.0.fly_to(center.into(), zoom);
    }

    /// Fits `bounds` into view.
    pub fn fit(&self, bounds: MapBounds) {
        self.0.fit(bounds);
    }

    /// Fits everything the layers hold into view.
    pub fn fit_all(&self) {
        self.0.fit_all();
    }

    /// Zooms in (positive) or out (negative) by `levels`.
    pub fn zoom_by(&self, levels: f64) {
        self.0.zoom_by(levels);
    }

    /// Replaces one layer's GeoJSON without re-rendering the component: the
    /// path for feeds that update many times a second. The next change to
    /// `layers` takes over again.
    pub fn set_data(&self, layer: &str, geojson: &str) {
        self.0.set_data(layer, geojson);
    }

    /// Sets the cursor over the map to a CSS cursor name, for a tool that is
    /// not selection. Empty restores the default.
    pub fn set_cursor(&self, cursor: &str) {
        self.0.set_cursor(cursor);
    }

    /// The MapLibre `Map` object, for anything this component does not
    /// cover. Layers and sources prefixed `yi:` belong to yewi.
    #[cfg(target_arch = "wasm32")]
    pub fn engine(&self) -> wasm_bindgen::JsValue {
        self.0.engine()
    }

    fn set_layers(&self, layers: &[MapLayer]) {
        self.0.set_layers(layers);
    }

    fn select(&self, id: Option<&str>) {
        self.0.select(id);
    }

    fn settle(&self, host: &NodeRef) {
        self.0.settle(host);
    }
}

#[cfg(target_arch = "wasm32")]
mod engine {
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    use js_sys::Array;
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlElement;
    use yew::prelude::*;

    use super::{LngLat, MapBounds, MapHandle, MapHit, MapLayer, MapPick, MapView};

    #[wasm_bindgen(module = "/src/components/map/glue.js")]
    extern "C" {
        type Engine;

        fn create(
            host: &HtmlElement,
            options: &str,
            on_pick: &Closure<dyn Fn(f64, f64, Array)>,
            on_move: &Closure<dyn Fn(f64, f64, f64)>,
            done: JsValue,
        );

        #[wasm_bindgen(js_name = toolbarKey)]
        pub fn toolbar_key(event: &JsValue);

        #[wasm_bindgen(method, js_name = setLayers)]
        fn set_layers(this: &Engine, ids: Array, kinds: Array, tones: Array, interactive: Array, data: Array);
        #[wasm_bindgen(method, js_name = setData)]
        fn set_data(this: &Engine, layer: &str, data: &str);
        #[wasm_bindgen(method, js_name = setSelected)]
        fn set_selected(this: &Engine, id: Option<&str>);
        #[wasm_bindgen(method, js_name = setCursor)]
        fn set_cursor(this: &Engine, cursor: &str);
        #[wasm_bindgen(method, js_name = flyTo)]
        fn fly_to(this: &Engine, lng: f64, lat: f64, zoom: f64);
        #[wasm_bindgen(method, js_name = fitBounds)]
        fn fit_bounds(this: &Engine, west: f64, south: f64, east: f64, north: f64);
        #[wasm_bindgen(method, js_name = fitAll)]
        fn fit_all(this: &Engine);
        #[wasm_bindgen(method, js_name = zoomBy)]
        fn zoom_by(this: &Engine, delta: f64);
        #[wasm_bindgen(method)]
        fn settle(this: &Engine, host: &HtmlElement);
        #[wasm_bindgen(method)]
        fn engine(this: &Engine) -> JsValue;
        #[wasm_bindgen(method)]
        fn destroy(this: &Engine);
    }

    // The engine, its worker, its stylesheet and the outline basemap are
    // fetched by `glue.js` when a map is first shown. Naming them here is what
    // has them copied beside it; nothing is imported from them eagerly.
    #[wasm_bindgen(module = "/src/components/map/vendor/maplibre-gl.js")]
    extern "C" {
        type BundledEngine;
    }
    #[wasm_bindgen(module = "/src/components/map/vendor/maplibre-gl-worker.js")]
    extern "C" {
        type BundledWorker;
    }
    #[wasm_bindgen(module = "/src/components/map/vendor/maplibre-gl-css.js")]
    extern "C" {
        type BundledStylesheet;
    }
    #[wasm_bindgen(module = "/src/components/map/vendor/world-110m.js")]
    extern "C" {
        type BundledOutline;
    }

    type Listeners = Rc<RefCell<(Callback<MapPick>, Callback<MapView>)>>;

    /// What the engine calls. Held for as long as it exists.
    struct Closures {
        on_pick: Closure<dyn Fn(f64, f64, Array)>,
        on_move: Closure<dyn Fn(f64, f64, f64)>,
    }

    pub struct Inner {
        engine: Engine,
        _closures: Rc<Closures>,
    }

    impl Drop for Inner {
        fn drop(&mut self) {
            self.engine.destroy();
        }
    }

    impl Inner {
        pub fn fly_to(&self, center: LngLat, zoom: Option<f64>) {
            self.engine.fly_to(center.lng, center.lat, zoom.unwrap_or(f64::NAN));
        }

        pub fn fit(&self, bounds: MapBounds) {
            let (sw, ne) = (bounds.south_west, bounds.north_east);
            self.engine.fit_bounds(sw.lng, sw.lat, ne.lng, ne.lat);
        }

        pub fn fit_all(&self) {
            self.engine.fit_all();
        }

        pub fn zoom_by(&self, delta: f64) {
            self.engine.zoom_by(delta);
        }

        pub fn set_data(&self, layer: &str, data: &str) {
            self.engine.set_data(layer, data);
        }

        pub fn set_cursor(&self, cursor: &str) {
            self.engine.set_cursor(cursor);
        }

        pub fn engine(&self) -> JsValue {
            self.engine.engine()
        }

        pub fn set_layers(&self, layers: &[MapLayer]) {
            let strings = |f: &dyn Fn(&MapLayer) -> &str| -> Array {
                layers.iter().map(|layer| JsValue::from_str(f(layer))).collect()
            };
            self.engine.set_layers(
                strings(&|layer| &layer.id),
                strings(&|layer| layer.kind.as_str()),
                strings(&|layer| layer.tone.map_or("", |tone| tone.as_str())),
                layers.iter().map(|layer| JsValue::from_bool(layer.interactive)).collect(),
                strings(&|layer| &layer.data),
            );
        }

        pub fn select(&self, id: Option<&str>) {
            self.engine.set_selected(id);
        }

        pub fn settle(&self, host: &NodeRef) {
            if let Some(host) = host.cast::<HtmlElement>() {
                self.engine.settle(&host);
            }
        }
    }

    /// A map that is starting or running.
    pub struct Running {
        alive: Rc<Cell<bool>>,
        started: Rc<RefCell<Option<MapHandle>>>,
    }

    impl Running {
        pub fn stop(self) {
            self.alive.set(false);
            // Dropping the last handle takes the map away; one the app still
            // holds keeps it until the app lets go.
            self.started.borrow_mut().take();
        }
    }

    pub fn start(
        host: &NodeRef,
        options: &str,
        listeners: Listeners,
        done: Callback<Result<MapHandle, AttrValue>>,
    ) -> Running {
        let running = Running { alive: Rc::new(Cell::new(true)), started: Rc::default() };
        let Some(host) = host.cast::<HtmlElement>() else { return running };

        // Until the first picture is drawn the map is busy; `settle` clears
        // both marks, which leaves the markup as the server rendered it.
        let _ = host.set_attribute("aria-busy", "true");
        let _ = host.set_attribute("data-yi-pending", "");

        let on_pick = Closure::<dyn Fn(f64, f64, Array)>::new({
            let listeners = listeners.clone();
            move |lng, lat, hits: Array| {
                let hits: Vec<AttrValue> = hits.iter().filter_map(|hit| hit.as_string()).map(AttrValue::from).collect();
                let hits =
                    hits.as_chunks::<2>().0.iter().map(|[layer, id]| MapHit { layer: layer.clone(), id: id.clone() }).collect();
                let on_pick = listeners.borrow().0.clone();
                on_pick.emit(MapPick { at: LngLat::new(lng, lat), hits });
            }
        });
        let on_move = Closure::<dyn Fn(f64, f64, f64)>::new(move |lng, lat, zoom| {
            let on_move = listeners.borrow().1.clone();
            on_move.emit(MapView { center: LngLat::new(lng, lat), zoom });
        });
        let closures = Rc::new(Closures { on_pick, on_move });

        // Called exactly once, with the engine or with why there is none.
        let finished = Closure::once_into_js({
            let (alive, started, host, closures) =
                (running.alive.clone(), running.started.clone(), host.clone(), closures.clone());
            move |engine: JsValue, message: Option<String>| {
                if engine.is_null() || engine.is_undefined() {
                    let _ = host.remove_attribute("aria-busy");
                    let _ = host.remove_attribute("data-yi-pending");
                    if alive.get() {
                        done.emit(Err(message.unwrap_or_else(|| "The map could not be started.".into()).into()));
                    }
                    return;
                }
                // Not kept when the component has gone: dropping it removes the map.
                let handle = MapHandle(Rc::new(Inner { engine: engine.unchecked_into(), _closures: closures }));
                if alive.get() {
                    *started.borrow_mut() = Some(handle.clone());
                    done.emit(Ok(handle));
                }
            }
        });
        create(&host, options, &closures.on_pick, &closures.on_move, finished);
        running
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod engine {
    use std::{cell::RefCell, rc::Rc};

    use yew::prelude::*;

    use super::{LngLat, MapBounds, MapHandle, MapLayer, MapPick, MapView};

    pub struct Inner;

    impl Inner {
        pub fn fly_to(&self, _: LngLat, _: Option<f64>) {}
        pub fn fit(&self, _: MapBounds) {}
        pub fn fit_all(&self) {}
        pub fn zoom_by(&self, _: f64) {}
        pub fn set_data(&self, _: &str, _: &str) {}
        pub fn set_cursor(&self, _: &str) {}
        pub fn set_layers(&self, _: &[MapLayer]) {}
        pub fn select(&self, _: Option<&str>) {}
        pub fn settle(&self, _: &NodeRef) {}
    }

    pub struct Running;

    impl Running {
        pub fn stop(self) {}
    }

    pub fn start(
        _: &NodeRef,
        _: &str,
        _: Rc<RefCell<(Callback<MapPick>, Callback<MapView>)>>,
        _: Callback<Result<MapHandle, AttrValue>>,
    ) -> Running {
        Running
    }
}

#[cfg(test)]
mod tests;
