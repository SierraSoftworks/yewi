crate::page! {
    title: "StatTile",
    category: Data,
    summary: "A headline figure with its label, and optionally how it changed and what it means.",
    examples: [
        basic => "Headline figures": "A row of tiles summarises a period. `delta` and `trend` show the change since the comparison period; set `lower_is_better` where a fall is good news. Format values in the app, and use a dash when there is no data.",
        plain => "Inside a card": "`Plain` drops the surface so figures can sit together in a card or panel. `detail` explains a figure and `tone` colours it when the figure itself is the alert.",
        selectable => "Choosing a metric": "With `on_click` a tile is a toggle button, for tiles that choose what the chart or table below shows. Keep exactly one `selected`.",
        sizes => "Sizes, units and links": "`size` scales the value. `unit` sits small after it. With `href` the whole tile links to where the figure is explained.",
        label_style => "Label style": "Labels are sentence case by default. An app that prefers small uppercase labels sets the label tokens on `.yi-stat-tile` in its stylesheet.",
    ],
}
