//! What a map is given: where to look, what to draw underneath, and the
//! layers on top. Everything here is plain data that also builds off the web.

use std::fmt::Write as _;

use yew::AttrValue;

use crate::Tone;

/// A position in degrees, longitude first as in GeoJSON.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LngLat {
    pub lng: f64,
    pub lat: f64,
}

impl LngLat {
    pub const fn new(lng: f64, lat: f64) -> Self {
        Self { lng, lat }
    }
}

impl From<(f64, f64)> for LngLat {
    fn from((lng, lat): (f64, f64)) -> Self {
        Self { lng, lat }
    }
}

impl From<[f64; 2]> for LngLat {
    fn from([lng, lat]: [f64; 2]) -> Self {
        Self { lng, lat }
    }
}

/// A rectangle on the map, for fitting the view to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapBounds {
    pub south_west: LngLat,
    pub north_east: LngLat,
}

impl MapBounds {
    pub fn new(south_west: impl Into<LngLat>, north_east: impl Into<LngLat>) -> Self {
        Self { south_west: south_west.into(), north_east: north_east.into() }
    }

    /// The smallest bounds holding every position, or `None` for no positions.
    pub fn containing(positions: impl IntoIterator<Item = LngLat>) -> Option<Self> {
        positions.into_iter().fold(None, |bounds: Option<Self>, at| {
            Some(match bounds {
                None => Self { south_west: at, north_east: at },
                Some(b) => Self {
                    south_west: LngLat::new(b.south_west.lng.min(at.lng), b.south_west.lat.min(at.lat)),
                    north_east: LngLat::new(b.north_east.lng.max(at.lng), b.north_east.lat.max(at.lat)),
                },
            })
        })
    }
}

/// Where the camera is: reported by `on_move`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapView {
    pub center: LngLat,
    pub zoom: f64,
}

/// The credit a basemap asks for, shown in the corner of the map.
#[derive(Debug, Clone, PartialEq)]
pub struct MapAttribution {
    pub text: AttrValue,
    pub href: Option<AttrValue>,
}

/// What is drawn underneath the layers.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Basemap {
    /// A world outline bundled with yewi and drawn in theme colours. Needs no
    /// network and no tile server; coarse beyond country level.
    #[default]
    Outline,
    /// Raster tiles from a tile server. Build one with [`Basemap::raster`].
    Raster(RasterBasemap),
    /// A MapLibre style document, for vector tiles. Build one with
    /// [`Basemap::style`].
    Style(StyleBasemap),
}

/// Raster tiles: `https://host/{z}/{x}/{y}.png`.
#[derive(Debug, Clone, PartialEq)]
pub struct RasterBasemap {
    pub tiles: AttrValue,
    /// Tiles drawn for a dark page. Without them a dark page dims the light
    /// ones, unless `dim` is off.
    pub dark_tiles: Option<AttrValue>,
    pub attribution: Option<MapAttribution>,
    /// Tile edge in pixels: 256, or 512 for high-resolution tiles.
    pub tile_size: u16,
    /// The deepest zoom the server has tiles for.
    pub max_zoom: u8,
    /// Dim and desaturate the tiles on a dark page.
    pub dim: bool,
}

/// A MapLibre style document by URL.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleBasemap {
    pub url: AttrValue,
    /// The style to use on a dark page.
    pub dark_url: Option<AttrValue>,
    pub attribution: Option<MapAttribution>,
}

impl Basemap {
    /// Raster tiles from a `{z}/{x}/{y}` URL template.
    pub fn raster(tiles: impl Into<AttrValue>) -> Self {
        Basemap::Raster(RasterBasemap {
            tiles: tiles.into(),
            dark_tiles: None,
            attribution: None,
            tile_size: 256,
            max_zoom: 19,
            dim: true,
        })
    }

    /// OpenStreetMap's own tile server, with the credit it requires. Fine for
    /// light use; read its tile usage policy before shipping anything busy.
    pub fn osm() -> Self {
        Self::raster("https://tile.openstreetmap.org/{z}/{x}/{y}.png")
            .attribution("© OpenStreetMap contributors", Some("https://www.openstreetmap.org/copyright"))
    }

    /// A MapLibre style document by URL.
    pub fn style(url: impl Into<AttrValue>) -> Self {
        Basemap::Style(StyleBasemap { url: url.into(), dark_url: None, attribution: None })
    }

    /// Sets the credit shown in the corner. No effect on `Outline`.
    pub fn attribution(mut self, text: impl Into<AttrValue>, href: Option<&'static str>) -> Self {
        let credit = Some(MapAttribution { text: text.into(), href: href.map(AttrValue::from) });
        match &mut self {
            Basemap::Outline => {}
            Basemap::Raster(raster) => raster.attribution = credit,
            Basemap::Style(style) => style.attribution = credit,
        }
        self
    }

    /// Sets what a dark page draws instead: a tile template for raster
    /// basemaps, a style URL for style basemaps. No effect on `Outline`,
    /// which already follows the theme.
    pub fn dark(mut self, url: impl Into<AttrValue>) -> Self {
        match &mut self {
            Basemap::Outline => {}
            Basemap::Raster(raster) => raster.dark_tiles = Some(url.into()),
            Basemap::Style(style) => style.dark_url = Some(url.into()),
        }
        self
    }

