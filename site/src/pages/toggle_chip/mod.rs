crate::page! {
    title: "ToggleChip",
    category: Form,
    summary: "Toggle chips switch a handful of independent filters or options on and off, right where they are listed.",
    examples: [
        basic => "Chips in a field": "Each chip is controlled: it shows `pressed` and reports the new state through `on_change`. `ToggleChipGroup` lays them out in a row that wraps, and a `Field` with `group` names the set.",
        filters => "Filters": "Outside a form, give the group its own `label`. A chip with an `icon` keeps it whether it's on or off; a chip without one gets a check mark while it's on.",
        states => "Sizes and states": "`Small` is the default and sits well beside text. `Medium` matches the height of other controls in a toolbar. A disabled chip still shows whether it's on.",
    ],
}
