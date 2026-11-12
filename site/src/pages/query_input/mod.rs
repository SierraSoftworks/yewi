crate::page! {
    title: "QueryInput",
    category: Form,
    summary: "A single-line editor for search and filter queries that suggests the fields a query can use and the values they take.",
    examples: [
        basic => "Scoped search terms": "Pass the `fields` a term can be scoped to and their `values`. Typing offers matching fields, and a scoped term offers that field's values. `↓` and `↑` move through the list, `Enter` or `Tab` accepts and `Esc` closes it. Nothing is highlighted until asked for, so `Enter` runs the query as typed.",
        expression => "Filter expressions": "`QuerySyntax::Expression` offers fields where an operand starts and quoted values after an operator. With `applied` the input edits a draft: it shows how to apply or revert, and `Esc` restores the query in effect. Validation is the caller's: pass the problem as `error` and it shows under the input.",
        field => "In a form": "Inside a `Field` the input takes its label, hint and error like any other control. Every query input looks like a search box by default; `icon` and `monospace` restyle one that should read as code.",
        states => "Sizes and states": "Sizes match `TextInput` and `Button`. `monospace` with a prompt icon gives an expression the look of code. `invalid` flags a problem reported elsewhere, and `disabled` takes the input out of use.",
    ],
}
