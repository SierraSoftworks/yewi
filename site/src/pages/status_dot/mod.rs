crate::page! {
    title: "StatusDot",
    category: Basic,
    summary: "A small coloured dot that carries a status, beside a label or inside a Pill.",
    examples: [
        tones => "Tones": "Pair the dot with a text label and choose the tone by meaning. Next to a label the dot is decorative, so the label alone is announced.",
        active => "Live states": "`active` adds a slow pulse for something live or in flight. Reserve it for states that are changing right now; it stays still for users who prefer reduced motion.",
        standalone => "On its own": "Where there is no room for text, such as a dense table column, give the dot a `label` so the status is still announced. `size` takes any CSS length.",
        inherited => "Following the text colour": "Without a `tone` the dot takes the colour of the text around it, so it matches a coloured label or a `Pill` with no extra work.",
    ],
}
