crate::page! {
    title: "Tooltip",
    category: Overlay,
    summary: "Describes an element in a small bubble while it is hovered or has keyboard focus.",
    examples: [
        basic => "Basic": "Wrap the element and give the tooltip its text. It shows after a short rest of the pointer, or at once on keyboard focus, and `Esc` dismisses it. A tooltip adds a description: an icon-only button still needs its own `aria-label`.",
        placement => "Placement": "`placement` is the preferred side and alignment. Near an edge of the window the tooltip flips to the opposite side and slides along it, so it is never cut off.",
        variants => "Variants": "`Surface` is a small card that follows the theme and suits rich content. `Dark` stays near-black in both themes. Both point at their element with an arrow unless `arrow` is turned off.",
        rich => "Rich content": "`tip` takes any phrasing content. The tooltip can be hovered itself, so its text can be selected and its links followed. Keep it short: content people must act on belongs in a popover.",
        unfocusable => "Icons, text and disabled controls": "Content that cannot take focus needs `focusable`, which makes the wrapper a tab stop so keyboard users can reach the tooltip. A disabled button cannot be focused either, and its tooltip is the place to say why.",
        controlled => "Controlled": "`open` takes over from the pointer and focus. Pass `Some(true)` to hold the tooltip open, for example to confirm an action, and `None` to hand control back.",
    ],
}
