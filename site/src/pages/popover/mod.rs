crate::page! {
    title: "Popover",
    category: Overlay,
    summary: "A popover is a floating surface attached to the element that opens it. Use it for detail that's too rich for a tooltip but too small for a dialog.",
    examples: [
        basic => "Basic": "Wrap the trigger and pass the `content`. Pressing the trigger toggles the popover, and `Esc`, a press outside or moving focus away closes it. `head` and `foot` are optional rows around the content, for a short status above or a muted note below.",
        hover => "Detail on hover": "`PopoverTrigger::Hover` shows the popover while the trigger or the popover itself is hovered, and when the trigger has keyboard focus. Use it for detail that people only read. If a trigger can't take focus, give it `focusable`. The popover doesn't add any layout of its own, so the two-column rows here are the caller's content.",
        placement => "Placement": "Use `placement` to choose which side the popover prefers and how it lines up. If there isn't room near the edge of the window, it flips to the other side and slides along it, and the arrow keeps pointing at the trigger.",
        actions => "Content to act on": "Controls inside the popover come after the trigger in the tab order. To close the popover from inside it, for example when an action completes, take control with `open` and `on_open_change`.",
        focus => "Under an input": "`PopoverTrigger::Focus` keeps the popover open while focus is inside the trigger or the popover. `arrow={false}` and `flush` suit a list that runs edge to edge. The list itself and its ARIA roles are up to the caller.",
        nested => "Nested": "A popover can open another popover. `Esc` closes the innermost one first, and pressing inside the inner popover doesn't close the outer one.",
    ],
}
