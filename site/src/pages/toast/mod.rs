crate::page! {
    title: "Toast",
    category: Overlay,
    summary: "Reports the outcome of an action in a corner of the screen, then gets out of the way.",
    examples: [
        basic => "Reporting an outcome": "Wrap the app in one `ToastHost` and call `use_toast` wherever something happens. Successes dismiss themselves after five seconds; failures stay until dismissed, because a refusal is something to read, not to catch.",
        tones => "Tones": "The tone says what kind of outcome it is. A title is optional: use one when the message needs a sentence of detail. `Toast` can also be rendered directly, as here, to show one in the flow of a page.",
        action => "With an action": "A toast can offer one follow-up, such as undoing what was just done. Activating it also dismisses the toast. Timers pause while the pointer or keyboard focus is on the stack, so there is time to reach it.",
        lifecycle => "Work in flight": "`show` returns an id. Keep it to replace a sticky `Active` toast with the result once the work finishes.",
        placement => "Placement": "Toasts stack at the bottom end of the viewport by default. Pick another edge with `placement`; the newest toast always sits nearest that edge.",
        accent => "Accent": "`ToastAccent::Icon` (the default) marks the tone with an icon. `ToastAccent::Stripe` draws a stripe along the leading edge instead. Set it once on the host so every toast agrees.",
    ],
}
