crate::page! {
    title: "Stack, Cluster, Grid",
    category: Layout,
    summary: "Three primitives that own the space between their children, so components never carry outer margins.",
    examples: [
        stack => "Stack": "Children in a column with an even gap from the spacing scale.",
        cluster => "Cluster": "Children in a row that wraps when it runs out of room: toolbars, button rows, tag lists.",
        grid => "Grid": "Equal columns that reflow to fit the container. Set `min` for the narrowest useful column, or `columns` for a fixed count.",
    ],
}
