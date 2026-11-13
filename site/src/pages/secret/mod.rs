crate::page! {
    title: "Secret",
    category: Form,
    summary: "SecretInput takes a password, token or key and keeps it masked until the user asks to see it. SecretReveal and SecretValue hand a newly created secret to the user, once.",
    examples: [
        basic => "Secret input": "The text stays masked until the user presses the show button, because people often need to check a token against wherever it came from. The input is controlled like `TextInput` and goes in a `Field` the same way.",
        generator => "Generating a secret": "`generator` adds a chip that fills the field with a random value from the browser's cryptographic generator and reveals it. This suits secrets that are agreed between two sides rather than issued by one. `generator_bytes` sets the strength.",
        states => "States": "A read-only secret can still be revealed. Turn `revealable` off for a secret that must never be read back from this screen. For account passwords, set `autocomplete` to `current-password` or `new-password`. The default keeps password managers away from API tokens.",
        sizes => "Sizes": "Sizes match `TextInput` and `Button`. `monospace` suits tokens and keys that people will compare character by character.",
        value => "Showing a stored secret": "`SecretValue` is a labelled, read-only secret with a copy button. With `concealed`, the value stays out of the page until the user shows it, and copying works either way.",
        reveal => "Revealing a new secret": "`SecretReveal` presents a secret that the server won't show again. It never closes on its own, so the user dismisses it once they've stored the value.",
        aside => "With a QR code and instructions": "`aside` holds content that belongs beside the values, and it drops below them in a narrow container. Replace `warning` when the consequence needs spelling out.",
    ],
}
