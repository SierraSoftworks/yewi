crate::page! {
    title: "Breadcrumbs",
    category: Navigation,
    summary: "Shows where the current page sits in a hierarchy and links back up it.",
    examples: [
        basic => "Basic": "Each `Crumb` with an `href` is a link. The last crumb has none: it is the page being shown and is announced as such.",
        separators => "Separators": "A slash by default, or a chevron. Separators are drawn in CSS and are never read out.",
        icons_and_long_labels => "Icons and long labels": "An icon can mark the root. Long labels are truncated at `--yi-breadcrumbs-crumb-max`, and the trail wraps when it runs out of room.",
        router => "With a router": "With the `router` feature, `NavTarget::route` follows a `yew-router` route without reloading, and `query` carries state such as active filters back up the trail.",
    ],
}
