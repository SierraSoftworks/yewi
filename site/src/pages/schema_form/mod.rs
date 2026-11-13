crate::page! {
    title: "SchemaForm",
    category: Form,
    summary: "SchemaForm draws a form from a JSON Schema and uses it to edit one JSON document. It's meant for settings whose shape is decided by a server or a plugin.",
    feature: "schema-form",
    examples: [
        basic => "A form from a schema": "Pass the `schema`, and the document as `value`. Every edit reports the whole new document through `on_change`. The schema's `title` becomes the label, `description` the hint and `examples` the placeholder. Each type gets its own control, whether that's text, a secret, a number with bounds, a choice, a switch or tags.",
        nested => "Objects, variants and lists": "A nested object becomes a titled group. If it's optional, it gets a switch that adds it to or removes it from the document. A tagged union (`oneOf` objects that share a `const`) becomes a picker followed by the chosen variant's fields, and values under the same name survive when you switch. A list gets one row per item.",
        groups => "Group styles": "`groups` sets how nested groups are set apart. `Boxed` (the default) gives each group a tonal box, `Ruled` indents them behind a rule, and `Flat` keeps only a title.",
        errors => "Errors from the server": "The form doesn't do validation, because the server that owns the schema does that. Pass whatever it reports as `errors`, addressed by JSON Pointer (or dotted path), and each error shows against its field or group. For dense settings pages, `layout` puts the labels beside the controls.",
        custom => "Custom controls": "Use `render` to swap in your app's own control for any value, usually keyed on the schema's `format`. The label, hint and error stay as they are. Here, a filter becomes a `QueryInput` and an account reference becomes a `Select` of linked accounts.",
        fallback => "Unreadable schemas and disabled forms": "When the form can't draw part of a schema, such as a free-form map, it shows a JSON box for that one value instead. The box only reports text that parses. `disabled` locks the whole form, for example while a save is in flight.",
    ],
}
