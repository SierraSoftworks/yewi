crate::page! {
    title: "InlineCode",
    category: Basic,
    summary: "InlineCode quotes a command, identifier, path or value inside a sentence.",
    examples: [
        basic => "In a sentence": "Wrap anything the reader might type or search for. The chip scales with the text around it, so it works in body copy, captions and headings alike.",
        wrapping => "Long values": "A long identifier wraps at any character rather than forcing its container to scroll, and it keeps its padding on every line.",
        in_links => "Inside a link": "Inside a `Link`, the chip takes the link's colour, so the whole phrase still reads as one link.",
        options => "Style options": "By default it's a borderless tinted chip. If your app prefers, the component tokens can give you an outlined chip or plain code-coloured text instead.",
    ],
}
