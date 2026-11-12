crate::page! {
    title: "TagPicker",
    category: Form,
    summary: "Choosing several of a list, or entering a list of free-form values: each one becomes a tag inside the field.",
    examples: [
        basic => "Tag picker": "The picker is controlled: it shows `values` and reports the whole new list through `on_change`. Typing narrows the options; a press or Enter adds or removes one and the list stays open for the next. Backspace in the empty field removes the last tag.",
        free => "Free-form tags": "With `allow_custom` and no options the field takes whatever is typed: Enter, a comma, or leaving the field adds it, and pasted text is split on commas. `mono` suits identifiers.",
        suggestions => "Suggestions": "With both, the options are suggestions and anything else can be added as typed. Text that names an option adds that option.",
        large => "Long lists": "The picker renders at most `max_results` options and counts the rest, so thousands of options cost no more than fifty.",
        states => "States": "Tags wrap onto as many lines as they need. A disabled picker cannot be focused and is not submitted. `invalid` marks a problem; inside a `Field` its `error` does this for you.",
        form => "In a form": "With a `name`, each value is submitted by a plain `<form>` through its own hidden input.",
    ],
}
