crate::page! {
    title: "UserMenu",
    category: Navigation,
    summary: "The signed-in person at the end of an app bar: an avatar button that opens who they are, the app's account actions and sign out.",
    examples: [
        basic => "Basic": "Give it the person's `name`, an optional `detail` such as their role, and the app's actions as `MenuItem` children. `on_sign_out` adds the sign out item at the end. It opens and closes like a `MenuButton`.",
        app_bar => "In an app bar": "Put it last in the `actions` of an `AppBar`, after the theme toggle and any status. On a narrow screen the button drops to the avatar alone and the menu still says who is signed in.",
        compact => "Avatar only": "`compact` keeps just the avatar on the button, for a bar that is already busy. The name stays the button's accessible name. Without `detail` the button and the menu show the name alone.",
        links => "Links and options": "The menu takes everything a `Menu` does: links, groups with headings and options with a check mark. `sign_out_href` makes sign out a link, for apps that end the session by visiting a URL.",
    ],
}
