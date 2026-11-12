crate::page! {
    title: "NotificationPanel",
    category: Overlay,
    summary: "A bell with a count that opens the list of what needs attention.",
    examples: [
        basic => "In an app bar": "Give the bell a `count` and the most severe `tone` in the list, and a `label` that says what the number means. Each `NotificationItem` names a subject and why it needs attention; following a title link closes the panel.",
        list => "In a page": "`NotificationList` is the panel's list on its own, for an overview page or a card. An item's `tone` colours the rule on its leading edge; `NotificationReason` gives each cause its own tone, and `meta` holds a short note such as a time.",
        empty => "Nothing to report": "With no items the bell loses its count and the panel shows `empty`. Say what was checked, so that an empty panel reads as good news and not as a failure to load.",
        actions => "Acting on the list": "`foot` holds an action for the whole list or a link to the full history. Take control with `open` and `on_open_change` to close the panel when the action completes.",
    ],
}
