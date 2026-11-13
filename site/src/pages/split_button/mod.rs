crate::page! {
    title: "SplitButton",
    category: Basic,
    summary: "A split button puts several actions on one thing into a single control. The most likely action goes on the face of the button, and the rest sit in a menu behind the chevron.",
    examples: [
        basic => "Basic": "`label` and `on_click` set the action on the face, and the `MenuItem` children make up the rest. Put the action people are most likely to want on the face, so the common case stays a single press.",
        variants => "Variants and sizes": "Both halves take the `variant` and `size`. As with any button, keep to one `Primary` split button per view.",
        states => "Disabled, loading and alone": "`disabled` turns off both halves. `loading` marks the face as busy but leaves the menu available. When there are no items there's no chevron either, so you don't need a special case for a list of actions that might hold only one.",
        grouped => "Grouped actions": "When the menu's actions are of different kinds, group them. Use `aside` to say where each one comes from, and put the destructive action last.",
    ],
}
