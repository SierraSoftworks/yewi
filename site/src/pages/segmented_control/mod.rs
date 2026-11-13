crate::page! {
    title: "SegmentedControl",
    category: Form,
    summary: "SegmentedControl and Segment give you a compact row for switching between two to five views or modes of the same content.",
    examples: [
        basic => "Segmented control": "The control is controlled, so it shows `value` and reports the chosen segment through `on_change`. One segment is always selected. `label` names the control for assistive technology. Tab moves focus to it, and the arrow keys change the selection.",
        icons => "With icons": "`icon` goes before the label. A segment that only has an icon needs a `label` for its accessible name.",
        sizes => "Sizes": "Sizes match `Button` and the text inputs, so the control lines up with them in a toolbar.",
        block => "Full width": "`block` makes the control fill its container and gives every segment the same width. This suits narrow panels and phone layouts.",
        states => "Disabled": "Setting `disabled` on one `Segment` makes that option unavailable. Setting it on the control disables all of them.",
        field => "In a field": "Inside a `Field` with `group`, the field's label names the control.",
    ],
}
