crate::page! {
    title: "Tooltip",
    category: Overlay,
    summary: "A tooltip describes an element in a small bubble while the element is hovered or has keyboard focus.",
    examples: [
        basic => "Basic": "Wrap the element and give the tooltip its text. It appears once the pointer has rested for a moment, or straight away on keyboard focus, and `Esc` dismisses it. A tooltip only adds a description, so an icon-only button still needs its own `aria-label`.",
        placement => "Placement": "Use `placement` to choose which side the tooltip prefers and how it lines up. If there isn't room near the edge of the window, it flips to the other side and slides along it so it's never cut off.",
        variants => "Variants": "`Surface` is a small card that follows the theme and suits rich content. `Dark` stays near-black in both themes. Both point at their element with an arrow unless you turn `arrow` off.",
        rich => "Rich content": "`tip` takes any phrasing content. People can hover over the tooltip itself, so they can select its text and follow its links. Keep it short, though, because content people need to act on belongs in a popover.",
        unfocusable => "Icons, text and disabled controls": "Content that can't take focus needs `focusable`, which makes the wrapper a tab stop so keyboard users can reach the tooltip. A disabled button can't be focused either, and its tooltip is the place to explain why it's disabled.",
        controlled => "Controlled": "`open` takes over from the pointer and focus. Pass `Some(true)` to hold the tooltip open, for example to confirm an action, and pass `None` to hand control back.",
    ],
}
