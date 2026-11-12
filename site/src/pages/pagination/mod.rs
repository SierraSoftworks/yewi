crate::page! {
    title: "Pagination",
    category: Navigation,
    summary: "Moves through a long list one page at a time.",
    examples: [
        basic => "Basic": "Pass the current `page` and the number of `pages`, and update the page from `on_change`. The first and last pages stay in reach and the row keeps its width as the reader moves.",
        ranges => "Siblings and boundaries": "`siblings` is how many numbers flank the current page and `boundaries` how many are pinned at each end.",
        compact => "Compact and unknown totals": "`compact` swaps the numbers for a short status. Without `pages`, as with a cursor-based API, only previous and next are offered and `has_next` says whether there is more.",
        sizes => "Sizes and disabled": "`Small` suits a table footer. `disabled` holds every control while a page loads.",
        links => "As links": "With `href` every control is a real link, so paging works without script and pages can be opened in a new tab.",
        router => "With a router": "With the `router` feature, `NavTarget::route` plus `query` keeps the page in the URL and moves between pages without reloading.",
    ],
}
