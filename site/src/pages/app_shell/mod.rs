crate::page! {
    title: "AppShell",
    category: Layout,
    summary: "The app shell is the frame of an application. It puts an app bar across the top, a sidebar of destinations down the side, and your page in the space that's left.",
    examples: [
        basic => "An application": "Pass an `AppBar` as `bar` and a `Sidebar` as `sidebar`, and the children become the page. The menu button hides and shows the sidebar. On a narrow screen it opens the sidebar in a drawer, which closes once a destination is picked. These examples are `embedded` so that they fit on this page, but a real shell leaves that off and fills the viewport.",
        grouped => "Grouped navigation, remembered": "When there are many destinations, they read better under group labels. `storage_key` remembers on this device that the sidebar was hidden, and `collapsed` sets how it starts.",
        scrolling => "A page that fills the shell": "By default the document scrolls while the bar and sidebar stay in view. With `fill`, the shell is exactly one viewport tall and the page area scrolls instead. Add `flush` for content that runs edge to edge, such as a map.",
        custom_toggle => "Your own navigation control": "If you want a navigation control somewhere other than the app bar, call `use_app_shell()`. It gives any component inside the shell the navigation's state and its toggle.",
    ],
}
