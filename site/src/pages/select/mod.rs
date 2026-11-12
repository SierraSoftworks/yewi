crate::page! {
    title: "Select",
    category: Form,
    summary: "Select and Combobox: choosing one of a list. Select shows a short list with the choice check-marked; Combobox narrows a long one as the user types.",
    examples: [
        basic => "Select": "The select is controlled: it shows the option whose value is `value` and reports a new choice through `on_change`. Arrow keys move, Enter or Space chooses, Esc closes, and typing jumps to the option starting with those letters.",
        grouped => "Groups, icons and descriptions": "Consecutive options with the same `group` share a heading. An option can carry an `icon`, a muted `description`, and be `disabled`, which keeps it visible but skips it from the keyboard.",
        states => "Sizes and states": "Sizes match `Button` and `TextInput`. A disabled select cannot be focused and is not submitted. `invalid` marks a problem; inside a `Field` its `error` does this for you.",
        toolbar => "In a toolbar": "A select fills its container, like a text input. `fit` sizes it to its content for toolbars and filter rows, where `label` names it in place of a `Field`.",
        combobox => "Combobox": "Typing narrows the list to options whose label or description contains every word typed. Leaving the field without choosing puts the current choice back.",
        custom => "Accepting new values": "`allow_custom` treats the options as suggestions: Enter, or leaving the field, reports the text as typed.",
        large => "Long lists": "A combobox renders at most `max_results` options and counts the rest, so thousands of options cost no more than fifty. For options that live on a server, load them from `on_input`, set `filter={false}` and show `loading` while they arrive.",
        form => "In a form": "With a `name`, the chosen value is submitted by a plain `<form>` through a hidden input.",
    ],
}
