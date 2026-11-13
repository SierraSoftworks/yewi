crate::page! {
    title: "NativeSelect",
    category: Form,
    summary: "NativeSelect is the platform's own select in the look of the other controls. It needs no client runtime, so it is the select for server-rendered pages without wasm.",
    examples: [
        basic => "Native select": "The browser opens its own picker, so it works on a page that ships no wasm and feels at home on a phone. Options are plain text; for icons, descriptions or filtering use `Select` or `Combobox`.",
        form => "In a form without a client runtime": "With a `name`, a plain `<form>` submits the choice and the server renders the next page with it as `value`. `placeholder` with `required` makes the browser ask for a choice before it submits.",
        states => "Sizes and states": "Sizes match `Button` and `TextInput`. `fit` sizes it to its content for toolbars, where `label` names it in place of a `Field`.",
        group => "In an input group": "In an `InputGroup` affix it becomes a chip: a unit, a protocol, a search scope.",
    ],
}
