crate::page! {
    title: "Tag",
    category: Basic,
    summary: "A compact label for metadata: categories, attributes and name and value pairs.",
    examples: [
        basic => "Basic": "Tags are squared and outlined so that metadata never reads as a status `Pill`. Use `mono` for identifiers and versions.",
        named => "Name and value": "`name` puts a quieter key before the value, for labels such as `env: production`.",
        tones => "Tones and icons": "A tag stays neutral unless it carries a status of its own, such as a destructive step. An icon goes before the label.",
        removable => "Removable": "`on_remove` adds a remove button, named after the tag for screen readers. Keep focus somewhere sensible after a removal; here it moves to the next tag's button.",
        link => "As a link": "With `href` the label is a link, for a tag that filters or opens what it names. A linked tag can still be removable.",
        shape => "Shape": "The corner radius is the `--yi-tag-radius` token. Squared is the default; set the token on `.yi-tag` in the app's stylesheet for rounded tags.",
    ],
}
