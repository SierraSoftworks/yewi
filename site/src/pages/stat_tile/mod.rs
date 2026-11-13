crate::page! {
    title: "StatTile",
    category: Data,
    summary: "A stat tile shows a headline figure with its label. It can also show how the figure changed and what it means.",
    examples: [
        basic => "Headline figures": "A row of tiles summarises a period. `delta` and `trend` show the change since the comparison period, and you can set `lower_is_better` when a fall is good news. Format the values in your app, and use a dash when there's no data.",
        plain => "Inside a card": "`Plain` drops the surface so figures can sit together inside a card or panel. `detail` explains a figure, and `tone` colours it when the figure itself is the alert.",
        selectable => "Choosing a metric": "With `on_click`, a tile becomes a toggle button. This is useful when the tiles choose what the chart or table below them shows. Keep exactly one tile `selected`.",
        sizes => "Sizes, units and links": "`size` scales the value, and `unit` sits in small type after it. With `href`, the whole tile links to the place where the figure is explained.",
        label_style => "Label style": "Labels are sentence case by default. If your app prefers small uppercase labels, set the label tokens on `.yi-stat-tile` in its stylesheet.",
    ],
}
