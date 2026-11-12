crate::page! {
    title: "Loading",
    category: Basic,
    summary: "Says that something is on its way when its shape is not known yet. When the shape is known, use a Skeleton.",
    examples: [
        basic => "Inline": "`Loading` is a spinner with a label, announced politely to assistive technology. Say what is loading whenever more than one thing could be.",
        block => "Standing in for a section": "`LoadingLayout::Block` centres the indicator in a padded block, for a card or page whose content has not arrived. Add `delay` so a fast response never flashes a spinner.",
        overlay => "Refreshing existing content": "`LoadingOverlay` keeps the content mounted at its size, dims it and makes it inert until the refresh finishes, so nothing jumps and nothing stale can be clicked.",
        label_hidden => "Spinner only": "With `label_hidden` only the spinner shows, for tight spaces; the label is still announced. `Spinner` is the bare ring for composing into other controls, and inherits the size and colour of its text.",
    ],
}
