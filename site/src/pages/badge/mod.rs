crate::page! {
    title: "Badge",
    category: Basic,
    summary: "A count, or a bare dot, that marks something as needing attention.",
    examples: [
        counts => "Counts": "Inline, a badge follows a label in a tab, menu row or heading. It is soft and neutral by default; add a tone when the count has a meaning, and `bold` when it asks for action.",
        limits => "Zero and large counts": "Counts above `max` (99 by default) show as `99+`. A count of zero hides the badge unless `show_zero` is set.",
        attached => "Attached": "Wrap a control and the badge sits on its top corner. Give it a `label` so a screen reader hears what the number means.",
        dot => "Dot": "Without a `count` the badge is a dot: something is new, without saying how much. A dot needs a `label` to be announced.",
        live => "Changing counts": "The badge appears, grows, caps and disappears as the count changes. Announce changes that matter with a live region, as the status line here does.",
    ],
}
