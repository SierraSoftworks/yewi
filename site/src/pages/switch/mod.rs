crate::page! {
    title: "Switch",
    category: Form,
    summary: "An on/off control for a setting that takes effect as soon as it is flipped.",
    examples: [
        basic => "Switch": "The switch is controlled: it shows `checked` and reports each flip through `on_change`. Label it with what it controls, not with its state.",
        settings => "Settings list": "In an inline `Field` the field's label names the switch, and the hint describes it. This is the usual shape of a settings page.",
        descriptions => "With a description": "`description` adds a quieter line under the label, announced with the switch.",
        states => "States and sizes": "A disabled switch keeps showing its state. Sizes follow the text size of the label.",
    ],
}
