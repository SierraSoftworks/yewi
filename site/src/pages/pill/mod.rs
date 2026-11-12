crate::page! {
    title: "Pill",
    category: Basic,
    summary: "A coloured state label, so a condition looks the same wherever it appears.",
    examples: [
        tones => "Tones": "A pill is soft with a leading dot by default. Choose the tone by what the state means, and keep the wording to a word or two.",
        emphasis => "Emphasis": "`active` pulses the dot for a state that is live or in flight. `bold` fills the pill for the one state on a view that must not be missed. Both are rare by design.",
        content => "Content": "Turn the dot off with `dot=false` for a plain label, or to place an icon or your own `StatusDot` as content.",
        link => "As a link": "With `href` the pill is a real link to what it names, such as the rollout behind a state.",
        live => "Changing state": "A pill only shows the state; announce a change with a live region near it, as the status line here does.",
    ],
}
