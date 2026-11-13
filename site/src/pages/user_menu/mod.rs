crate::page! {
    title: "UserMenu",
    category: Navigation,
    summary: "The user menu shows the signed-in person at the end of an app bar. It's an avatar button that opens a menu showing who they are, the app's account actions and sign out.",
    examples: [
        basic => "Basic": "Give it the person's `name`, an optional `detail` such as their role, and the app's actions as `MenuItem` children. `on_sign_out` adds the sign out item at the end. The button is the avatar, and the menu it opens starts with who's signed in. Otherwise it works like a `MenuButton`.",
        app_bar => "In an app bar": "Put it last in the `actions` of an `AppBar`, after the theme toggle and any status. When nobody is signed in, show a sign in button in its place.",
        named => "With the name": "`show_name` puts the name, the `detail` and a chevron beside the avatar, for a bar with room to spare. On a narrow screen the button goes back to showing only the avatar.",
        links => "Links and options": "The menu takes everything a `Menu` does, including links, groups with headings, and options with a check mark. `sign_out_href` turns sign out into a link, for apps that end the session by visiting a URL.",
    ],
}
