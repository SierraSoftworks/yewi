crate::page! {
    title: "QueryInput",
    category: Form,
    summary: "QueryInput is a single-line editor for search and filter queries. It suggests the fields a query can use and the values they can take.",
    examples: [
        basic => "Scoped search terms": "Pass the `fields` that a term can be scoped to, along with their `values`. As you type, the input offers matching fields, and once a term is scoped it offers that field's values. `↓` and `↑` move through the list, `Enter` or `Tab` accepts a suggestion and `Esc` closes the list. Nothing is highlighted until you ask for it, so `Enter` runs the query as typed.",
        expression => "Filter expressions": "With `QuerySyntax::Expression`, the input offers fields where an operand starts and quoted values after an operator. When you pass `applied`, the input edits a draft. It shows how to apply or revert the draft, and `Esc` restores the query that's in effect. Validation is up to the caller, so pass any problem as `error` and it shows under the input.",
        field => "In a form": "Inside a `Field`, the input takes its label, hint and error like any other control. Every query input looks like a search box by default, but you can use `icon` and `monospace` to restyle one that should read as code.",
        states => "Sizes and states": "Sizes match `TextInput` and `Button`. Combining `monospace` with a prompt icon makes an expression look like code. `invalid` flags a problem that's reported elsewhere, and `disabled` takes the input out of use.",
    ],
}
