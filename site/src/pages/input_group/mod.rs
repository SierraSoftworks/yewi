crate::page! {
    title: "Input group",
    category: Form,
    summary: "InputGroup puts affixes, buttons and a dropdown inside one outlined field. SearchInput is the ready-made search box built from it.",
    examples: [
        affixes => "Prefix and suffix": "Affixes are plain text inside the field: the fixed part of a value before it, the unit after it. They are not read out with the control, so repeat the unit in the label or hint.",
        button => "Attached button": "A `Button` in an affix becomes a small chip inside the field. Its variant still applies, so the one main action can be `Primary`. Use `kind: Submit` in a form and Enter works too.",
        dropdown => "Attached dropdown": "`InputGroupSelect` is a native select drawn as a chip, for a handful of short options that qualify the value: a protocol, a unit, a scope.",
        search => "Search input": "`SearchInput` reports every edit through `on_input`, which suits filtering a list that is already loaded. The clear button appears once there is text, and Escape clears it too.",
        search_bar => "Search bar": "For a search that costs a request, run it from `on_search`, which fires on Enter. `suffix` takes a scope dropdown and a button, and `loading` swaps the icon for a spinner while results are on the way.",
        sizes => "Sizes": "`size` on the group sets the control and the chips inside it, and matches `Button` and `TextInput` so they line up in a toolbar.",
        states => "States": "The group takes its state from the control inside: disable, mark invalid or make read-only the control (or its `Field`) and the whole field follows.",
    ],
}
