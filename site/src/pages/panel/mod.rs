crate::page! {
    title: "Panel",
    category: Layout,
    summary: "A panel is a section inside a card. Sibling panels stack flush against each other, and you tell them apart by their alternating tonal fills rather than by borders.",
    examples: [
        siblings => "Sibling panels": "Each panel covers one aspect of the card's subject and has its own heading and actions. The fill alternates automatically.",
        fills => "Fills": "Set `fill` to opt out of the alternation. Use `Plain` for an introduction that belongs to the card itself, or `Soft` or `Strong` to fix the fill.",
        collapsible => "Collapsible": "A collapsible panel is a native disclosure, so it opens with the mouse, Enter or Space, and it works before the page becomes interactive. Keep detail that people rarely need `collapsed`.",
    ],
}
