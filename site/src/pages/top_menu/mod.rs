crate::page! {
    title: "TopMenu",
    category: Navigation,
    summary: "An application's main destinations as a row of links, for apps with too few to need a sidebar.",
    examples: [
        basic => "In an app bar": "Put the menu in the `AppBar`, after the brand. Mark where the user is with `current`: the label darkens and an underline in the accent sits on the bar's lower edge. A `NavTarget::external` item opens a new tab and says so. The bar's `actions` sit at the end.",
        standalone => "Standalone": "The menu needs no bar or shell. Alone it is a navigation landmark that fills the row it is given, here above a page's content.",
        groups => "Groups": "`TopMenuGroup` puts related destinations behind one label, which opens them as a menu of `MenuItem`s. Set `current` on the group while one of its pages is showing.",
        collapsed => "Collapsed": "With less than about 30rem to itself the row gives way to a menu button at the end of the bar. Its dropdown holds the bar's `actions` in a row, then each destination as a row of its own, with groups as headings. `collapse` moves that point (`Wide`), forces it (`Always`, shown here) or turns it off (`Never`, which scrolls sideways instead).",
        routed => "With a router": "With the `router` feature, `NavTarget::route(..)` makes an item a client-side link that marks itself current from the router's location.",
    ],
}
