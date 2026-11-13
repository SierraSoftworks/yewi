crate::page! {
    title: "Panel",
    category: Layout,
    summary: "A section inside a card. Sibling panels stack flush and are told apart by alternating tonal fills, not borders.",
    examples: [
        siblings => "Sibling panels": "Each panel is one aspect of the card's subject, with its own heading and actions. The fill alternates automatically.",
        fills => "Fills": "Set `fill` to opt out of the alternation: `Plain` for an introduction that belongs to the card itself, `Soft` or `Strong` to fix a fill.",
        collapsible => "Collapsible": "A collapsible panel is a native disclosure, so it opens with the mouse, Enter or Space, and works before the page is interactive. Keep rarely needed detail `collapsed`.",
    ],
}
