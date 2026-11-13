crate::page! {
    title: "StatusDot",
    category: Basic,
    summary: "A status dot is a small coloured dot that shows a status. It sits beside a label or inside a Pill.",
    examples: [
        tones => "Tones": "Pair the dot with a text label, and choose the tone by what it means. When it's next to a label the dot is decorative, so only the label is announced.",
        active => "Live states": "`active` adds a slow pulse for something that's live or in progress. Save it for states that are changing right now. The dot stays still for users who prefer reduced motion.",
        standalone => "On its own": "When there's no room for text, such as in a dense table column, give the dot a `label` so the status is still announced. `size` takes any CSS length.",
        inherited => "Following the text colour": "Without a `tone`, the dot takes the colour of the text around it. That way it matches a coloured label or a `Pill` with no extra work.",
    ],
}
