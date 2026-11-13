crate::page! {
    title: "Text",
    category: Basic,
    summary: "Text places body copy on the type scale and in the colour hierarchy. It can also truncate text and format numbers.",
    examples: [
        colours => "Colour hierarchy": "Three neutral colours rank information: primary for content, `Muted` for descriptions and `Subtle` for captions. `Accent` and the status tones carry meaning, so only use them where the text is reporting something.",
        sizes => "Sizes and weights": "`Md` (14px) suits controls and dense UI, and `Lg` (15px) suits longer reading text. Emphasis goes no heavier than semibold.",
        truncate => "Truncating": "`truncate` keeps text on one line, and `lines` clamps a paragraph. Both end with an ellipsis and keep the full text available to assistive technology.",
        numbers => "Numbers and identifiers": "`tabular` gives digits a fixed width, so columns line up and counters don't jitter. `mono` sets hashes, addresses and other identifiers in the code font.",
        elements => "Semantic elements": "`tag` chooses the HTML element. `Strong` and `Em` mark importance and stress, `Small` marks side comments, and `P` makes a paragraph.",
    ],
}
