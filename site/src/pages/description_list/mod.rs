crate::page! {
    title: "DescriptionList",
    category: Data,
    summary: "Labelled facts about one thing: a host, a certificate, a release. A native description list, so terms and values stay paired for assistive technology.",
    examples: [
        basic => "Basic": "Each `DescriptionItem` pairs a `term` with its value. Terms share one column, so values start on the same line however long the terms are.",
        rich_values => "Rich values": "A value is any content: links, icons, several lines. `mono` suits paths, ids and hashes, and an item with no value shows a muted dash instead of collapsing.",
        ruled => "Ruled": "`ruled` adds a hairline between rows. Use it for long lists that are scanned by term, such as settings or metrics.",
        stacked => "Stacked": "`DescriptionLayout::Stacked` puts each term above its value and flows items into as many columns as fit: summary strips under a page title. Set `min` or `columns` to control the columns, and `wide` for an item that needs the whole row.",
        narrow => "In narrow spaces": "An inline list measures its own width, not the viewport. Below 20rem each term moves above its value, so the same list works in a page, a sidebar or a popover.",
    ],
}
