crate::page! {
    title: "Surfaces and shape",
    category: Foundations,
    summary: "Lifted cards, tonal sibling panels, two radii, a 4px spacing grid and a compact density.",
    api: [],
    examples: [
        card => "Lifted card": "A card is an object that sits on the page: a soft border and a faint two-layer shadow, never a hard outline. The `card-surface` Sass mixin applies it.",
        tonal => "Tonal siblings": "Sibling panels inside a card are stacked flush and full width with no gap or border, told apart by alternating `--yw-tone-1` and `--yw-tone-2`.",
        radius => "Radius and elevation": "Controls use 6px and cards 9px. `--yw-shadow-overlay` is for things that float above the page, such as menus and popovers.",
        spacing => "Spacing": "A 4px grid. Layout primitives take these steps as their `gap`.",
        density => "Density": "`data-yw-density=\"compact\"` drops controls from 32px to 28px for tables and toolbars.",
    ],
}
