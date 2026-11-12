crate::page! {
    title: "Link",
    category: Basic,
    summary: "Navigation in running text and on its own, plus a router-aware link for in-app routes.",
    examples: [
        basic => "In text and alone": "Inside running text a link is underlined, because colour alone must not mark it. On its own, in a list of actions or a footer, the underline appears on hover.",
        variants => "Variants and underline": "`Subtle` takes the colour of the surrounding text, for names in tables and lists where a column of accent is too loud. `underline` overrides the automatic behaviour.",
        external => "External and with an icon": "`external` opens a new tab, adds an arrow and tells screen readers. An `icon` goes before the text.",
        action => "As an action": "Without `href` the link renders a real `<button>`, for a low-key action that does not navigate. `disabled` works on both forms.",
        router => "Router-aware": "With the `router` feature, `RouteLink` takes a `yew-router` route. It renders a real `href`, navigates without a page load and marks the current route with `aria-current`.",
    ],
}
