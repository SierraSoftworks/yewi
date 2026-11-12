crate::page! {
    title: "PageHeader",
    category: Layout,
    summary: "The title row that opens a page: a light, large title with its context above and beside it and the page's actions at the end.",
    examples: [
        basic => "Title, subtitle and actions": "The title is the page's `h1`. Keep one primary action; the row wraps on a narrow page so the actions drop under the title.",
        context => "Breadcrumbs and meta": "`breadcrumbs` sits above the title for the way back. `meta` sits beside it for a short status. A detail page usually has both.",
        content => "With content beneath": "Children span the full width under the header, for a filter bar or tabs that belong to the whole page. `divided` rules the header off from content that is not carded.",
    ],
}
