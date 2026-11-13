crate::page! {
    title: "Pagination",
    category: Navigation,
    summary: "Pagination lets people move through a long list one page at a time.",
    examples: [
        basic => "Basic": "Pass the current `page` and the number of `pages`, and update the page from `on_change`. The first and last pages are always in reach, and the row keeps the same width as the reader moves through it.",
        ranges => "Siblings and boundaries": "`siblings` sets how many page numbers appear on each side of the current page, and `boundaries` sets how many stay pinned at each end.",
        compact => "Compact and unknown totals": "`compact` replaces the page numbers with a short status. If you don't know the total, as with a cursor-based API, leave out `pages`. Only previous and next are offered then, and `has_next` says whether there's more.",
        sizes => "Sizes and disabled": "`Small` suits a table footer. `disabled` holds every control while a page is loading.",
        links => "As links": "With `href`, every control is a real link, so paging works without script and people can open pages in a new tab.",
        router => "With a router": "With the `router` feature, use `NavTarget::route` with `query` to keep the page number in the URL and move between pages without a reload.",
    ],
}
