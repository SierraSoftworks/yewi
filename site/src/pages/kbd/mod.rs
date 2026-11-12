crate::page! {
    title: "Kbd",
    category: Basic,
    summary: "A key or keyboard shortcut, for hints beside the action it triggers.",
    examples: [
        basic => "Single keys": "Name a key the way it is printed on the keyboard. The cap scales with the surrounding text.",
        shortcuts => "Shortcuts": "`keys` takes a whole shortcut: `+` joins keys held together and a space separates steps pressed in turn.",
        hints => "Beside an action": "A shortcut hint sits after the label it belongs to, in muted text so the label still leads.",
        options => "Style options": "The default cap has a heavier lower edge. Set the component tokens on a `Kbd` for a flat key (`--yi-kbd-depth: 1px`) or a tinted one.",
    ],
}
