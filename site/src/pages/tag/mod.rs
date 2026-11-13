crate::page! {
    title: "Tag",
    category: Basic,
    summary: "Tags are compact labels for metadata, such as categories, attributes, and name and value pairs.",
    examples: [
        basic => "Basic": "Tags are squared and outlined, so metadata never gets mistaken for a status `Pill`. Use `mono` for identifiers and versions.",
        named => "Name and value": "`name` puts a quieter key before the value, for labels such as `env: production`.",
        tones => "Tones and icons": "A tag stays neutral unless it carries a status of its own, such as marking a destructive step. An icon goes before the label.",
        removable => "Removable": "`on_remove` adds a remove button, which is named after the tag for screen readers. Make sure focus lands somewhere sensible after a removal. In this example it moves to the next tag's button.",
        link => "As a link": "With `href`, the label becomes a link. Use this for a tag that filters by, or opens, the thing it names. A linked tag can still be removable.",
        shape => "Shape": "The corner radius comes from the `--yi-tag-radius` token. Tags are squared by default, and you can set the token on `.yi-tag` in your app's stylesheet if you want rounded ones.",
    ],
}
