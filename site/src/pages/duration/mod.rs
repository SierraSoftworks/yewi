crate::page! {
    title: "Duration",
    category: Data,
    summary: "Duration shows how long something took, or how long it has been running, with as much precision as matters at that size.",
    feature: "chrono",
    examples: [
        basic => "How long things took": "Give it a `value` and the duration picks its own units: milliseconds for a request, minutes and seconds for a build, or days and hours for an outage.",
        styles => "Styles": "`Short` is the default. `Compact` keeps only the largest unit for tight columns, and `Long` writes the units out as words so they read well in a sentence.",
        span => "Between two moments": "When you pass `since` and `until`, the duration is the time between them, so you don't need to do any arithmetic on a record's start and end times.",
        running => "Still running": "With only `since`, the duration shows the time elapsed and keeps counting. It updates every second while seconds are shown and every minute after that, using one timer that the whole page shares.",
    ],
}
