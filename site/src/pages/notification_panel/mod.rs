crate::page! {
    title: "NotificationPanel",
    category: Overlay,
    summary: "A notification panel is a bell with a count, which opens a list of the things that need attention.",
    examples: [
        basic => "In an app bar": "Give the bell a `count`, the most severe `tone` in the list, and a `label` that says what the number means. Each `NotificationItem` names a subject and says why it needs attention. Following a title link closes the panel.",
        list => "In a page": "`NotificationList` is the panel's list on its own, which you can use on an overview page or in a card. An item's `tone` colours the rule along its leading edge. `NotificationReason` gives each cause its own tone, and `meta` holds a short note such as a time.",
        empty => "Nothing to report": "When there are no items, the bell loses its count and the panel shows `empty`. Say what was checked, so that an empty panel reads as good news rather than as something that failed to load.",
        actions => "Acting on the list": "Use `footer` for an action on the whole list or a link to the full history. If you want the panel to close when that action completes, take control with `open` and `on_open_change`.",
    ],
}
