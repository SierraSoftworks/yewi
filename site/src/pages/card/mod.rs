crate::page! {
    title: "Card",
    category: Layout,
    summary: "Cards are raised surfaces that group the content about one subject. They can have a heading row and a footer.",
    examples: [
        basic => "Title, subtitle and actions": "A card holds one subject. Put the action the card exists for in `actions`, at the end of the heading row, so it never gets lost among the content it acts on.",
        panels => "With panels": "When a card's body is made of `Panel`s, the card lays them out flush. They run full width with no gap or border, and alternating tonal fills tell them apart.",
        header => "Header styles": "`Plain` lets the heading flow into the body. `Ruled` and `Tonal` set it apart, which suits cards whose body is a table or a form.",
        footer => "Footer": "The footer is a closing strip that holds a form's buttons or a line that summarises the card.",
        grid => "In a grid": "Cards have no outer margin, so a `Grid` or `Stack` controls the space between them. Cards in the same row stretch to the same height.",
    ],
}
