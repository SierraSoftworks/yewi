crate::page! {
    title: "Skeleton",
    category: Basic,
    summary: "Reserves the layout of content that is loading, so nothing moves when it arrives.",
    examples: [
        shapes => "Shapes": "`Text` stands in for a line of type and takes exactly one line of the surrounding text. `Block` is a rectangle with a `height`, and `Circle` an avatar or marker sized by `width`.",
        paragraph => "Paragraphs": "`lines` repeats a text placeholder; the last line is shorter, like a real paragraph. Line height follows the surrounding type.",
        composed => "In the shape of the content": "Lay placeholders out exactly as the content will be. They are hidden from assistive technology, so mark the region `aria-busy` and announce the wait once.",
        animation => "Animation": "`Shimmer` is the default. `Pulse` is quieter, and `None` suits placeholders that stay on screen. People who prefer reduced motion always get the static form.",
    ],
}
