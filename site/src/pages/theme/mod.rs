crate::page! {
    title: "Theme",
    category: Utilities,
    summary: "Light and dark themes follow the system setting until the user chooses one themselves.",
    examples: [
        toggle => "ThemeToggle": "`ThemeToggle` cycles through system, light and dark, and remembers the choice. Its icon is picked in CSS from the document's `data-theme`, so it's right on the very first paint.",
        scoped => "A themed region": "`data-theme` works on any element, not just the root, because the colour tokens resolve through `color-scheme`.",
        from_rust => "Reading the theme": "`use_theme` reports the preference and whether the page is dark. Use it for code that has to restyle a canvas or a map, but prefer CSS wherever CSS can do the job.",
    ],
}
