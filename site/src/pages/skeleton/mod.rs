crate::page! {
    title: "Skeleton",
    category: Basic,
    summary: "Skeletons hold the space for content that's still loading, so nothing on the page moves when it arrives.",
    examples: [
        shapes => "Shapes": "`Text` stands in for a line of type and takes up exactly one line of the surrounding text. `Block` is a rectangle with a `height`, and `Circle` stands in for an avatar or marker and is sized by `width`.",
        paragraph => "Paragraphs": "Use `lines` to repeat a text placeholder. The last line is shorter, like the end of a real paragraph, and the line height follows the surrounding type.",
        composed => "In the shape of the content": "Lay the placeholders out exactly as the content will be. They're hidden from assistive technology, so mark the region `aria-busy` and announce the wait once.",
        animation => "Animation": "`Shimmer` is the default. `Pulse` is quieter, and `None` suits placeholders that stay on screen. People who prefer reduced motion always get the static version.",
    ],
}
