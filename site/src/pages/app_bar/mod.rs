crate::page! {
    title: "AppBar",
    category: Navigation,
    summary: "The bar across the top of an application: brand at the start, the app's own controls after it, actions at the end.",
    examples: [
        basic => "Brand and actions": "`brand` takes an `AppBarBrand` and `actions` sits at the end, where the user and the theme toggle go. Inside an `AppShell` with a sidebar the bar also shows the menu button.",
        slots => "With a menu and status": "The children follow the brand and are aligned to the start: the place for a `TopMenu`, a breadcrumb or an environment badge. Controls given to the menu's `actions` rather than the bar's fold into its dropdown on a narrow screen, so nothing is clipped on a phone.",
        brand => "Brands": "`AppBarBrand` takes a `logo` (an icon, an image or inline SVG) and the name. With `href` it links home. The logo is drawn in the accent; set `--yi-app-bar-logo-color` to change that.",
    ],
}
