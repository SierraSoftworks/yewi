crate::page! {
    title: "ToggleSwitch",
    category: Form,
    summary: "A toggle switch is an on/off control for a setting that takes effect as soon as you flip it.",
    examples: [
        basic => "Switch": "The switch is controlled. It shows `checked` and reports each flip through `on_change`. Label it with what it controls rather than with its state.",
        settings => "Settings list": "In an inline `Field`, the field's label names the switch and the hint describes it. This is the usual layout for a settings page.",
        descriptions => "With a description": "`description` adds a quieter line under the label, and screen readers read it out along with the label.",
        states => "States and sizes": "A disabled switch still shows its state. The switch's size follows the text size of its label.",
    ],
}