    /// Leaves raster tiles as they are on a dark page instead of dimming them.
    pub fn undimmed(mut self) -> Self {
        if let Basemap::Raster(raster) = &mut self {
            raster.dim = false;
        }
        self
    }

    /// Sets the deepest zoom a raster tile server has tiles for.
    pub fn max_zoom(mut self, zoom: u8) -> Self {
        if let Basemap::Raster(raster) = &mut self {
            raster.max_zoom = zoom;
        }
        self
    }

    /// The credit to show, if the basemap asks for one.
    pub fn credit(&self) -> Option<&MapAttribution> {
        match self {
            Basemap::Outline => None,
            Basemap::Raster(raster) => raster.attribution.as_ref(),
            Basemap::Style(style) => style.attribution.as_ref(),
        }
    }

    pub(super) fn write_json(&self, out: &mut String) {
        match self {
            Basemap::Outline => out.push_str(r#"{"kind":"outline"}"#),
            Basemap::Raster(raster) => {
                out.push_str(r#"{"kind":"raster","tiles":["#);
                json_string(out, &raster.tiles);
                out.push_str(r#"],"darkTiles":"#);
                match &raster.dark_tiles {
                    Some(tiles) => {
                        out.push('[');
                        json_string(out, tiles);
                        out.push(']');
                    }
                    None => out.push_str("null"),
                }
                let _ = write!(
                    out,
                    r#","tileSize":{},"maxZoom":{},"dim":{}}}"#,
                    raster.tile_size, raster.max_zoom, raster.dim
                );
            }
            Basemap::Style(style) => {
                out.push_str(r#"{"kind":"style","url":"#);
                json_string(out, &style.url);
                out.push_str(r#","darkUrl":"#);
                json_option(out, style.dark_url.as_deref());
                out.push('}');
            }
        }
    }
}

/// Where the map engine is loaded from.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MapEngine {
    /// The MapLibre GL build bundled with yewi, served beside the app's wasm
    /// and fetched the first time a map is shown.
    #[default]
    Bundled,
    /// A MapLibre GL build the app serves itself. Build with [`MapEngine::at`].
    Custom(CustomMapEngine),
}

/// URLs of a self-served MapLibre GL build (version 5 or later, ES module).
#[derive(Debug, Clone, PartialEq)]
pub struct CustomMapEngine {
    /// `maplibre-gl.mjs`.
    pub script: AttrValue,
    /// `maplibre-gl-worker.mjs`, when it is not beside the script.
    pub worker: Option<AttrValue>,
    /// `maplibre-gl.css`. The bundled stylesheet is used when unset.
    pub stylesheet: Option<AttrValue>,
}

impl MapEngine {
    /// A MapLibre GL ES module the app serves itself.
    pub fn at(script: impl Into<AttrValue>) -> Self {
        MapEngine::Custom(CustomMapEngine { script: script.into(), worker: None, stylesheet: None })
    }

    pub fn worker(mut self, url: impl Into<AttrValue>) -> Self {
        if let MapEngine::Custom(engine) = &mut self {
            engine.worker = Some(url.into());
        }
        self
    }

    pub fn stylesheet(mut self, url: impl Into<AttrValue>) -> Self {
        if let MapEngine::Custom(engine) = &mut self {
            engine.stylesheet = Some(url.into());
        }
        self
    }

    pub(super) fn write_json(&self, out: &mut String) {
        match self {
            MapEngine::Bundled => out.push_str("null"),
            MapEngine::Custom(engine) => {
                out.push_str(r#"{"script":"#);
                json_string(out, &engine.script);
                out.push_str(r#","worker":"#);
                json_option(out, engine.worker.as_deref());
                out.push_str(r#","stylesheet":"#);
                json_option(out, engine.stylesheet.as_deref());
                out.push('}');
            }
        }
    }
}

/// How a layer's features are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum MapLayerKind {
    /// Points as dots, with their `label` underneath.
    #[default]
    Markers,
    /// Lines, and the outlines of polygons.
    Line,
    /// Polygons, filled and outlined.
    Area,
}

impl MapLayerKind {
    pub const ALL: [MapLayerKind; 3] = [MapLayerKind::Markers, MapLayerKind::Line, MapLayerKind::Area];

    /// The name the engine knows it by.
    pub const fn as_str(self) -> &'static str {
        match self {
            MapLayerKind::Markers => "markers",
            MapLayerKind::Line => "line",
            MapLayerKind::Area => "area",
        }
    }
}

/// One point for [`MapLayer::markers`].
#[derive(Debug, Clone, PartialEq)]
pub struct MapMarker {
    /// Reported by `on_pick` and matched by `selected`.
    pub id: AttrValue,
    pub at: LngLat,
    /// Drawn under the dot.
    pub label: Option<AttrValue>,
    /// Overrides the layer's tone for this marker.
    pub tone: Option<Tone>,
}

impl MapMarker {
    pub fn new(id: impl Into<AttrValue>, at: impl Into<LngLat>) -> Self {
        Self { id: id.into(), at: at.into(), label: None, tone: None }
    }

    pub fn label(mut self, label: impl Into<AttrValue>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }
}

/// A set of features drawn one way. Layers are drawn in the order given,
/// the last on top.
///
/// The data is GeoJSON text. Feature properties the map understands: `id`
/// (reported by `on_pick`, matched by `selected`), `label` (markers), `tone`
/// (a [`Tone`] name such as `"ok"`) and `color` (a hex or `rgb()` colour).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapLayer {
    /// Unique among the map's layers.
    pub id: AttrValue,
    pub kind: MapLayerKind,
    /// A GeoJSON `FeatureCollection`, `Feature` or geometry, as text.
    pub data: AttrValue,
    /// The colour of features that set neither `tone` nor `color`. Unset
    /// means the accent.
    pub tone: Option<Tone>,
    /// Whether clicks on its features are reported by `on_pick`.
    pub interactive: bool,
}

impl MapLayer {
    /// A layer from GeoJSON text, for data that is already GeoJSON.
    pub fn geojson(id: impl Into<AttrValue>, kind: MapLayerKind, data: impl Into<AttrValue>) -> Self {
        Self { id: id.into(), kind, data: data.into(), tone: None, interactive: true }
    }

    /// A layer of point markers.
    pub fn markers(id: impl Into<AttrValue>, markers: impl IntoIterator<Item = MapMarker>) -> Self {
        let mut data = String::from(r#"{"type":"FeatureCollection","features":["#);
        for (index, marker) in markers.into_iter().enumerate() {
            if index > 0 {
                data.push(',');
            }
            data.push_str(r#"{"type":"Feature","geometry":{"type":"Point","coordinates":"#);
            position(&mut data, marker.at);
            data.push_str(r#"},"properties":{"id":"#);
            json_string(&mut data, &marker.id);
            if let Some(label) = &marker.label {
                data.push_str(r#","label":"#);
                json_string(&mut data, label);
            }
            if let Some(tone) = marker.tone {
                let _ = write!(data, r#","tone":"{}""#, tone.as_str());
            }
            data.push_str("}}");
        }
        data.push_str("]}");
        Self::geojson(id, MapLayerKind::Markers, data)
    }

    /// A layer holding one line through `path`. Its feature id is the layer id.
    pub fn line(id: impl Into<AttrValue>, path: impl IntoIterator<Item = LngLat>) -> Self {
        let id = id.into();
        let mut data = String::from(r#"{"type":"Feature","geometry":{"type":"LineString","coordinates":"#);
        positions(&mut data, path);
        feature_tail(&mut data, &id);
        Self::geojson(id, MapLayerKind::Line, data)
    }

    /// A layer holding one polygon with the outline `ring`, closed for you.
    /// Its feature id is the layer id.
    pub fn area(id: impl Into<AttrValue>, ring: impl IntoIterator<Item = LngLat>) -> Self {
        let id = id.into();
        let mut ring: Vec<LngLat> = ring.into_iter().collect();
        if ring.first() != ring.last() {
            ring.extend(ring.first().copied());
        }
        let mut data = String::from(r#"{"type":"Feature","geometry":{"type":"Polygon","coordinates":["#);
        positions(&mut data, ring);
        data.push(']');
        feature_tail(&mut data, &id);
        Self::geojson(id, MapLayerKind::Area, data)
    }

    /// Sets the colour of features that do not choose their own.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// Stops `on_pick` reporting clicks on this layer.
    pub fn inert(mut self) -> Self {
        self.interactive = false;
        self
    }
}

/// One feature under a click.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapHit {
    /// The [`MapLayer::id`] it belongs to.
    pub layer: AttrValue,
    /// Its `id` property; empty when the feature has none.
    pub id: AttrValue,
}

/// A click on the map: reported by `on_pick`.
#[derive(Debug, Clone, PartialEq)]
pub struct MapPick {
    pub at: LngLat,
    /// The features of interactive layers under the click, topmost first.
    /// Empty for a click on bare map.
    pub hits: Vec<MapHit>,
}

fn feature_tail(out: &mut String, id: &str) {
    out.push_str(r#"},"properties":{"id":"#);
    json_string(out, id);
    out.push_str("}}");
}

fn positions(out: &mut String, path: impl IntoIterator<Item = LngLat>) {
    out.push('[');
    for (index, at) in path.into_iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        position(out, at);
    }
    out.push(']');
}

fn position(out: &mut String, at: LngLat) {
    out.push('[');
    json_number(out, at.lng);
    out.push(',');
    json_number(out, at.lat);
    out.push(']');
}

/// JSON has no NaN or infinity; a position that is neither is nowhere.
pub(super) fn json_number(out: &mut String, value: f64) {
    let _ = write!(out, "{}", if value.is_finite() { value } else { 0.0 });
}

pub(super) fn json_option(out: &mut String, value: Option<&str>) {
    match value {
        Some(value) => json_string(out, value),
        None => out.push_str("null"),
    }
}

pub(super) fn json_string(out: &mut String, value: &str) {
    out.push('"');
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}
