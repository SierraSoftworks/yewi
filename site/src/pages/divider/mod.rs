crate::page! {
    title: "Divider",
    category: Layout,
    summary: "A divider draws a hairline between groups of content. Try space or a tonal fill first, and only use a rule when those aren't enough.",
    examples: [
        basic => "Between content": "A divider has no margin of its own, so the `Stack` it sits in sets the space on both sides.",
        labelled => "With a label": "A label names what follows, such as a date in a feed or the \"or\" between two ways to sign in. Use `align` to move it to the start or end.",
        vertical => "Vertical": "In a row, a vertical divider separates groups of controls and stretches to the height of the row.",
    ],
}
