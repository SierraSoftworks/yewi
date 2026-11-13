crate::page! {
    title: "Tabs",
    category: Navigation,
    summary: "Tabs switch between views of one subject without leaving the page. They can also switch between separate pages that share a header.",
    examples: [
        basic => "Basic": "Each `Tab` carries its label and the panel it shows. Only the selected panel is rendered, so a panel that loads data does so when it's first opened.",
        variants => "Line and pill": "`Line` suits the main views of a page. `Pill` has no rule underneath, so it fits better in a card header or a toolbar.",
        active_label => "Active label colour": "The selected label uses the normal text colour, and only the underline uses the accent. If your app wants the label in the accent colour too, set `--yi-tabs-fg-active`.",
        icons_and_counts => "Icons, counts and disabled tabs": "An icon goes before the label and a count goes after it. A disabled tab can't be selected, and the arrow keys skip over it.",
        sizes => "Sizes": "`Small` suits tabs inside a card or next to compact controls.",
        controlled => "Controlled": "Set `selected` when you want to own the selection yourself, for example to open a tab from a link elsewhere. `manual` waits for Enter or Space before switching tabs, and `keep_mounted` keeps the state of panels that aren't showing.",
        shared_panel => "One panel for every tab": "When the tabs change what one view shows, not the view itself, pass that view as `panel` and leave the tabs without children. Keep `selected` yourself and draw the panel from it.",
        navigation => "As navigation": "When each view is a page of its own, `TabNav` renders real links. Mark the page that's being shown with `current`.",
        router => "With a router": "With the `router` feature, `NavTarget::route` follows a `yew-router` route without reloading the page, and the current tab is worked out from the location.",
    ],
}
