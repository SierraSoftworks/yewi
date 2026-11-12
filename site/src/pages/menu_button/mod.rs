crate::page! {
    title: "MenuButton",
    category: Basic,
    summary: "A button that opens a menu of actions, for several actions on one thing where none is the obvious default.",
    examples: [
        basic => "Basic": "Give the button a `label` and its actions as `MenuItem` children. A press or `Enter` opens the menu on its first item; choosing an item, `Esc` or a press outside closes it and returns focus to the button.",
        variants => "Variants and sizes": "A menu button takes the same `variant`, `size` and `icon` as a `Button`. The chevron says it opens something rather than acting; drop it with `caret={false}` only where that is already plain.",
        row_actions => "Row actions": "`icon_only` gives a square button for the actions of a row, with `label` as its accessible name. Open towards the row with `Placement::BottomEnd`. The menu exists only while open, so one per row is cheap.",
        grouped => "Groups and options": "The menu takes everything a `Menu` does: groups with headings, options with a check mark, links and a dangerous action set apart at the end.",
    ],
}
