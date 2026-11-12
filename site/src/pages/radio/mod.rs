crate::page! {
    title: "Radio",
    category: Form,
    summary: "RadioGroup and Radio: one choice out of a few options that are all worth seeing at once.",
    examples: [
        basic => "Radio group": "The group is controlled: it shows `value` and reports the chosen option through `on_change`. A `Field` with `group` gives it a label and hint. Tab moves to the group and the arrow keys change the selection.",
        descriptions => "Options with descriptions": "`description` explains what choosing an option means. A single `disabled` option stays visible but is skipped by the keyboard.",
        horizontal => "Horizontal": "`RadioOrientation::Horizontal` suits two or three short labels. It wraps when the row runs out of room.",
        states => "States and sizes": "`disabled` on the group disables every option. `invalid` marks every ring; inside a `Field` its `error` does this for you.",
        form => "Required in a form": "A group starts with nothing selected when no option is a safe default. The field's error is announced and describes the group.",
    ],
}
