crate::page! {
    title: "Heading",
    category: Basic,
    summary: "Headings title pages, sections and panels. The level sets the document outline, and the size sets how it looks.",
    examples: [
        levels => "Levels": "Each level has a default size, and the weight gets lighter as the size grows. Page titles are light, sections are regular, panel titles are medium, and only body-size headings are semibold.",
        sizes => "Size apart from level": "Pick `level` for structure and `size` for looks. A card title deep in a page stays an `H3` in the outline while it's drawn at panel size.",
        in_context => "A page header": "This page header has a title with its supporting line, followed by sections. `truncate` keeps a long generated title on one line.",
    ],
}
