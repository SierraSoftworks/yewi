crate::page! {
    title: "Link",
    category: Basic,
    summary: "Links navigate to a URL or an in-app route, either inside running text or on their own.",
    examples: [
        basic => "In text and alone": "Inside running text a link is underlined, because colour alone shouldn't be what marks it. When it stands on its own, such as in a list of actions or a footer, the underline appears on hover.",
        variants => "Variants and underline": "`Subtle` takes the colour of the surrounding text, for names in tables and lists where a whole column of accent colour would be too loud. `underline` overrides the automatic underline behaviour.",
        external => "External and with an icon": "`NavTarget::external` opens the link in a new tab, adds an arrow and tells screen readers that it does. An `icon` goes before the text.",
        action => "As an action": "Without `href`, the link renders a real `<button>`, for a low-key action that doesn't navigate. `disabled` works on both forms.",
        router => "Router-aware": "With the `router` feature, `href` takes a `yew-router` route as `NavTarget::route(..)`. It renders a real `href`, navigates without a page load, and marks the current route with `aria-current`. Every component that has an `href` accepts routes the same way.",
    ],
}
