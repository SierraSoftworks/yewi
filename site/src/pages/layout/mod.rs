crate::page! {
    title: "Stack, Cluster, Grid",
    category: Layout,
    summary: "Stack, Cluster and Grid manage the space between their children, so components never need outer margins of their own.",
    examples: [
        stack => "Stack": "A stack lays its children out in a column, with an even gap taken from the spacing scale.",
        cluster => "Cluster": "A cluster lays its children out in a row that wraps when it runs out of room. It suits toolbars, button rows and tag lists.",
        grid => "Grid": "A grid has equal columns that reflow to fit the container. Set `min` to the narrowest useful column width, or set `columns` for a fixed number of columns.",
    ],
}
