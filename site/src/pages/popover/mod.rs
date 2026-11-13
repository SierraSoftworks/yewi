crate::page! {
    title: "Popover",
    category: Overlay,
    summary: "A floating surface attached to the element that opens it, for detail too rich for a tooltip and too small for a dialog.",
    examples: [
        basic => "Basic": "Wrap the trigger and pass the `content`. Pressing the trigger toggles the popover; `Esc`, a press outside or moving focus away closes it. `header` and `footer` are optional rows around the content: a short status above, a muted note below.",
        hover => "Detail on hover": "`PopoverTrigger::Hover` shows the popover while the trigger or the popover itself is hovered, and on keyboard focus. Use it for detail people only read, and give triggers that cannot take focus `focusable`. The popover adds no layout of its own: the two-column rows here are the caller's content.",
        placement => "Placement": "`placement` is the preferred side and alignment. Near an edge of the window the popover flips to the opposite side and slides along it, and the arrow keeps pointing at the trigger.",
        actions => "Content to act on": "Controls inside the popover follow the trigger in the tab order. Take control with `open` and `on_open_change` to close the popover from inside it, for example when an action completes.",
        focus => "Under an input": "`PopoverTrigger::Focus` keeps the popover open while focus is inside the trigger or the popover. `arrow={false}` and `flush` suit a list that runs edge to edge; the list itself, and its ARIA roles, belong to the caller.",
        nested => "Nested": "A popover can open another. `Esc` closes the innermost first, and a press inside the inner popover does not close the outer one.",
    ],
}
