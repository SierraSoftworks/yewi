crate::page! {
    title: "Drawer",
    category: Overlay,
    summary: "A modal panel that slides in from the edge of the screen, for details and forms that should keep the page behind them in view.",
    examples: [
        basic => "A drawer": "The body scrolls and the footer stays pinned. Like a dialog it holds the keyboard, closes on `Escape` or a click outside, and returns focus to its trigger.",
        navigation => "From the start edge": "`side` picks the edge. Without a `title` there is no header, so give it a `label`; this is the shape of an app's navigation on a narrow screen.",
        nested => "With a confirmation": "A dialog opened from a drawer stacks above it. `Escape` closes only the topmost, and focus steps back the way it came.",
    ],
}
