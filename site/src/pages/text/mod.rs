crate::page! {
    title: "Text",
    category: Basic,
    summary: "Body text placed on the type scale and in the colour hierarchy, with truncation and number formatting.",
    examples: [
        colours => "Colour hierarchy": "Three neutrals rank information: primary for content, `Muted` for descriptions, `Subtle` for captions. `Accent` and the status tones carry meaning, so use them only where the text reports something.",
        sizes => "Sizes and weights": "`Md` (14px) suits controls and dense UI, `Lg` (15px) longer reading text. Emphasis stops at semibold.",
        truncate => "Truncating": "`truncate` keeps text on one line; `lines` clamps a paragraph. Both end with an ellipsis and keep the full text for assistive technology.",
        numbers => "Numbers and identifiers": "`tabular` gives digits a fixed width so columns line up and counters do not jitter. `mono` sets hashes, addresses and other identifiers in the code font.",
        elements => "Semantic elements": "`tag` chooses the element. `Strong` and `Em` mark importance and stress, `Small` marks side comments, and `P` makes a paragraph.",
    ],
}
