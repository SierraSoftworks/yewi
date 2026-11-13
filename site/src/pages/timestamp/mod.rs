crate::page! {
    title: "Timestamp",
    category: Data,
    summary: "A timestamp shows a moment relative to now while it's recent, and as a date once it isn't. Hover it to see the exact local and UTC time.",
    feature: "chrono",
    examples: [
        basic => "Recent and older moments": "By default, a timestamp is relative when it's within a day of now, shows the weekday and time within a week, and shows the date beyond that. Hover any of them to see the exact time. Relative text keeps itself up to date using one timer that the whole page shares.",
        display => "Choosing what to show": "Use `display` to fix the form when the automatic choice doesn't suit. For example, you might always want relative times in an activity feed, or a full date and time in audit records.",
        compact => "Compact, for tables": "`compact` abbreviates each form and drops the year from dates in the current year, so a column of timestamps stays narrow.",
        utc => "UTC": "`utc` writes dates and times in UTC and says so. This helps with logs and schedules that people in several time zones read together. Relative text is the same in every time zone, so it's left alone.",
        detail => "The exact time, always visible": "`TimestampDetail` is what the tooltip shows: the viewer's local time, followed by the same moment in UTC. Use it in a detail pane, where there's room for it and nobody should have to hover.",
        keyboard => "Reaching the tooltip from the keyboard": "`focusable` makes a timestamp a tab stop, so the exact time shows on focus too. Use it for the one or two timestamps that matter on a view. In a long list it would add a tab stop to every row.",
        live => "Staying current": "A timestamp follows the clock without the page around it re-rendering. Record an event and leave it, and after a minute it'll read \"1 minute ago\".",
    ],
}
