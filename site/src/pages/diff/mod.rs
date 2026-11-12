crate::page! {
    title: "Diff",
    category: Data,
    summary: "Shows what changed between two texts, unified or side by side, so a change can be reviewed before it is applied.",
    feature: "diff",
    examples: [
        compare => "Comparing two texts": "Pass `before` and `after` and the diff is computed in the browser. Unchanged lines further than `context` lines from a change fold behind a button, and an edited line marks the part that changed.",
        unified => "A unified diff": "When a server or `git diff` already produced the patch, pass it as `unified`. The file header becomes the title and each hunk header separates the regions of the file.",
        split => "Side by side": "`DiffMode::Split` puts the old text on the left and the new on the right, which suits wide views and rewritten blocks. Lines wrap, and the two sides stack when the diff is too narrow for them.",
        tokens => "Custom line content": "`render_line` draws the text of each line, for syntax highlighting or for tokens. Here secret references are shown as tokens, so a rotation reads as a changed fingerprint and the value itself is never shown.",
        long => "Long lines and long diffs": "`wrap` keeps long lines inside the diff instead of scrolling sideways. Set `--yi-diff-max-height` to scroll a long diff within the page; the scrolling region takes keyboard focus.",
    ],
}
