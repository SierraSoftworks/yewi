crate::page! {
    title: "Anchor positioning",
    category: Utilities,
    summary: "Places a floating element beside the element it belongs to and keeps it on screen. Tooltip and Popover are built on it.",
    examples: [
        basic => "A floating panel": "`use_anchored` takes the anchor, the floating element and whether it is open. Render the floating element only while open, with the `yi-anchored` class and `popover=\"manual\"` so that nothing clips it or stacks above it.",
        placement => "Placement": "`AnchorOptions` sets the preferred `Placement`, the gap and the distance kept from the window's edges. The side actually used is written to `data-yi-side` on the floating element, for arrows and side-aware styling.",
        collision => "Staying on screen": "When the preferred side lacks room the element flips to the opposite one, and it slides along the anchor to stay inside the window. It follows the anchor as anything scrolls and hides once the anchor leaves the window. Scroll the box to see each.",
    ],
}
