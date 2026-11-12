crate::page! {
    title: "VirtualList",
    category: Data,
    summary: "Lists and tables too long to draw in full. `VirtualList` and `VirtualTable` keep only the rows in view in the document, so a hundred thousand rows scroll like thirty.",
    examples: [
        table => "A hundred thousand rows": "`VirtualTable` takes the columns, sorting and variants of `Table` and draws a window of about thirty rows over the sorted data. Rows share one height, columns have fixed widths and long cells are cut short, which is what lets it jump anywhere without measuring what came before.",
        list => "A long list": "`VirtualList` draws each item with `render`, in rows of `row_height` pixels. Screen readers hear each row's place in the whole list. Use it for logs, feeds and pickers; for a few hundred items a plain list is simpler.",
        selection => "Selecting and opening rows": "Selection and row clicks work as they do in `Table`, tracked by `row_key`, so ticked rows stay ticked while they are scrolled out of the document. `compact` and the `Zebra` and `Ruled` variants apply unchanged.",
        load_more => "Loading more as you scroll": "`on_range` reports the rows now drawn. When its end reaches what you hold, fetch the next page and append it: the scroll position is kept because earlier rows do not move.",
    ],
}
