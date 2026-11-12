crate::page! {
    title: "Timeline",
    category: Data,
    summary: "Events in order along a rail: a commit log, an incident's updates, an audit trail.",
    examples: [
        commit_log => "Commit log": "The base look: an outlined dot per event, a title, a muted `meta` line and a trailing `aside`. A `tone` fills the dot and colours the rail below it; a `Diamond` sets apart an event of another kind. With `href` or `on_click` the whole event is the target, while controls in the other slots stay usable.",
        incident => "Status updates": "Give every event a `tone` and the rail records how things stood between updates. `active` pulses the marker of an event that is still going on. Children hold the detail.",
        summary => "Horizontal summary": "`Horizontal` shows only the markers, on a line that fills the width; each event's title, detail and `meta` appear in a popover on hover or keyboard focus. Use it for a handful of events, such as an incident on an overview page.",
        markers => "Markers and icons": "`marker` picks the shape; `icon` replaces it with a symbol on a tinted circle. Use a `Ring` for something not yet settled, and icons where events differ in kind rather than in status.",
        options => "Run-off, rules and size": "`run_off` chooses which ends of the rail fade, saying that time ran on before or after what is shown. `divided` rules off each event, and `Small` suits a dense log.",
    ],
}
