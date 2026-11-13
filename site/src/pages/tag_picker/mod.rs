crate::page! {
    title: "TagPicker",
    category: Form,
    summary: "A tag picker lets you choose several items from a list, or enter a list of free-form values. Each value becomes a tag inside the field.",
    examples: [
        basic => "Tag picker": "The picker is controlled. It shows `values` and reports the whole new list through `on_change`. Typing narrows the options, and clicking an option or pressing Enter adds or removes it, while the list stays open for the next one. Pressing Backspace in an empty field removes the last tag.",
        free => "Free-form tags": "With `allow_custom` and no options, the field accepts whatever you type. Pressing Enter, typing a comma or leaving the field adds the value, and pasted text is split on commas. `mono` suits identifiers.",
        suggestions => "Suggestions": "When you set both, the options act as suggestions and anything else can still be added as typed. Text that names an option adds that option.",
        large => "Long lists": "The picker renders at most `max_results` options and shows a count of the rest, so thousands of options cost no more than fifty.",
        states => "States": "Tags wrap onto as many lines as they need. A disabled picker can't be focused and isn't submitted with the form. `invalid` marks a problem, and inside a `Field` the field's `error` does this for you.",
        form => "In a form": "When you give it a `name`, each value is submitted with a plain `<form>` through its own hidden input.",
    ],
}
