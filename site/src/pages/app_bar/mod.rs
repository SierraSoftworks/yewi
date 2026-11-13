crate::page! {
    title: "AppBar",
    category: Navigation,
    summary: "The app bar runs across the top of an application. The brand sits at the start, the app's own controls come after it, and actions go at the end.",
    examples: [
        basic => "Brand and actions": "`brand` takes an `AppBarBrand`, and `actions` sits at the end of the bar, which is where things like the current user and the theme toggle go. Inside an `AppShell` with a sidebar, the bar also shows the menu button.",
        slots => "With a menu and status": "The children follow the brand and line up at the start, so this is the place for a `TopMenu`, a breadcrumb or an environment badge. A `TopMenu` here takes over the bar's `actions` and folds them into its dropdown on a narrow screen, so nothing gets clipped on a phone.",
        filter => "With a filter": "A filter or search box among the children needs more width than a phone's bar has left. `wrap` gives the children a row of their own beneath the brand and actions on a narrow screen, and leaves a wide bar as it is.",
        brand => "Brands": "`AppBarBrand` takes a `logo` and the name. The logo can be an icon, an image or inline SVG, and with `href` the brand links home. The logo is drawn in the accent colour, and you can set `--yi-app-bar-logo-color` to change it.",
    ],
}
