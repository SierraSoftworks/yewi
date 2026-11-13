crate::page! {
    title: "TopMenu",
    category: Navigation,
    summary: "A top menu shows an application's main destinations as a row of links. It's meant for apps with too few destinations to need a sidebar.",
    examples: [
        basic => "In an app bar": "Put the menu in the `AppBar`, after the brand. Mark where the user is with `current`, which darkens the label and puts an accent underline on the bar's lower edge. A `NavTarget::external` item opens a new tab and says so. The bar's `actions` sit at the end.",
        standalone => "Standalone": "The menu doesn't need a bar or shell. On its own it's a navigation landmark that fills the row you give it, which in this example is above a page's content.",
        groups => "Groups": "`TopMenuGroup` puts related destinations behind one label, which opens them as a menu of `MenuItem`s. Set `current` on the group while one of its pages is showing.",
        collapsed => "Collapsed": "When the menu has less than about 30rem to itself, the row gives way to a menu button at the end of the bar. Its dropdown holds the bar's `actions` in a row, followed by each destination on a row of its own, with groups as headings. Use `collapse` to move that point (`Wide`), force it (`Always`, shown here) or turn it off (`Never`, which scrolls sideways instead).",
        routed => "With a router": "With the `router` feature, `NavTarget::route(..)` turns an item into a client-side link that marks itself as current from the router's location.",
    ],
}
