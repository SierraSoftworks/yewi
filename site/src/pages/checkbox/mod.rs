crate::page! {
    title: "Checkbox",
    category: Form,
    summary: "A box the user ticks: one independent yes-or-no choice, or any number of options from a list.",
    examples: [
        basic => "Checkbox": "The checkbox is controlled: it shows `checked` and reports each toggle through `on_change`. The label is part of the click target.",
        descriptions => "Several options with descriptions": "Each checkbox in a list is independent. `description` adds a quieter line that is read out with the option. A `Field` with `group` names the set.",
        select_all => "Select all": "`indeterminate` shows a dash when only some of the items a checkbox stands for are selected. Clicking it reports `true`, so a mixed parent selects everything.",
        states => "States and sizes": "Disabled checkboxes cannot be focused and are not submitted. `invalid` marks a problem; inside a `Field` its `error` does this for you. `Small` suits tables and compact toolbars.",
        field => "Required in a form": "A single checkbox in a `Field` takes the field's error and required state. It is a native input with a `name`, so a plain `<form>` submits it.",
        bare => "Without a visible label": "In a selection column the meaning comes from context. Give `label` so the checkbox still has an accessible name.",
    ],
}
