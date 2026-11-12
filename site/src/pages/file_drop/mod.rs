crate::page! {
    title: "FileDrop",
    category: Form,
    summary: "Takes files by drag and drop, by click or from the keyboard, and hands the browser's own file objects to the caller.",
    examples: [
        basic => "Choosing one file": "The zone is the label of a real file input, so it opens the picker on click, Enter or Space and takes a drop anywhere inside it. It reports the file and keeps nothing: show what was chosen next to it.",
        multiple => "Several files with limits": "`multiple` takes any number of files. `accept` and `max_size` apply to dropped files as well as the picker, and `on_reject` receives whatever was turned away with the reason, so the caller can say why.",
        field => "In a field": "Inside a `Field` the zone takes the field's label, hint, error and required state like any other control.",
        upload => "While uploading": "Set `busy` while the upload is in flight. The zone shows a spinner and `busy_label`, and takes no new files until it is cleared.",
        states => "States": "Invalid, disabled and busy zones. A zone without an icon suits tight layouts.",
        sizes => "Sizes": "`Size::Small` is a single row for toolbars and dense forms. `Size::Large` suits a page whose main purpose is the upload.",
    ],
}
