crate::page! {
    title: "UserMenu",
    category: Navigation,
    summary: "The signed-in person at the end of an app bar: an avatar button that opens who they are, the app's account actions and sign out.",
    examples: [
        basic => "Basic": "Give it the person's `name`, an optional `detail` such as their role, and the app's actions as `MenuItem` children. `on_sign_out` adds the sign out item at the end. The button is the avatar; the menu opens with who is signed in, and works like a `MenuButton`.",
        app_bar => "In an app bar": "Put it last in the `actions` of an `AppBar`, after the theme toggle and any status. When nobody is signed in, show a sign in button in its place.",
        named => "With the name": "`show_name` puts the name, the `detail` and a chevron beside the avatar, for a bar with room to spare. On a narrow screen the button drops back to the avatar alone.",
        links => "Links and options": "The menu takes everything a `Menu` does: links, groups with headings and options with a check mark. `sign_out_href` makes sign out a link, for apps that end the session by visiting a URL.",
    ],
}
