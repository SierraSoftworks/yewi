crate::page! {
    title: "Loading",
    category: Basic,
    summary: "Loading tells people that something is on its way when you don't know its shape yet. When you do know the shape, use a Skeleton instead.",
    examples: [
        basic => "Inline": "`Loading` is a spinner with a label, and it's announced politely to assistive technology. Whenever more than one thing could be loading, say what is.",
        block => "Standing in for a section": "`LoadingLayout::Block` centres the indicator in a padded block, for a card or page whose content hasn't arrived yet. Add a `delay` so a fast response never flashes a spinner.",
        overlay => "Refreshing existing content": "`LoadingOverlay` keeps the content mounted at its size, dims it, and makes it inert until the refresh finishes. That way nothing jumps around and nothing stale can be clicked.",
        label_hidden => "Spinner only": "With `label_hidden`, only the spinner shows, which helps in tight spaces, but the label is still announced. `Spinner` is the bare ring for building into other controls, and it inherits the size and colour of its text.",
    ],
}
