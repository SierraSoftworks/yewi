crate::page! {
    title: "Map",
    category: Visualisation,
    summary: "Map shows an interactive map with a toolbar, a sidebar and panels floating over it. It runs on MapLibre GL, which is bundled and only fetched when a map is shown.",
    feature: "map",
    examples: [
        markers => "Markers": "Give the map GeoJSON as `layers` and it draws them in theme colours. The default `Basemap::Outline` is a bundled world outline, so it doesn't need the network. For street-level tiles use `Basemap::osm()` or `Basemap::raster(url)`, and for vector tiles use `Basemap::style(url)`. `on_pick` tells you what was clicked, and `selected` highlights it.",
        layout => "Toolbar, sidebar and panel": "The chrome floats over the map in five slots: `toolbar`, `sidebar`, `overlay`, `panel` and `footer`. They're cells of one grid, so nothing overlaps at any width. A wide map gets three columns, which drop to two below 62rem and a single stack below 40rem. Put tools in a `MapToolbar` as subtle `IconButton`s. You should always list what's on the map in the sidebar, because that's the way in for anyone who can't use a canvas with a pointer.",
        shapes => "Lines and areas": "`MapLayer::line` and `MapLayer::area` cover the simple cases, and `MapLayer::geojson` takes any GeoJSON text. Layers are drawn in order, with the last one on top. A layer's `tone` colours every feature that doesn't set its own.",
        locator => "A fixed locator": "When you turn `interactive`, `controls` and `scale` off, the map becomes a picture. That's enough to show where something is inside a card or beside a form.",
        live => "Moving the camera and the data": "`on_ready` hands you a `MapHandle`, which you can use to move the camera. When a feed updates faster than the page should re-render, use its `set_data` to push a layer's data straight to the engine.",
    ],
}
