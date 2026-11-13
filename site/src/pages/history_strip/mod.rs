crate::page! {
    title: "HistoryStrip",
    category: Data,
    summary: "A history strip is a row of small cells, one per period or run, each coloured by how it went. Hovering a cell shows its detail.",
    examples: [
        basic => "Basic": "Pass the `cells`, oldest first, and a `detail` callback that builds the popover for one cell. Hover or press a cell to see its popover. From the keyboard, the strip is a single tab stop and the arrow keys move between cells. You decide the popover's layout.",
        discrete => "Discrete runs": "`HistoryStripVariant::Discrete` draws separate cells of a fixed width, gathered at the end. It suits the last few runs of a job rather than an unbroken record. `on_select` reports the cell that was pressed or chosen with `Enter`.",
        plain => "Without detail": "Without `detail`, the strip isn't interactive and takes no focus, which suits a dense table column. Cells still carry their labels for assistive technology. `Tone::Neutral` stands for no data, and `size` sets the height.",
        narrow => "Not enough room": "Cells never shrink below a readable width. When the strip is too narrow to fit them all, the oldest cells are clipped and the most recent ones stay in view.",
    ],
}
