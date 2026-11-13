crate::page! {
    title: "Badge",
    category: Basic,
    summary: "Badges show a count, or just a dot, to mark something as needing attention.",
    examples: [
        counts => "Counts": "An inline badge follows a label in a tab, a menu row or a heading. It's soft and neutral by default. Add a tone when the count means something, and `bold` when it asks the user to act.",
        limits => "Zero and large counts": "Counts above `max`, which is 99 by default, show as `99+`. A count of zero hides the badge unless you set `show_zero`.",
        attached => "Attached": "When you wrap a control in a badge, the badge sits on the control's top corner. Give it a `label` so a screen reader can say what the number means.",
        dot => "Dot": "Without a `count`, the badge is a dot. It says that something is new without saying how much. A dot needs a `label` so that it gets announced.",
        live => "Changing counts": "The badge appears, grows, caps and disappears as the count changes. When a change matters, announce it with a live region, as the status line in this example does.",
    ],
}
