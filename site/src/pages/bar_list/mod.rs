crate::page! {
    title: "BarList",
    category: Data,
    summary: "A bar list shows a ranked breakdown. Each row has a label, a figure and a bar that shows its size compared with the largest row.",
    examples: [
        basic => "Top pages": "Rows are ranked largest first, and each bar is drawn against the largest row. `show_share` adds each row's share of the total, and `heading` and `value_heading` label the columns.",
        variants => "Bar placement": "`Behind` draws a soft bar behind the label and is the densest form. `Under` draws a thin bar on a track beneath the label, which suits long labels and gives the figures more room.",
        tabbed => "Tabbed breakdown in a card": "To switch between different dimensions of the same data, put one list in each `Tab` of pill `Tabs`. Rows can take an `icon` or a short text `glyph` before the label, and a `muted` row stands for missing values.",
        filter => "Rows that filter": "With `on_select`, every row becomes a toggle button. Mark the active filter with `selected`, and clear it when the same row is chosen again. `action` adds a control at the end of each row, which shows on hover and focus.",
        overflow => "Long lists": "`Expand` shows the first rows with a control to reveal the rest, and `Pages` shows one page at a time. When the rows are only the top of a longer list, set `total` so that the shares stay honest.",
        tones => "Tones, links and custom figures": "Set `tone` on the list or on a single item to colour the bar with a status. An item with `href` becomes a link, and `display` replaces the formatted amount. `BarListOrder::Given` keeps the data in the order you gave it.",
    ],
}
