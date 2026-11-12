crate::page! {
    title: "Tabs",
    category: Navigation,
    summary: "Switches between views of one subject without leaving the page, or between pages that share a header.",
    examples: [
        basic => "Basic": "Each `Tab` carries its label and the panel it shows. Only the selected panel is rendered, so a panel that loads data does so when it is first opened.",
        variants => "Line and pill": "`Line` suits the main views of a page. `Pill` has no rule, so it sits better in a card head or a toolbar.",
        active_label => "Active label colour": "The selected label uses the text colour and only the underline carries the accent. Set `--yi-tabs-fg-active` for an app that wants the label in the accent too.",
        icons_and_counts => "Icons, counts and disabled tabs": "An icon goes before the label and a count after it. A disabled tab cannot be selected and the arrow keys skip it.",
        sizes => "Sizes": "`Small` suits tabs inside a card or beside compact controls.",
        controlled => "Controlled": "Set `selected` to own the selection, for example to open a tab from a link elsewhere. `manual` waits for Enter or Space before switching, and `keep_mounted` keeps the state of panels that are not showing.",
        navigation => "As navigation": "`TabNav` renders real links for views that are pages of their own. Mark the page being shown with `current`.",
        router => "With a router": "With the `router` feature, `NavTarget::route` follows a `yew-router` route without reloading and the current tab is worked out from the location.",
    ],
}
