crate::page! {
    title: "use_id",
    category: Utilities,
    summary: "Generated element ids for wiring labels, descriptions and popups to their controls.",
    examples: [
        nested => "Nested": "Each label is tied to its own input by a generated id, whatever depth the pair sits at. A server render and the client that hydrates it may number the pairs differently; hydration rewrites both ends together, so the wiring always holds. Pass an explicit `id` where the value itself must be stable.",
    ],
}
