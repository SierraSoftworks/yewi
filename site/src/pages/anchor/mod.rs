crate::page! {
    title: "Anchor positioning",
    category: Utilities,
    summary: "Anchor positioning places a floating element beside the element it belongs to and keeps it on screen. Tooltip and Popover are built on it.",
    examples: [
        basic => "A floating panel": "`use_anchored` takes the anchor, the floating element and whether it's open. Only render the floating element while it's open, and give it the `yi-anchored` class and `popover=\"manual\"` so that nothing clips it or stacks above it.",
        placement => "Placement": "Use `AnchorOptions` to set the preferred `Placement`, the gap from the anchor and the distance to keep from the window's edges. The side that's actually used is written to `data-yi-side` on the floating element, so you can style arrows and other details to match it.",
        collision => "Staying on screen": "When there isn't room on the preferred side, the element flips to the opposite one, and it slides along the anchor to stay inside the window. It follows the anchor as anything scrolls, and it hides once the anchor leaves the window. Scroll the box to see each of these.",
    ],
}
