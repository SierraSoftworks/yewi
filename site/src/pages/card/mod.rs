crate::page! {
    title: "Card",
    category: Layout,
    summary: "A lifted surface that groups one subject's worth of content, with an optional heading row and footer.",
    examples: [
        basic => "Title, subtitle and actions": "A card holds one subject. Put the action the card exists for in `actions`, at the end of the heading row, so it is never lost among the content it acts on.",
        panels => "With panels": "A card whose body is made of `Panel`s lays them out flush: full width, no gap or border, told apart by alternating tonal fills.",
        header => "Header styles": "`Plain` lets the heading flow into the body. `Ruled` and `Tonal` set it apart, which suits cards whose body is a table or a form.",
        footer => "Footer": "The footer is a closing strip for the buttons of a form, or a line summarising the card.",
        grid => "In a grid": "Cards carry no outer margin. A `Grid` or `Stack` owns the space between them, and cards in a row stretch to the same height.",
    ],
}
