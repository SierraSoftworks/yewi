crate::page! {
    title: "Markdown",
    category: Data,
    summary: "Markdown renders a document with yewi's headings, links, tables and code blocks. It's safe to use on text written by someone you don't trust.",
    feature: "markdown",
    examples: [
        document => "A document": "Give `source` the Markdown text. It supports CommonMark along with tables, strikethrough and task lists. Fenced code becomes a `CodeBlock` and a table becomes a `SimpleTable`. The document picks up the text size and colour of wherever you place it.",
        embedded => "Inside a card": "When a document sits under one of the page's headings, its own headings should continue that outline. `top_heading` sets the level that a `#` heading renders at, which also brings its size down to suit a card or panel.",
        untrusted => "Untrusted text": "Nothing in the source reaches the page as markup. Raw HTML is shown as written, or dropped if you use `MarkdownHtml::Omit`. Links and images keep their text but lose their address unless it's `http`, `https` or relative (links may also use `mailto` and `tel`). Turning `images` off replaces each image with its alternative text, so the reader's browser doesn't contact anyone.",
        links => "Links": "`external_links` opens links to other sites in a new tab and marks them as external. `on_link_click` reports every link activation before the browser follows it, so an app with a client-side router can prevent the default and navigate itself.",
        keys => "Keys and line breaks": "Markdown has no syntax for keys or line breaks, so two pieces of inline HTML are understood: `<kbd>` for a key and `<br>` for a line break. Attributes on them aren't accepted.",
    ],
}
