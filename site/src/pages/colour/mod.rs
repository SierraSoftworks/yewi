crate::page! {
    title: "Colour",
    category: Foundations,
    summary: "Neutral surfaces and text, one accent per app, and status tones that are only ever semantic.",
    api: [],
    examples: [
        neutrals => "Neutrals": "Pure grey on white in light mode and a cool charcoal in dark mode. Three text steps: `--yw-fg` for content, `--yw-fg-muted` for supporting text and `--yw-fg-subtle` for labels and metadata.",
        accent => "Accent": "An app sets `--yw-accent`; fills, text, soft tints and the focus ring are derived from it with their lightness clamped for contrast. The neutrals lean towards the accent's hue.",
        status => "Status tones": "Each tone has a text colour and a soft fill. Use them to report state, never to decorate.",
    ],
}
