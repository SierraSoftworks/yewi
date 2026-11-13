crate::page! {
    title: "Highlight",
    category: Data,
    summary: "Highlight colours the syntax of source text. Small built-in lexers cover common languages, and you can switch on syntect for the rest.",
    feature: "highlight",
    examples: [
        inline => "Inline and in your own block": "`Highlight` renders a `code` element and nothing else, so it can sit in a sentence or inside a `pre` of your own. For a framed block with a header, use `CodeBlock`.",
        languages => "Built-in languages": "The built-in lexers recognise comments, strings, numbers, keywords and a few shapes such as calls and keys. That's most of what makes code readable, and it costs only a few kilobytes of wasm. If you need full grammars and many more languages, enable `highlight-syntect`, which adds about a megabyte.",
        tokens => "Tokens and their colours": "If you draw code yourself, `tokenize` returns the tokens without rendering them. Each `TokenKind` has a class and a `--yi-highlight-*` token, and you can set the token on any ancestor to recolour that kind.",
    ],
}
