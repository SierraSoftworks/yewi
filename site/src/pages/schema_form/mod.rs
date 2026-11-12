crate::page! {
    title: "SchemaForm",
    category: Form,
    summary: "Draws a form from a JSON Schema and edits one JSON document with it, for settings whose shape is decided by a server or a plugin.",
    feature: "schema-form",
    examples: [
        basic => "A form from a schema": "Pass the `schema` and the document as `value`; every edit reports the whole new document through `on_change`. `title` is the label, `description` the hint, `examples` the placeholder, and each type gets its control: text, secret, number with bounds, choice, switch and tags.",
        nested => "Objects, variants and lists": "A nested object is a titled group, and an optional one has a switch that adds or removes it from the document. A tagged union (`oneOf` objects sharing a `const`) is a picker followed by the chosen variant's fields; values under the same name survive a switch. A list has a row per item.",
        groups => "Group styles": "`groups` sets how nested groups are set apart: `Boxed` (the default) gives each a tonal box, `Ruled` indents them behind a rule, and `Flat` keeps a title only.",
        errors => "Errors from the server": "The form does not validate: the server that owns the schema does. Pass what it reports as `errors`, addressed by JSON Pointer (or dotted path), and each shows against its field or group. `layout` puts labels beside the controls for dense settings pages.",
        custom => "Custom controls": "`render` swaps in an app's own control for any value, usually keyed on the schema's `format`. The label, hint and error stay. Here a filter becomes a `QueryInput` and an account reference a `Select` of linked accounts.",
        fallback => "Unreadable schemas and disabled forms": "A part of a schema the form cannot draw, such as a free-form map, becomes a JSON box for that one value, which reports only text that parses. `disabled` locks the whole form while a save is in flight.",
    ],
}
