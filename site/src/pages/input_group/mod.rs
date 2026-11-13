crate::page! {
    title: "InputGroup",
    category: Form,
    summary: "InputGroup puts affixes, buttons and a dropdown inside one outlined field. SearchInput is a ready-made search box built from it.",
    examples: [
        affixes => "Prefix and suffix": "Affixes are plain text inside the field, such as the fixed part of a value before it or the unit after it. They aren't read out with the control, so repeat the unit in the label or hint.",
        button => "Attached button": "A `Button` in an affix becomes a small chip inside the field. Its variant still applies, so your one main action can be `Primary`. Use `kind: Submit` in a form so that Enter works too.",
        dropdown => "Attached dropdown": "A `Select` in an affix becomes a chip that opens its list under the field. For a handful of short options, such as a protocol, a unit or a scope, `NativeSelect` gives you a lighter native select with the same look.",
        menu => "Attached menu": "A `MenuButton` in an affix becomes a chip that opens a menu of actions on the value. Use `icon_only` when the field is narrow.",
        search => "Search input": "`SearchInput` reports every edit through `on_input`, which suits filtering a list that's already loaded. The clear button appears once there's text, and Escape clears it too.",
        search_bar => "Search bar": "When each search costs a request, run it from `on_search`, which fires on Enter. `suffix` takes a scope dropdown and a button, and `loading` swaps the icon for a spinner while results are on the way.",
        sizes => "Sizes": "`size` on the group sets the size of the control and the chips inside it. It matches `Button` and `TextInput`, so they line up in a toolbar.",
        states => "States": "The group takes its state from the control inside it. If you disable the control (or its `Field`), mark it invalid or make it read-only, the whole field follows.",
    ],
}
