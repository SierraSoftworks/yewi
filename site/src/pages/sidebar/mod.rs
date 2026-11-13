crate::page! {
    title: "Sidebar",
    category: Navigation,
    summary: "An application's destinations as a vertical list, optionally under group labels.",
    examples: [
        basic => "Destinations": "One `SidebarItem` per destination, each with an icon or none of them. Mark where the user is with `current`: the row lifts and its icon takes the accent. An item with `href` is a link; without one it is a button.",
        routes => "With a router": "With the `router` feature, give `href` a route as `NavTarget::route(..)`. The item is a real link that navigates without a page load, and the item for the route being shown is marked `current` by itself.",
        groups => "Groups": "`SidebarGroup` gathers related destinations under a label once there are too many to scan as one list. Grouped lists usually go without icons.",
        nested => "Nested items, counts and a footer": "`nested` indents a set beneath the item above it, `trailing` puts a count or a status at the end of a row, and `footer` pins links that leave the app to the bottom. `NavTarget::external` opens a new tab and says so.",
        group_labels => "Group label style": "Labels are small uppercase eyebrows by default. The `--yi-sidebar-heading-*` tokens restyle them, here as sentence case.",
    ],
}
