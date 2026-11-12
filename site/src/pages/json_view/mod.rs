crate::page! {
    title: "JsonView",
    category: Data,
    summary: "A JSON document as a collapsible tree to explore, or as highlighted source to read.",
    examples: [
        basic => "Exploring a document": "The tree folds every object and array behind its key, with a count of what is inside. `open` sets how many levels start expanded and `name` labels the root. The button copies the whole document, indented.",
        source => "Reading a payload": "`Source` prints the document as indented JSON, exactly as it would be copied. Use it for short payloads that are read from top to bottom, such as a workflow input or an event body.",
        variants => "Tree or source": "The same document both ways. The tree drops quotes and commas and writes short arrays on one line, so it is denser; the source is the literal text.",
        plain => "Inside another surface": "`plain` drops the tinted surface so the view can sit in a card, a panel or an expanded table row. `copy` can be turned off where the surrounding view already offers it.",
        large => "Large documents": "A folded node renders nothing but its summary, and a long array or object is shown `page_size` children at a time, so the page only ever holds what has been asked for. `max_height` keeps a long document from taking over the view.",
        parse => "From text": "`Json::parse` reads a response body and reports where it stopped on bad input. With the `serde_json` feature a `serde_json::Value` converts with `Json::from`. Parse once, in a `use_memo` or in state, not on every render.",
    ],
}
