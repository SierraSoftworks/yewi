crate::page! {
    title: "SegmentBar",
    category: Data,
    summary: "A segment bar is a thin bar divided in proportion to its parts, which shows how a whole splits into different states.",
    examples: [
        basic => "Basic": "Pass the segments and the `total`. Whatever the segments leave of the total stays as empty track, and `rest_label` names it. Hover over a segment to see its number. Screen readers announce the whole bar as one sentence.",
        states => "Several states": "Choose each segment's tone by what it means. `Tone::Active` adds a moving sheen for work that's in progress. A segment stays visible however small its share is, but segments with nothing in them are left out.",
        legend => "Legend and sizes": "`legend` lists every part under the bar, which helps when a bar stands on its own rather than in a row of them. `size` sets the thickness.",
    ],
}
