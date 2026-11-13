crate::page! {
    title: "MenuButton",
    category: Basic,
    summary: "A menu button opens a menu of actions. Use it when there are several actions on one thing and none of them is the obvious default.",
    examples: [
        basic => "Basic": "Give the button a `label` and pass its actions as `MenuItem` children. Pressing the button or `Enter` opens the menu on its first item. Choosing an item, pressing `Esc` or pressing outside closes it and returns focus to the button.",
        variants => "Variants and sizes": "A menu button takes the same `variant`, `size` and `icon` as a `Button`. The chevron tells people it opens something rather than acting straight away, so only drop it with `caret={false}` where that's already obvious.",
        row_actions => "Row actions": "`icon_only` gives you a square button for a row's actions, and `label` becomes its accessible name. Use `Placement::BottomEnd` so the menu opens towards the row. The menu only exists while it's open, so having one per row is cheap.",
        grouped => "Groups and options": "The menu accepts everything a `Menu` does, including groups with headings, options with a check mark, links, and a dangerous action set apart at the end.",
    ],
}
