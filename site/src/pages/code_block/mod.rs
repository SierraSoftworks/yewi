crate::page! {
    title: "CodeBlock",
    category: Data,
    summary: "Code blocks show source code or configuration in a frame. The header names the file and the language, and it has a button to copy the code.",
    feature: "code",
    examples: [
        basic => "A file": "Give the block the code, its `language` and, where it helps, the `filename`. The copy button copies the text exactly as you gave it. Syntax colouring needs the `highlight` feature, and without it the same block shows the code plain.",
        languages => "Languages": "`language` accepts a name, an alias or a file extension in any case, just as a Markdown code fence does. The header shows the language's proper name. If there's no lexer for a language, the code is shown plain under the name you gave.",
        lines => "Line numbers and marked lines": "`line_numbers` numbers the lines, and `marked` draws attention to some of them. You give `marked` line numbers and ranges, counted from 1. Neither the numbers nor the marks are selected or copied with the code.",
        dark => "Always dark": "A block follows the page's theme. To keep it dark on a light page, as a terminal would be, set `theme` to `Theme::Dark`.",
        long => "Long lines and long files": "Code scrolls sideways rather than wrapping, and you can focus the block to scroll it with the arrow keys. Use `wrap` for prose-like text such as logs. To make a long file scroll inside the block, set the `--yi-code-block-max-height` token. `actions` adds controls beside the copy button.",
        bare => "Without a header": "With no file name, no language and `copy` turned off, the block has no header. This suits short snippets in running documentation.",
    ],
}
