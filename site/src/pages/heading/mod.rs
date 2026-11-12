crate::page! {
    title: "Heading",
    category: Basic,
    summary: "Titles for pages, sections and panels. The level sets the document outline; the size sets the look.",
    examples: [
        levels => "Levels": "Each level has a default size, and the weight falls as the size rises: light page titles, regular sections, medium panel titles, semibold only at body size.",
        sizes => "Size apart from level": "Pick `level` for structure and `size` for looks. A card title deep in a page stays an `H3` for the outline while it is drawn at panel size.",
        in_context => "A page header": "A title with its supporting line, then sections. `truncate` keeps a long generated title on one line.",
    ],
}
