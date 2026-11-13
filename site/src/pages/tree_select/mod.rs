crate::page! {
    title: "TreeSelect",
    category: Form,
    summary: "A tree select lets you choose one entry from a large hierarchy, either by typing to search or by opening branches. For a flat list, use Select or Combobox instead.",
    examples: [
        basic => "Tree select": "The select is controlled. It shows the entry whose value is `value` and reports a new choice through `on_change`. Each entry is placed by its `path`, and a step in a path that was never listed itself becomes a branch that can be opened but not chosen. The panel opens on the level that holds the current choice.",
        search => "Search and details": "Typing searches the whole tree, and each result says where it lives. A `detail` is a code that's shown beside the name and searched along with it. Entries that can be chosen and also hold other entries show an arrow. Pressing the row chooses the entry, while pressing the arrow (or the right arrow key) opens it.",
        media => "Pictures": "An entry can carry an `icon`, or a `media` key that `render_media` turns into a picture. This suits catalogues where each entry is a symbol.",
        custom => "Clearing and custom values": "`clear_label` adds a first row that goes back to having nothing chosen. `custom` lets text that doesn't name any entry be used as it stands. Return the value the text stands for, or `None` when it isn't well formed. A value that the tree doesn't hold is shown as it is.",
        large => "Large trees": "Only one level, or the best `max_results` matches, is rendered at a time, so thousands of entries cost no more than forty.",
        states => "Sizes and states": "Sizes match `Select`. `fit` sizes the control to its content, which suits toolbars. `invalid` marks a problem, and with a `name` the value is submitted by a plain `<form>`.",
    ],
}
