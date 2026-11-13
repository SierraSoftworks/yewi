crate::page! {
    title: "Sidebar",
    category: Navigation,
    summary: "A sidebar lists an application's destinations vertically, optionally under group labels.",
    examples: [
        basic => "Destinations": "Add one `SidebarItem` per destination. Either give every item an icon or give none of them one. Mark where the user is with `current`, which lifts the row and gives its icon the accent colour. An item with `href` is a link, and without one it's a button.",
        routes => "With a router": "With the `router` feature, give `href` a route as `NavTarget::route(..)`. The item is then a real link that navigates without a page load, and the item for the route being shown is marked `current` automatically.",
        groups => "Groups": "Once there are too many destinations to scan as one list, use `SidebarGroup` to gather related ones under a label. Grouped lists usually go without icons.",
        nested => "Nested items, counts and a footer": "`nested` indents a set of items beneath the item above it. `trailing` puts a count or a status at the end of a row, and `footer` pins links that leave the app to the bottom. `NavTarget::external` opens a new tab and tells the user it will.",
        group_labels => "Group label style": "By default, group labels are small uppercase eyebrows. You can restyle them with the `--yi-sidebar-heading-*` tokens, as this example does to make them sentence case.",
    ],
}
