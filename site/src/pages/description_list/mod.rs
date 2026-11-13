crate::page! {
    title: "DescriptionList",
    category: Data,
    summary: "Description lists show labelled facts about one thing, such as a host, a certificate or a release. They render a native description list, so terms and values stay paired for assistive technology.",
    examples: [
        basic => "Basic": "Each `DescriptionItem` pairs a `term` with its value. The terms share one column, so the values line up however long the terms are.",
        rich_values => "Rich values": "A value can be any content, including links, icons and several lines. `mono` suits paths, ids and hashes. An item with no value shows a muted dash instead of collapsing.",
        ruled => "Ruled": "`ruled` adds a hairline between rows. Use it for long lists that people scan by term, such as settings or metrics.",
        stacked => "Stacked": "`DescriptionLayout::Stacked` puts each term above its value and flows the items into as many columns as fit. It works well for summary strips under a page title. Set `min` or `columns` to control the columns, and `wide` for an item that needs the whole row.",
        narrow => "In narrow spaces": "An inline list measures its own width rather than the viewport's. Below 20rem each term moves above its value, so the same list works in a page, a sidebar or a popover.",
    ],
}
