crate::page! {
    title: "Table",
    category: Data,
    summary: "Rows of the same kind of thing, compared across columns. `Table` renders typed rows through column definitions and sorts, selects and expands them; `SimpleTable` is the same frame around markup you write yourself.",
    examples: [
        basic => "Basic": "A `Column` has a key, a header and a function from a row to its cell. The default look is open: one rule under a sentence-case header and whitespace between rows. Mark the column that names each row with `row_header`, and right-align numbers with `numeric`.",
        sorting => "Sorting": "Give a column an ordering and its header becomes a button: click to sort, click again to reverse. Orderings are typed: `sort_text` reads `web-2` before `web-10`, `sort_by_key` takes numbers, timestamps or an enum whose order is the order statuses should appear in. Ties keep the order of the data.",
        server_sort => "Sorting on the server": "For data that is paged or too large to hold, mark columns `sortable` without an ordering, own the `sort` state, and fetch in `on_sort`. The table shows the direction and leaves the rows as given.",
        variants => "Variants": "`Open` is the default. `Ruled` adds a hairline between rows for dense data read across; `Zebra` uses a tonal header and alternating fills for wide tables.",
        dense => "Large and dense": "`compact` tightens rows, `max_height` with `sticky_header` keeps the header in view, and `fixed` lays columns out from their declared widths so the browser need not measure every row. `multi_sort` lets Shift-click add a tie-break column. Rows redraw only when their own data changes.",
        selection => "Selecting rows": "Pass `selected` to add a checkbox column, and update it from `on_select`. Selection is tracked by `row_key`, so it survives sorting and reloading. The header box selects everything and shows a dash when only some rows are selected.",
        expandable => "Expandable rows": "`detail` adds a disclosure button to each row and renders the details in a full-width row underneath. Details are only rendered while open; `on_expand` is the place to load them.",
        row_click => "Clickable rows": "`on_row_click` opens a row from a click anywhere on it, except on a link or control inside it. Keep a real link in the row for the same destination: it is what the keyboard and screen readers use.",
        states => "Loading and empty": "While `loading` with no rows the table reserves its shape with placeholder rows; with rows it keeps them, dimmed, until the refresh lands. With nothing to show it renders `empty`, by default a small `EmptyState` titled with `empty_label`.",
        in_card => "In a card": "Make the card body `flush` and the table runs edge to edge: its first and last columns take the card's padding, so the first column starts under the card title and row fills reach the border. A flush `Panel` body does the same.",
        simple => "Hand-written tables": "`SimpleTable` takes plain table markup for rows that are not uniform, such as group headings, and for rendered documents. `SortHeader` gives its columns the same sort button.",
    ],
}
