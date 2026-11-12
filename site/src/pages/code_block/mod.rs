crate::page! {
    title: "CodeBlock",
    category: Data,
    summary: "Source code or configuration in a framed block, with a header naming the file and language and a button to copy it.",
    feature: "code",
    examples: [
        basic => "A file": "Give the block the code, its `language` and, where it helps, the `filename`. The copy button copies the text exactly as given. Colouring needs the `highlight` feature; without it the same block shows the code plain.",
        languages => "Languages": "`language` takes a name, alias or file extension in any case, as a Markdown code fence does. The header shows the language's proper name, and a language without a lexer is shown plain under the name given.",
        lines => "Line numbers and marked lines": "`line_numbers` numbers the lines and `marked` draws attention to some of them, as numbers and ranges counted from 1. Neither is selected or copied with the code.",
        dark => "Always dark": "A block follows the theme of the page. Set `theme` to `Theme::Dark` to keep it dark on a light page, as a terminal would be.",
        long => "Long lines and long files": "Code scrolls sideways rather than wrapping, and the block can be focused so that the arrow keys scroll it. Use `wrap` for prose-like text such as logs, and the `--yi-code-block-max-height` token to make a long file scroll inside the block. `actions` adds controls beside the copy button.",
        bare => "Without a header": "With no file name, no language and `copy` off there is no header, which suits short snippets in running documentation.",
    ],
}
