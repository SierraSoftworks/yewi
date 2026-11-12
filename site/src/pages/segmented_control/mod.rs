crate::page! {
    title: "Segmented control",
    category: Form,
    summary: "SegmentedControl and Segment: a compact row for switching between two to five views or modes of the same content.",
    examples: [
        basic => "Segmented control": "The control is controlled: it shows `value` and reports the chosen segment through `on_change`. One segment is always selected. `label` names it for assistive technology; Tab moves to it and the arrow keys change the selection.",
        icons => "With icons": "`icon` goes before the label. A segment with only an icon needs `label` for its accessible name.",
        sizes => "Sizes": "Sizes match `Button` and the text inputs, so a control lines up with them in a toolbar.",
        block => "Full width": "`block` fills the container and gives every segment the same width, for narrow panels and phone layouts.",
        states => "Disabled": "`disabled` on one `Segment` makes that option unavailable; on the control it disables all of them.",
        field => "In a field": "Inside a `Field` with `group`, the field's label names the control.",
    ],
}
