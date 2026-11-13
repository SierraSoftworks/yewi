crate::page! {
    title: "VirtualList",
    category: Data,
    summary: "Use these for lists and tables that are too long to draw in full. `VirtualList` and `VirtualTable` keep only the rows in view in the document, so a hundred thousand rows scroll like thirty.",
    examples: [
        table => "A hundred thousand rows": "`VirtualTable` takes the columns, sorting and variants of `Table`, and draws a window of about thirty rows over the sorted data. Rows all share one height, columns have fixed widths and long cells are cut short. That's what lets it jump anywhere without measuring the rows that came before.",
        list => "A long list": "`VirtualList` draws each item with `render`, in rows that are `row_height` pixels tall. Screen readers hear each row's place in the whole list. Use it for logs, feeds and pickers, but for a few hundred items a plain list is simpler.",
        selection => "Selecting and opening rows": "Selection and row clicks work the same way as in `Table`. They're tracked by `row_key`, so ticked rows stay ticked while they're scrolled out of the document. `compact` and the `Zebra` and `Ruled` variants apply unchanged.",
        load_more => "Loading more as you scroll": "`on_range` reports which rows are currently drawn. When the end of that range reaches the end of the data you hold, fetch the next page and append it. The scroll position is kept because the earlier rows don't move.",
    ],
}
