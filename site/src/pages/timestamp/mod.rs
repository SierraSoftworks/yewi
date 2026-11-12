crate::page! {
    title: "Timestamp",
    category: Data,
    summary: "Shows a moment relative to now while it is recent and as a date once it is not, with the exact local and UTC time on hover.",
    feature: "chrono",
    examples: [
        basic => "Recent and older moments": "By default a timestamp is relative within a day of now, the weekday and time within a week, and the date beyond that. Hover any of them for the exact time. Relative text keeps itself current, on one timer shared by the whole page.",
        display => "Choosing what to show": "`display` fixes the form where the automatic one does not suit: always relative for activity feeds, a full date and time for audit records.",
        compact => "Compact, for tables": "`compact` abbreviates each form and drops the year from dates in the current year, so a column of timestamps stays narrow.",
        utc => "UTC": "`utc` writes dates and times in UTC and says so, for logs and schedules that people in several time zones read together. Relative text is the same in every zone, so it is left alone.",
        detail => "The exact time, always visible": "`TimestampDetail` is what the tooltip shows: the viewer's local time, then the same moment in UTC. Use it in a detail pane, where there is room and nobody should have to hover.",
        keyboard => "Reaching the tooltip from the keyboard": "`focusable` makes a timestamp a tab stop, so the exact time shows on focus too. Use it for the one or two timestamps that matter on a view; in a long list it would add a stop to every row.",
        live => "Staying current": "A timestamp follows the clock without the page around it re-rendering. Record an event and leave it: after a minute it reads \"1 minute ago\".",
    ],
}
