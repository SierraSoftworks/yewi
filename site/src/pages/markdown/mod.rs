crate::page! {
    title: "Markdown",
    category: Data,
    summary: "A Markdown document drawn with yewi's headings, links, tables and code blocks, safe to use on text from an untrusted author.",
    feature: "markdown",
    examples: [
        document => "A document": "Give `source` the Markdown text. CommonMark is supported along with tables, strikethrough and task lists; fenced code becomes a `CodeBlock` and a table a `SimpleTable`. The document inherits the size and colour of where it is placed.",
        embedded => "Inside a card": "A document shown under a heading of the page should continue that outline. `top_heading` sets the level a `#` heading renders at, which also brings its size down to suit a card or panel.",
        untrusted => "Untrusted text": "Nothing in the source reaches the page as markup. Raw HTML is shown as written, or dropped with `MarkdownHtml::Omit`; links and images whose address is not `http`, `https` or relative (plus `mailto` and `tel` for links) keep their text and lose the address. `images` off replaces each image with its alternative text, so a reader's browser contacts nobody.",
        links => "Links": "`external_links` opens links to other sites in a new tab and marks them. `on_link_click` reports every activation before the browser follows it, so an app with a client-side router can prevent the default and navigate itself.",
        keys => "Keys and line breaks": "Two pieces of inline HTML are understood because Markdown has no syntax for them: `<kbd>` for a key and `<br>` for a line break. Attributes on them are not accepted.",
    ],
}
