crate::page! {
    title: "Select",
    category: Form,
    summary: "Select and Combobox let people choose one item from a list. Select shows a short list with the current choice check-marked, and Combobox narrows a long list as the user types.",
    examples: [
        basic => "Select": "The select is controlled, so it shows the option whose value is `value` and reports a new choice through `on_change`. The arrow keys move through the options, Enter or Space chooses one and Esc closes the list. Typing jumps to the option that starts with those letters.",
        grouped => "Groups, icons and descriptions": "Consecutive options with the same `group` share a heading. An option can have an `icon` and a muted `description`. It can also be `disabled`, which keeps it visible but means the keyboard skips it.",
        states => "Sizes and states": "Sizes match `Button` and `TextInput`. A disabled control can't be focused and isn't submitted. `invalid` marks a problem, and inside a `Field` the field's `error` does this for you.",
        toolbar => "In a toolbar": "Like a text input, a select fills its container. In toolbars and filter rows, use `fit` to size it to its content, and use `label` to name it since there's no `Field`.",
        combobox => "Combobox": "Typing narrows the list to the options whose label or description contains every word you've typed. If you leave the field without choosing, the current choice is put back.",
        custom => "Accepting new values": "`allow_custom` treats the options as suggestions. Pressing Enter or leaving the field reports the text exactly as typed.",
        large => "Long lists": "A combobox renders at most `max_results` options and counts the rest, so thousands of options cost no more than fifty. When the options live on a server, load them from `on_input`, set `filter={false}` and show `loading` while they arrive.",
        form => "In a form": "When you give it a `name`, a plain `<form>` submits the chosen value through a hidden input.",
    ],
}
