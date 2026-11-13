crate::page! {
    title: "Diff",
    category: Data,
    summary: "Diff shows what changed between two texts, either unified or side by side, so you can review a change before it's applied.",
    feature: "diff",
    examples: [
        compare => "Comparing two texts": "Pass `before` and `after`, and the diff is computed in the browser. Unchanged lines more than `context` lines away from a change fold behind a button, and an edited line marks the part that changed.",
        unified => "A unified diff": "When a server or `git diff` has already produced the patch, pass it as `unified`. The file header becomes the title, and each hunk header separates the regions of the file.",
        split => "Side by side": "`DiffMode::Split` puts the old text on the left and the new text on the right, which suits wide views and rewritten blocks. Lines wrap, and the two sides stack when there isn't room for them side by side.",
        tokens => "Custom line content": "Use `render_line` to draw the text of each line yourself, for syntax highlighting or tokens. Here secret references are shown as tokens, so a rotation reads as a changed fingerprint and the value itself is never shown.",
        long => "Long lines and long diffs": "`wrap` keeps long lines inside the diff instead of scrolling sideways. To scroll a long diff within the page, set `--yi-diff-max-height`. The scrolling region takes keyboard focus, so it can be scrolled from the keyboard too.",
    ],
}
