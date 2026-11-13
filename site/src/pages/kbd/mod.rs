crate::page! {
    title: "Kbd",
    category: Basic,
    summary: "Kbd shows a key or keyboard shortcut, as a hint beside the action it triggers.",
    examples: [
        basic => "Single keys": "Name a key the way it's printed on the keyboard. The key cap scales with the surrounding text.",
        shortcuts => "Shortcuts": "`keys` takes a whole shortcut. A `+` joins keys that are held together, and a space separates steps that are pressed in turn.",
        hints => "Beside an action": "A shortcut hint goes after the label it belongs to, in muted text so the label still leads.",
        options => "Style options": "By default it's a borderless tinted key. Set the component tokens on a `Kbd` for an outlined key, or add `--yi-kbd-depth: 2px` for a raised cap.",
    ],
}
