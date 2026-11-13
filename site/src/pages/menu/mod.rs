crate::page! {
    title: "Menu",
    category: Navigation,
    summary: "A menu is a list of actions, split into groups when the actions are of different kinds. You can open one from a button with `MenuButton` or `SplitButton`.",
    examples: [
        basic => "Basic": "A `Menu` holds `MenuItem`s and reports each choice through the item's `on_select`. It doesn't draw a surface of its own, so put it in a popover or, as here, a card. The arrow keys move between items, `Home` and `End` jump to either end, and typing a letter jumps to the next item that starts with it.",
        groups => "Groups": "Use `MenuGroup` to set related actions apart with a rule and an optional heading. `aside` shows where an action comes from or what its shortcut is. An action that ends or undoes something should take `Tone::Danger` and go last, and it gets a rule above it automatically.",
        states => "Options, links and disabled items": "`checked` turns an item into an option that's either on or off. `href` makes it a real link, which people can open in a new tab. A `disabled` item stays visible so people learn the action exists, but the keyboard skips over it.",
    ],
}
