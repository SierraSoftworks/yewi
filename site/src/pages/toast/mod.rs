crate::page! {
    title: "Toast",
    category: Overlay,
    summary: "Toasts report the outcome of an action in a corner of the screen, then get out of the way.",
    examples: [
        basic => "Reporting an outcome": "Wrap your app in one `ToastHost`, and call `use_toast` wherever something happens. Successes dismiss themselves after five seconds. Failures stay until they're dismissed, because a refusal is something people need to read, not something they should have to catch.",
        tones => "Tones": "The tone says what kind of outcome the toast reports. The title is optional, so use one when the message needs a sentence of detail. You can also render `Toast` directly, as here, to show one in the flow of a page.",
        action => "With an action": "A toast can offer one follow-up action, such as undoing what was just done. Activating it also dismisses the toast. The timers pause while the pointer or keyboard focus is on the stack, so people have time to reach it.",
        lifecycle => "Work in flight": "`show` returns an id. Keep hold of it so you can replace a sticky `Active` toast with the result once the work finishes.",
        placement => "Placement": "By default, toasts stack at the bottom end of the viewport. You can pick another edge with `placement`, and the newest toast always sits nearest that edge.",
    ],
}
