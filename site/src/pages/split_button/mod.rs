crate::page! {
    title: "SplitButton",
    category: Basic,
    summary: "Several actions on one thing as a single control: the likeliest on the button's face, the rest in a menu behind the chevron.",
    examples: [
        basic => "Basic": "`label` and `on_click` are the action on the face; the `MenuItem` children are the rest. Put the action people most likely want on the face, so the common case stays one press.",
        variants => "Variants and sizes": "Both halves take the `variant` and `size`. Keep to one `Primary` split button per view, as with any button.",
        states => "Disabled, loading and alone": "`disabled` turns off both halves. `loading` marks the face busy and leaves the menu available. With no items there is no chevron, so a list of actions that may hold only one needs no special case.",
        grouped => "Grouped actions": "Group the menu when its actions are of different kinds, say where each comes from with `aside`, and leave the destructive one for last.",
    ],
}
