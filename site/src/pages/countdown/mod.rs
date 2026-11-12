crate::page! {
    title: "Countdown",
    category: Data,
    summary: "Shows the time left until a moment, counting down by itself and stopping at zero.",
    feature: "chrono",
    examples: [
        basic => "Time left": "Give the moment to count down to. Hover the countdown for the exact time it ends.",
        formats => "Formats": "`Clock` is the default and suits timers. `Units` reads the way `Duration` does, which sits better inside a sentence.",
        done => "When the time is up": "A countdown never goes negative. Once the moment has passed it shows `done`, or a zero when there is none, and stops updating.",
        live => "Counting down": "The countdown ticks on one timer shared by the whole page, and the page around it does not re-render. `on_done` is called once when it reaches zero: use it to refresh, or to announce the outcome.",
    ],
}
