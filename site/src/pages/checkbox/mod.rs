crate::page! {
    title: "Checkbox",
    category: Form,
    summary: "Checkboxes let the user tick a box. Use one for a single yes-or-no choice, or several to let the user pick any number of options from a list.",
    examples: [
        basic => "Checkbox": "The checkbox is controlled, so it shows `checked` and reports each toggle through `on_change`. Clicking the label toggles it too.",
        descriptions => "Several options with descriptions": "Each checkbox in a list is independent. `description` adds a quieter line that's read out with the option, and a `Field` with `group` gives the set a name.",
        select_all => "Select all": "`indeterminate` shows a dash when only some of the items a checkbox stands for are selected. Clicking it reports `true`, so a mixed parent selects everything.",
        states => "States and sizes": "Disabled checkboxes can't be focused and aren't submitted. `invalid` marks a problem, and inside a `Field` the field's `error` does this for you. `Small` suits tables and compact toolbars.",
        field => "Required in a form": "A single checkbox in a `Field` picks up the field's error and required state. It's a native input with a `name`, so a plain `<form>` submits it.",
        bare => "Without a visible label": "In a selection column, the context makes the meaning clear without a visible label. Set `label` anyway so that the checkbox still has an accessible name.",
    ],
}
