crate::page! {
    title: "Copyable",
    category: Basic,
    summary: "Copyable shows a value with a button that puts it on the clipboard. It's handy for ids, hashes, tokens and addresses.",
    examples: [
        basic => "Basic": "Wrap any value that someone is likely to paste elsewhere. The children are what's shown and `value` is what's copied, so a shortened hash still copies in full.",
        variants => "Variants": "`Inline` sits in text, tables and description lists. `Chip` sets a value apart from the prose around it. `Field` is a full-width box for something the user is expected to take away.",
        field => "Long values": "A long value wraps so that all of it can be read. Use `truncate` when keeping it on one line matters more, and turn `mono` off for values that are prose.",
        button => "Button only": "Use `CopyButton` when the content is already on screen, such as in the header of a code sample. If you give it children, it shows them as a label, and the label confirms the copy in place.",
        hook => "Custom trigger": "`use_clipboard` gives any element the same behaviour. Call `copy` from a click handler and render from `state`.",
    ],
}
