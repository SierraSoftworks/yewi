crate::page! {
    title: "Duration",
    category: Data,
    summary: "Shows how long something took, or has been running, to the precision that matters at its size.",
    feature: "chrono",
    examples: [
        basic => "How long things took": "Give a `value` and the duration picks its own grain: milliseconds for a request, minutes and seconds for a build, days and hours for an outage.",
        styles => "Styles": "`Short` is the default. `Compact` keeps only the largest unit for tight columns, and `Long` writes words for use in a sentence.",
        span => "Between two moments": "With `since` and `until` the duration is the time between them, so a record's start and end times need no arithmetic.",
        running => "Still running": "With only `since` the duration is the time elapsed and keeps counting: every second while seconds show, every minute after that, on one timer shared by the whole page.",
    ],
}
