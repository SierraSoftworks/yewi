crate::page! {
    title: "PageHeader",
    category: Layout,
    summary: "The page header is the title row at the top of a page. It has a large, light title with its context above and beside it, and the page's actions at the end.",
    examples: [
        basic => "Title, subtitle and actions": "The title is the page's `h1`. Keep to one primary action. On a narrow page the header reads top to bottom: the title, the subtitle, then the actions.",
        context => "Breadcrumbs and meta": "`breadcrumbs` sits above the title and shows the way back. `meta` sits beside the title for a short status. A detail page usually has both.",
        content => "With content beneath": "Children span the full width under the header, which suits a filter bar or tabs that belong to the whole page. `divided` adds a rule between the header and content that isn't in cards.",
    ],
}
