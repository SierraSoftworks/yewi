crate::page! {
    title: "KeyValueEditor",
    category: Form,
    summary: "KeyValueEditor edits a short list of name and value pairs, such as labels, environment variables or headers.",
    examples: [
        basic => "Labels": "The editor is controlled: it shows `rows` and reports every edit, added row and removed row through `on_change`. Put it in a `Field` with `group` for a label and hint, and set `row_label` so each input is named after what a row is.",
        validation => "Checking rows": "The editor doesn't check anything itself. Work out what's wrong from the rows and pass each problem as the row's index and a message. `KeyValue::is_blank` finds rows that were added and never filled in, which are usually skipped.",
        options => "Separator, limit and disabled": "An empty `separator` leaves two plain columns, which reads better for prose values. `max_rows` disables the add button at the limit, and `--yi-key-value-editor-key-width` changes how the row is shared.",
    ],
}
