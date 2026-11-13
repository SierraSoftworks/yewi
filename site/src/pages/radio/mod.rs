crate::page! {
    title: "Radio",
    category: Form,
    summary: "RadioGroup and Radio let people pick one choice from a few options, when all the options are worth seeing at once.",
    examples: [
        basic => "Radio group": "The group is controlled, so it shows `value` and reports the chosen option through `on_change`. Put it in a `Field` with `group` to give it a label and hint. Tab moves focus to the group, and the arrow keys change the selection.",
        descriptions => "Options with descriptions": "Use `description` to explain what choosing an option means. A single `disabled` option stays visible, but the keyboard skips it.",
        horizontal => "Horizontal": "`Orientation::Horizontal` suits two or three short labels. The options wrap when the row runs out of room.",
        states => "States and sizes": "Setting `disabled` on the group disables every option. `invalid` marks every ring, and inside a `Field` the field's `error` does this for you.",
        form => "Required in a form": "When no option is a safe default, start the group with nothing selected. The field's error is announced and describes the group.",
    ],
}
