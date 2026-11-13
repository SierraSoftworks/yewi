crate::page! {
    title: "Drawer",
    category: Overlay,
    summary: "A drawer is a modal panel that slides in from the edge of the screen. Use it for details and forms when the page behind should stay in view.",
    examples: [
        basic => "A drawer": "The body scrolls while the footer stays pinned. Like a dialog, a drawer holds the keyboard, closes on `Escape` or a click outside, and returns focus to its trigger.",
        navigation => "From the start edge": "Use `side` to pick which edge the drawer comes from. Without a `title` there's no header, so give it a `label` instead. This is how an app's navigation usually looks on a narrow screen.",
        nested => "With a confirmation": "A dialog opened from a drawer stacks above it. `Escape` closes only the topmost one, and focus steps back the way it came.",
    ],
}
