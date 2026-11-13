crate::page! {
    title: "Surfaces and shape",
    category: Foundations,
    summary: "Cards lift off the page, and the panels inside them are told apart by tone. This page covers those surfaces along with the two corner radii, the 4px spacing grid and the compact density.",
    api: [],
    examples: [
        card => "Lifted card": "A card is an object that sits on the page. It has a soft border and a faint two-layer shadow, never a hard outline. The `card-surface` Sass mixin applies this look.",
        tonal => "Tonal siblings": "Sibling panels inside a card stack flush and full width, with no gap or border between them. You tell them apart because they alternate between `--yi-tone-1` and `--yi-tone-2`.",
        radius => "Radius and elevation": "Controls use a 6px radius and cards use 9px. `--yi-shadow-overlay` is for things that float above the page, such as menus and popovers.",
        spacing => "Spacing": "Spacing follows a 4px grid, and the layout primitives take these steps as their `gap`.",
        density => "Density": "Setting `data-yi-density=\"compact\"` drops controls from 32px to 28px, which suits tables and toolbars.",
    ],
}
