crate::page! {
    title: "Steps",
    category: Data,
    summary: "An ordered sequence with a status per step: where a wizard stands, or how a run went.",
    examples: [
        wizard => "Wizard progress": "Horizontal steps suit a short flow. Give finished steps `on_click` (or `href`) so the user can go back; the current step carries `aria-current`. When the container is narrower than 30rem the steps stack.",
        statuses => "Statuses": "Each status has its own marker, and its wording is read out after the title. `Current` is where the user is; `Running` is work in flight. A step without a status icon shows its number.",
        report => "Run report": "Vertical steps take a trailing `meta`, a description and any detail as children, which suits the log of a run. `Small` keeps a long report dense.",
        sizes => "Sizes and icons": "`size` scales the marker and text together. `icon` replaces the number or status icon when a step has a natural symbol.",
    ],
}
