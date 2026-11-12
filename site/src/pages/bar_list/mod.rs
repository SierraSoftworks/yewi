crate::page! {
    title: "BarList",
    category: Data,
    summary: "A ranked breakdown: each row is a label, a figure and a bar showing its size against the largest.",
    examples: [
        basic => "Top pages": "Rows are ranked largest first and each bar is drawn against the largest row. `show_share` adds each row's share of the total, and `heading` and `value_heading` label the columns.",
        variants => "Bar placement": "`Behind` draws a soft bar behind the label and is the densest form. `Under` draws a thin bar on a track beneath it, which suits long labels and gives the figures more air.",
        tabbed => "Tabbed breakdown in a card": "Put one list in each `Tab` of pill `Tabs` to switch between dimensions of the same data. Rows take an `icon` or a short text `glyph` before the label, and a `muted` row stands for missing values.",
        filter => "Rows that filter": "With `on_select` every row is a toggle button; mark the active filter with `selected` and clear it when the same row is chosen again. `action` adds a control at the end of each row, shown on hover and focus.",
        overflow => "Long lists": "`Expand` shows the first rows with a control for the rest. `Pages` shows a page at a time. Set `total` when the rows are only the top of a longer list, so shares stay honest.",
        tones => "Tones, links and custom figures": "`tone` on the list or on a single item colours the bar to carry a status. An item with `href` is a link, `display` replaces the formatted amount and `BarListOrder::Given` keeps data in its own order.",
    ],
}
