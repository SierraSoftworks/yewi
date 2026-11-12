crate::page! {
    title: "Menu",
    category: Navigation,
    summary: "A list of actions, in groups when they are of different kinds. `MenuButton` and `SplitButton` open one from a button.",
    examples: [
        basic => "Basic": "A `Menu` holds `MenuItem`s and reports each choice through the item's `on_select`. It draws no surface of its own, so put it in a popover or, as here, a card. Arrow keys move between items, `Home` and `End` jump to the ends and a letter jumps to the next item starting with it.",
        groups => "Groups": "`MenuGroup` sets related actions apart with a rule and an optional heading, and `aside` says where an action comes from or what its shortcut is. An action that ends or undoes something takes `Tone::Danger` and goes last: it gets a rule above it without asking.",
        states => "Options, links and disabled items": "`checked` makes an item an option that is on or off. `href` makes it a real link, which can be opened in a new tab. A `disabled` item stays visible, so people learn the action exists, but the keyboard skips it.",
    ],
}
