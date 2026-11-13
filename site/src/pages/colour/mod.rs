crate::page! {
    title: "Colour",
    category: Foundations,
    summary: "The colour system uses neutral surfaces and text, a single accent for each app, and status tones that only ever carry meaning.",
    api: [],
    examples: [
        neutrals => "Neutrals": "Light mode uses pure grey on white, and dark mode uses a cool charcoal. There are three steps for text. `--yi-fg` is for content, `--yi-fg-muted` is for supporting text and `--yi-fg-subtle` is for labels and metadata.",
        accent => "Accent": "Each app sets `--yi-accent`, and the fills, text, soft tints and focus ring are derived from it. Their lightness is clamped so that they keep enough contrast, and the neutrals lean towards the accent's hue.",
        status => "Status tones": "Each tone has a text colour and a soft fill. Use them to report state, and never just to decorate.",
    ],
}
