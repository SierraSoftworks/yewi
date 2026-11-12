crate::page! {
    title: "Inline code",
    category: Basic,
    summary: "A command, identifier, path or value quoted inside a sentence.",
    examples: [
        basic => "In a sentence": "Wrap anything the reader might type or search for. The chip scales with the text around it, so it works in body copy, captions and headings alike.",
        wrapping => "Long values": "A long identifier wraps at any character rather than forcing its container to scroll, and keeps its padding on every line.",
        in_links => "Inside a link": "Inside a `Link` the chip takes the link's colour, so the whole phrase still reads as one link.",
        options => "Style options": "The default is a borderless tinted chip. The component tokens give an outlined chip or plain code-coloured text where an app prefers it.",
    ],
}
