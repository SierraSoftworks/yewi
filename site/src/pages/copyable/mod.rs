crate::page! {
    title: "Copyable",
    category: Basic,
    summary: "A value with a button that puts it on the clipboard, for ids, hashes, tokens and addresses.",
    examples: [
        basic => "Basic": "Wrap any value a user is likely to paste elsewhere. The children are what is shown and `value` is what is copied, so a shortened hash still copies in full.",
        variants => "Variants": "`Inline` sits in text, tables and description lists. `Chip` sets a value apart from the prose around it. `Field` is a full-width box for something the user is expected to take away.",
        field => "Long values": "A long value wraps so that all of it can be read. Use `truncate` where one line matters more, and turn `mono` off for values that are prose.",
        button => "Button only": "Use `CopyButton` where the content is already on screen, such as the header of a code sample. With children it shows a label, which confirms in place.",
        hook => "Custom trigger": "`use_clipboard` gives any element the same behaviour: call `copy` from a click and render from `state`.",
    ],
}
