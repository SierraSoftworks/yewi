crate::page! {
    title: "QrCode",
    category: Data,
    summary: "QrCode shows a link or short text as a QR code that people can scan with a phone. It's encoded locally, so the value never leaves the page.",
    feature: "qr",
    examples: [
        basic => "A link to scan": "Give the code a `label` that says what it is, and a caption that says what scanning it does. Keep the same value available as a link or as text people can copy, for anyone who can't scan.",
        live => "A changing value": "The code is redrawn whenever `value` changes, so it can follow a selection or a freshly minted token.",
        sizes => "Sizes": "`size` sets the width of the code in pixels. Without it, the code uses the `--yi-qr-code-size` token (200px). Codes are always dark on white in both themes, because many scanners can't read an inverted code.",
        error_correction => "Error correction": "Higher levels survive more glare, dirt and damage, but they make the code denser. `Medium` suits screens, and `High` is better for print.",
        too_long => "Values that do not fit": "A QR code can hold at most about 2,900 bytes. If the value is longer, the code shows a message instead, and you can replace that message with your own content using `fallback`.",
    ],
}
