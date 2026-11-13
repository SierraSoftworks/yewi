crate::page! {
    title: "Pill",
    category: Basic,
    summary: "A pill is a coloured label for a state, so that a condition looks the same wherever it appears.",
    examples: [
        tones => "Tones": "By default a pill is soft with a leading dot. Choose the tone based on what the state means, and keep the wording to a word or two.",
        emphasis => "Emphasis": "`active` pulses the dot for a state that's live or in progress. `bold` fills the pill, for the one state on a view that mustn't be missed. Both are meant to be used rarely.",
        content => "Content": "Turn the dot off with `dot=false` for a plain label, or when you want to put an icon or your own `StatusDot` inside.",
        link => "As a link": "With `href`, the pill becomes a real link to the thing it names, such as the rollout behind a state.",
        live => "Changing state": "A pill only shows the state and doesn't announce changes. To announce a change, use a live region near it, as the status line here does.",
    ],
}
