crate::page! {
    title: "Timeline",
    category: Data,
    summary: "A timeline shows events in order along a rail, such as a commit log, an incident's updates or an audit trail.",
    examples: [
        commit_log => "Commit log": "In the base look, each event has an outlined dot, a title, a muted `meta` line and a trailing `aside`. A `tone` fills the dot and colours the rail below it, and a `Diamond` sets apart an event of a different kind. With `href` or `on_click` the whole event becomes the target, while controls in the other slots stay usable.",
        incident => "Status updates": "Give every event a `tone` and the rail records how things stood between updates. `active` pulses the marker of an event that's still going on, and the children hold the detail.",
        summary => "Horizontal summary": "`Horizontal` shows only the markers, on a line that fills the width. Each event's title, detail and `meta` appear in a popover on hover or keyboard focus. Use it for a handful of events, such as an incident on an overview page.",
        markers => "Markers and icons": "`marker` picks the shape, and `icon` replaces it with a symbol on a tinted circle. Use a `Ring` for something that hasn't settled yet, and use icons when events differ in kind rather than in status.",
        options => "Run-off, rules and size": "`run_off` chooses which ends of the rail fade out, to show that time ran on before or after what's shown. `divided` separates each event with a rule, and `Small` suits a dense log.",
    ],
}
