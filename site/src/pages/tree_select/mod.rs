crate::page! {
    title: "TreeSelect",
    category: Form,
    summary: "Choosing one entry out of a large hierarchy: searched by typing, walked by opening branches. For a flat list use Select or Combobox.",
    examples: [
        basic => "Tree select": "The select is controlled: it shows the entry whose value is `value` and reports a new choice through `on_change`. Each entry is placed by its `path`; a step that was never listed itself becomes a branch that can be opened but not chosen. The panel opens on the level holding the current choice.",
        search => "Search and details": "Typing searches the whole tree and each result says where it lives. A `detail` is a code shown beside the name and searched with it. Entries that are themselves choices and also hold others show an arrow: a press on the row chooses it, a press on the arrow (or the right arrow key) opens it.",
        media => "Pictures": "An entry can carry an `icon`, or a `media` key that `render_media` turns into a picture, for catalogues where each entry is a symbol.",
        custom => "Clearing and custom values": "`clear_label` adds a first row that returns to nothing chosen. `custom` lets text that names no entry be used as it stands: return the value it stands for, or `None` when it is not well formed. A value the tree does not hold is shown as it is.",
        large => "Large trees": "Only one level, or the best `max_results` matches, is rendered at a time, so thousands of entries cost no more than forty.",
        states => "Sizes and states": "Sizes match `Select`. `fit` sizes the control to its content for toolbars, `invalid` marks a problem, and with a `name` the value is submitted by a plain `<form>`.",
    ],
}
