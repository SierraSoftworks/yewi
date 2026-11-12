crate::page! {
    title: "Chart",
    category: Visualisation,
    summary: "Plots one or more series over time: the primary series as an area, the rest as lines. Built to stay light on very large datasets.",
    feature: "charts",
    examples: [
        basic => "A single series": "Give the chart a `label` and one `Series`. The first series is drawn as an area. Hover, or focus the plot and use the arrow keys, to read exact values.",
        compare => "Comparing series": "Further series are drawn as lines and a legend appears. Use `SeriesColor::Muted` and a dash for context such as the previous period, so the primary series stays in front.",
        styles => "Colour, dash and markers": "Each series can set its `color`, `dash` and `markers`. Colour identifies a series: pin it with `SeriesColor::Slot` when series can be filtered, and keep `SeriesColor::Tone` for series that mean that status.",
        bars => "Bars": "`SeriesKind::Bars` suits sparse counts that are read one at a time. Measures on very different scales belong in separate charts stacked on the same positions, never on a second axis.",
        select => "Selecting a range": "With `selectable`, dragging across the plot (or Shift with the arrow keys, then Enter) reports a range through `on_select`, typically to zoom the time range.",
        large => "Large datasets": "A long series is reduced to a minimum and a maximum per horizontal bucket, so the markup stays the same size and spikes survive. Hold the data in a `use_memo`: `SeriesData` compares by pointer, so re-renders never touch the points.",
        formatting => "Units and empty data": "`y_label` formats values for the axis and tooltip. When no series has any points the `empty` message is shown at the same height.",
        sparkline => "Sparkline": "`Sparkline` is a trend with no axes for table rows and tiles. It fills its container's width. Give it a `label` when the trend is not described by the text beside it.",
    ],
}
