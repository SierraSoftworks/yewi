crate::page! {
    title: "Steps",
    category: Data,
    summary: "Steps show an ordered sequence with a status for each step. Use them to show where the user is in a wizard, or how a run went.",
    examples: [
        wizard => "Wizard progress": "Horizontal steps suit a short flow. Give the finished steps `on_click` (or `href`) so the user can go back to them. The current step carries `aria-current`. When the container is narrower than 30rem, the steps stack.",
        statuses => "Statuses": "Each status has its own marker, and its wording is read out after the step's title. `Current` is where the user is, while `Running` means work is in progress. A step without a status icon shows its number.",
        report => "Run report": "Vertical steps take a trailing `meta`, a description and any further detail as children, which suits the log of a run. `Small` keeps a long report dense.",
        sizes => "Sizes and icons": "`size` scales the marker and the text together. When a step has a natural symbol, use `icon` to replace the number or status icon.",
    ],
}
