crate::page! {
    title: "SegmentBar",
    category: Data,
    summary: "A thin bar divided in proportion to its parts, showing how a whole splits into states.",
    examples: [
        basic => "Basic": "Give the segments and the `total`. What the segments leave of the total stays as empty track, named by `rest_label`. Hover a segment for its number; the whole bar is announced as one sentence.",
        states => "Several states": "Choose each segment's tone by meaning. `Tone::Active` carries a moving sheen for work in flight, and a segment stays visible however small its share. Segments with nothing in them are left out.",
        legend => "Legend and sizes": "`legend` lists every part under the bar, for a bar that stands on its own rather than in a row of them. `size` sets the thickness.",
    ],
}
