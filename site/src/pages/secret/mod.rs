crate::page! {
    title: "Secret",
    category: Form,
    summary: "SecretInput takes a password, token or key and masks it until asked. SecretReveal and SecretValue hand a newly created secret to the user, once.",
    examples: [
        basic => "Secret input": "The text is masked until the user presses the show button, because a token often has to be checked against where it came from. The input is controlled like `TextInput` and joins a `Field` the same way.",
        generator => "Generating a secret": "`generator` adds a chip that fills the field with a random value from the browser's cryptographic generator and reveals it, for secrets that are agreed rather than issued. `generator_bytes` sets the strength.",
        states => "States": "A read-only secret can still be revealed. `revealable` off suits a secret that must never be read back from this screen. Set `autocomplete` to `current-password` or `new-password` for account passwords; the default keeps password managers away from API tokens.",
        sizes => "Sizes": "Sizes match `TextInput` and `Button`. `mono` suits tokens and keys that will be compared character by character.",
        value => "Showing a stored secret": "`SecretValue` is a labelled, read-only secret with a copy button. With `concealed` the value stays out of the page until the user shows it, and copying works either way.",
        reveal => "Revealing a new secret": "`SecretReveal` presents a secret the server will not show again. It never closes on its own: the user dismisses it once the value is stored.",
        aside => "With a QR code and instructions": "`aside` holds content that belongs beside the values and drops below them in a narrow container. Replace `warning` when the consequence needs spelling out.",
    ],
}
