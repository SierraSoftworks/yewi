crate::page! {
    title: "Highlight",
    category: Data,
    summary: "Colours the syntax of source text. Small built-in lexers cover common languages, and syntect can be switched on for the rest.",
    feature: "highlight",
    examples: [
        inline => "Inline and in your own block": "`Highlight` renders a `code` element and nothing else, so it sits in a sentence or inside a `pre` of your own. For a framed block with a header, use `CodeBlock`.",
        languages => "Built-in languages": "The built-in lexers recognise comments, strings, numbers, keywords and a few shapes such as calls and keys. That is most of what makes code readable, for a few kilobytes of wasm. Enable `highlight-syntect` for full grammars and many more languages, at about a megabyte.",
        tokens => "Tokens and their colours": "`tokenize` returns the tokens without rendering them, for a caller that draws code itself. Each `TokenKind` has a class and a `--yi-highlight-*` token; set the token on any ancestor to recolour that kind.",
    ],
}
