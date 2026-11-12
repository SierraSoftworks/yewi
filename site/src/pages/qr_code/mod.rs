crate::page! {
    title: "QrCode",
    category: Data,
    summary: "Shows a link or short text as a QR code to scan with a phone. Encoded locally, so the value never leaves the page.",
    feature: "qr",
    examples: [
        basic => "A link to scan": "Give the code a `label` that says what it is and a caption that says what scanning it does. Keep the same value available as a link or copyable text for people who cannot scan.",
        live => "A changing value": "The code is redrawn whenever `value` changes, so it can follow a selection or a freshly minted token.",
        sizes => "Sizes": "`size` sets the edge in pixels; without it the code uses the `--yi-qr-code-size` token (200px). Codes are always dark on white, in both themes, because many scanners cannot read an inverted code.",
        error_correction => "Error correction": "Higher levels survive more glare, dirt and damage but make the code denser. `Medium` suits screens; use `High` for print.",
        too_long => "Values that do not fit": "A QR code holds at most about 2,900 bytes. A longer value shows a message instead, which `fallback` replaces with your own content.",
    ],
}
