crate::page! {
    title: "JsonView",
    category: Data,
    summary: "JsonView shows a JSON document either as a collapsible tree to explore or as highlighted source to read.",
    examples: [
        basic => "Exploring a document": "The tree folds every object and array behind its key and shows a count of what's inside. `open` sets how many levels start expanded, and `name` labels the root. The button copies the whole document, indented.",
        source => "Reading a payload": "`Source` prints the document as indented JSON, exactly as it would be copied. Use it for short payloads that people read from top to bottom, such as a workflow input or an event body.",
        variants => "Tree or source": "Here's the same document both ways. The tree drops quotes and commas and puts short arrays on one line, so it's denser, while the source shows the literal text.",
        plain => "Inside another surface": "`plain` drops the tinted surface so the view can sit in a card, a panel or an expanded table row. You can turn `copy` off where the surrounding view already offers copying.",
        large => "Large documents": "A folded node renders nothing but its summary, and a long array or object shows `page_size` children at a time, so the page only ever holds what someone has asked to see. `max_height` stops a long document from taking over the view.",
        parse => "From text": "`Json::parse` reads a response body and, on bad input, reports where it stopped. With the `serde_json` feature, you can convert a `serde_json::Value` with `Json::from`. Parse once, in a `use_memo` or in state, rather than on every render.",
    ],
}
