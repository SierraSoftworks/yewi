crate::page! {
    title: "FileDrop",
    category: Form,
    summary: "FileDrop takes files by drag and drop, by click or from the keyboard, and hands you the browser's own file objects.",
    examples: [
        basic => "Choosing one file": "The zone is the label of a real file input, so it opens the picker on click, Enter or Space, and it takes a drop anywhere inside it. It reports the file but keeps nothing, so show what was chosen next to it.",
        multiple => "Several files with limits": "`multiple` lets the zone take any number of files. `accept` and `max_size` apply to dropped files as well as the picker, and `on_reject` receives whatever was turned away along with the reason, so you can tell the user why.",
        field => "In a field": "Inside a `Field`, the zone takes the field's label, hint, error and required state like any other control.",
        upload => "While uploading": "Set `busy` while the upload is in flight. The zone shows a spinner and `busy_label`, and it won't take new files until `busy` is cleared.",
        states => "States": "Here are invalid, disabled and busy zones. Leaving out the icon suits tight layouts.",
        sizes => "Sizes": "`Size::Small` is a single row, for toolbars and dense forms. `Size::Large` suits a page whose main purpose is the upload.",
    ],
}
