crate::page! {
    title: "use_id",
    category: Utilities,
    summary: "`use_id` generates element ids, so you can wire labels, descriptions and popups to their controls.",
    examples: [
        nested => "Nested": "Each label is tied to its own input by a generated id, however deeply the pair is nested. A server render and the client that hydrates it may number the pairs differently, but hydration rewrites both ends together, so the wiring always holds. Pass an explicit `id` when the value itself must stay stable.",
    ],
}
