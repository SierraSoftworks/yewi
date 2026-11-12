crate::page! {
    title: "HistoryStrip",
    category: Data,
    summary: "A row of small cells, one per period or run and coloured by how it went, with the detail of each on hover.",
    examples: [
        basic => "Basic": "Give the `cells`, oldest first, and a `detail` callback that builds the popover for one cell. Hover or press a cell to see it; from the keyboard the strip is one tab stop and the arrow keys move between cells. The popover's layout is the caller's.",
        discrete => "Discrete runs": "`HistoryStripVariant::Discrete` draws separate cells of a fixed width, gathered at the end: the last runs of a job rather than an unbroken record. `on_select` reports the cell that was pressed or chosen with `Enter`.",
        plain => "Without detail": "Without `detail` the strip is not interactive and takes no focus, which suits a dense table column. Cells still carry their labels for assistive technology, `Tone::Neutral` stands for no data, and `size` sets the height.",
        narrow => "Not enough room": "Cells never shrink below a readable width. When the strip is too narrow for all of them, the oldest are clipped and the most recent stay in view.",
    ],
}
