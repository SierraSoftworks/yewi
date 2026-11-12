crate::page! {
    title: "AppShell",
    category: Layout,
    summary: "The frame of an application: an app bar across the top, a sidebar of destinations, and the page.",
    examples: [
        basic => "An application": "Pass an `AppBar` as `bar` and a `Sidebar` as `sidebar`; the children are the page. The menu button hides and shows the sidebar, and on a narrow screen opens it in a drawer that closes once a destination is picked. These examples are `embedded` so that they fit on this page; a real shell leaves that off and fills the viewport.",
        grouped => "Grouped navigation, remembered": "Many destinations read better under group labels. `storage_key` remembers on this device that the sidebar was hidden, and `collapsed` sets how it starts.",
        scrolling => "A page that fills the shell": "By default the document scrolls and the bar and sidebar stay in view. With `fill` the shell is exactly one viewport tall and the page area scrolls instead; add `flush` for content that runs edge to edge, such as a map.",
        custom_toggle => "Your own navigation control": "`use_app_shell()` gives any component inside the shell the navigation's state and its toggle, for a control that lives somewhere other than the app bar.",
    ],
}
