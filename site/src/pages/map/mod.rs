crate::page! {
    title: "Map",
    category: Visualisation,
    summary: "An interactive map with a toolbar, a sidebar and panels floating over it. The engine is MapLibre GL, bundled and fetched only when a map is shown.",
    feature: "map",
    examples: [
        markers => "Markers": "Give the map GeoJSON as `layers` and it draws them in theme colours. The default `Basemap::Outline` is a bundled world outline that needs no network; use `Basemap::osm()` or `Basemap::raster(url)` for street-level tiles and `Basemap::style(url)` for vector tiles. `on_pick` reports what was clicked and `selected` highlights it.",
        layout => "Toolbar, sidebar and panel": "The chrome floats over the map in five slots: `toolbar`, `sidebar`, `overlay`, `panel` and `footer`. They are cells of one grid, so nothing overlaps at any width: three columns on a wide map, two below 62rem and one stack below 40rem. Tools are subtle `IconButton`s in a `MapToolbar`. Always list what is on the map in the sidebar: it is the way in for anyone who cannot use a canvas with a pointer.",
        shapes => "Lines and areas": "`MapLayer::line` and `MapLayer::area` cover the simple cases; `MapLayer::geojson` takes any GeoJSON text. Layers draw in order, the last on top. A layer's `tone` colours every feature that does not set its own.",
        locator => "A fixed locator": "With `interactive`, `controls` and `scale` off the map is a picture: enough to show where something is, inside a card or beside a form.",
        live => "Moving the camera and the data": "`on_ready` hands over a `MapHandle`. Use it to move the camera, and to push a layer's data straight to the engine with `set_data` when a feed updates faster than the page should re-render.",
    ],
}
