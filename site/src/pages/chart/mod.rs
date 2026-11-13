crate::page! {
    title: "Chart",
    category: Visualisation,
    summary: "Charts plot one or more series over time. The primary series is drawn as an area and the rest as lines, and charts stay light even with very large datasets.",
    feature: "charts",
    examples: [
        basic => "A single series": "Give the chart a `label` and one `Series`, and the first series is drawn as an area. To read exact values, hover over the plot, or focus it and use the arrow keys.",
        compare => "Comparing series": "Any further series are drawn as lines, and a legend appears. For context such as the previous period, use `SeriesColor::Muted` and a dash so that the primary series stays in front.",
        styles => "Colour, dash and markers": "Each series can set its `color`, `dash` and `markers`. Colour is how people tell series apart, so pin it with `SeriesColor::Slot` when series can be filtered. Keep `SeriesColor::Tone` for series that really mean that status.",
        bars => "Bars, and charts that share positions": "`SeriesKind::Bars` suits sparse counts that people read one at a time, and `integer_ticks` keeps a count's axis to whole numbers. Measures on very different scales belong in separate charts stacked on the same positions, so don't put them on a second axis: give them the same axis width, pass each the position the other reports through `on_hover` as its `cursor`, and set `x_ticks=0` on all but the last.",
        select => "Selecting a range": "With `selectable`, you can drag across the plot to pick a range, or hold Shift with the arrow keys and then press Enter. The range is reported through `on_select`, which you'd typically use to zoom the time range.",
        large => "Large datasets": "A long series is reduced to a minimum and a maximum for each horizontal bucket, so the markup stays the same size and spikes still show. Hold the data in a `use_memo`. `SeriesData` compares by pointer, so re-renders never touch the points.",
        formatting => "Units and empty data": "`x_label` and `y_label` format positions and values for the axes, and for the tooltip too unless `x_tooltip` and `y_tooltip` say something fuller there. When no series has any points, the chart shows the `empty` message at the same height.",
        sparkline => "Sparkline": "`Sparkline` shows a trend with no axes, for table rows and tiles, and it fills the width of its container. Give it a `label` when the text beside it doesn't already describe the trend.",
    ],
}
