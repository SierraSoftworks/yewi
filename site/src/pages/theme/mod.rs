crate::page! {
    title: "Theme",
    category: Utilities,
    summary: "Light and dark themes that follow the system until the user chooses otherwise.",
    examples: [
        toggle => "ThemeToggle": "Cycles system, light and dark, and remembers the choice. Its icon is picked in CSS from the document's `data-theme`, so it is right on first paint.",
        scoped => "A themed region": "`data-theme` works on any element, not only the root, because colour tokens resolve through `color-scheme`.",
        from_rust => "Reading the theme": "`use_theme` reports the preference and whether the page is dark, for code that must restyle a canvas or a map. Prefer CSS wherever CSS can do the job.",
    ],
}
