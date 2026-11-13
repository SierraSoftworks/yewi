crate::page! {
    title: "Table",
    category: Data,
    summary: "Tables let you compare rows of the same kind of thing across columns. `Table` renders typed rows from column definitions and can sort, select and expand them. `SimpleTable` gives you the same styling around markup you write yourself.",
    examples: [
        basic => "Basic": "A `Column` has a key, a header and a function that turns a row into its cell. The default look is open, with one rule under a sentence-case header and whitespace between the rows. Mark the column that names each row with `row_header`, and right-align numbers with `numeric`.",
        sorting => "Sorting": "Give a column an ordering and its header becomes a button. Click it to sort, and click again to reverse the order. Orderings are typed, so `sort_text` puts `web-2` before `web-10`, and `sort_by_key` takes numbers, timestamps, or an enum whose order is the order you want statuses to appear in. Rows that tie keep the order of the data.",
        server_sort => "Sorting on the server": "When the data is paged or too large to hold, mark columns `sortable` without an ordering, own the `sort` state yourself, and fetch the rows in `on_sort`. The table shows the sort direction but leaves the rows in the order you give them.",
        variants => "Variants": "`Open` is the default. `Ruled` adds a hairline between rows, for dense data that people read across. `Zebra` uses a tonal header and alternating fills, which helps with wide tables.",
        dense => "Large and dense": "For large tables, `compact` tightens the rows and `max_height` with `sticky_header` keeps the header in view while you scroll. `fixed` lays the columns out from their declared widths, so the browser doesn't need to measure every row. With `multi_sort`, Shift-click adds another column to break ties. Rows only redraw when their own data changes.",
        selection => "Selecting rows": "Pass `selected` to add a checkbox column, and update it from `on_select`. Selection is tracked by `row_key`, so it survives sorting and reloading. The checkbox in the header selects everything, and it shows a dash when only some rows are selected.",
        expandable => "Expandable rows": "`detail` adds a disclosure button to each row and renders the details in a full-width row underneath it. Details are only rendered while they're open, so `on_expand` is the place to load them.",
        row_click => "Clickable rows": "`on_row_click` opens a row when you click anywhere on it, except on a link or control inside it. Keep a real link to the same destination in the row as well, because that's what keyboard and screen reader users rely on.",
        states => "Loading and empty": "While `loading` is set and there are no rows yet, the table holds its shape with placeholder rows. If it already has rows, it keeps them on screen, dimmed, until the refresh arrives. When there's nothing to show, it renders `empty`, which by default is a small `EmptyState` titled with `empty_label`.",
        in_card => "In a card": "Make the card body `flush` and the table runs edge to edge. Its first and last columns take on the card's padding, so the first column starts under the card title and the row fills reach the border. A flush `Panel` body works the same way.",
        simple => "Hand-written tables": "`SimpleTable` takes plain table markup. Use it for rows that aren't uniform, such as group headings, and for rendered documents. `SortHeader` gives its columns the same sort button as `Table`.",
    ],
}
