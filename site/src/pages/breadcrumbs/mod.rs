crate::page! {
    title: "Breadcrumbs",
    category: Navigation,
    summary: "Breadcrumbs show where the current page sits in a hierarchy and link back up to the pages above it.",
    examples: [
        basic => "Basic": "Each `Crumb` with an `href` is a link. The last crumb has no `href` because it's the page being shown, and it's announced as the current page.",
        separators => "Separators": "The separator is a slash by default, or you can use a chevron. Separators are drawn in CSS, so they're never read out.",
        icons_and_long_labels => "Icons and long labels": "An icon can mark the root of the trail. Long labels are truncated at `--yi-breadcrumbs-crumb-max`, and the trail wraps when it runs out of room.",
        router => "With a router": "With the `router` feature, `NavTarget::route` follows a `yew-router` route without reloading the page. `query` carries state such as active filters back up the trail.",
    ],
}
