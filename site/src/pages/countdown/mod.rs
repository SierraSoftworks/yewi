crate::page! {
    title: "Countdown",
    category: Data,
    summary: "Countdown shows the time left until a moment. It counts down by itself and stops at zero.",
    feature: "chrono",
    examples: [
        basic => "Time left": "Give the countdown the moment to count down to. Hover over it to see the exact time it ends.",
        formats => "Formats": "`Clock` is the default and suits timers. `Units` reads the way `Duration` does, which fits better inside a sentence.",
        done => "When the time is up": "A countdown never goes negative. Once the moment has passed, it shows `done` (or a zero if you didn't set `done`) and stops updating.",
        live => "Counting down": "Every countdown on the page ticks on one shared timer, and the page around it doesn't re-render. `on_done` is called once when the countdown reaches zero, so you can use it to refresh or to announce the outcome.",
    ],
}
