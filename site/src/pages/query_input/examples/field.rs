use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

const KNOWN: [&str; 3] = ["title", "draft", "tag"];

#[function_component]
pub fn Example() -> Html {
    let filter = use_state(|| "draft == false && owner == \"ada\"".to_string());

    let fields = use_memo((), |_| -> Rc<[QueryField]> {
        Rc::from([
            QueryField::new("title").description("The entry's title"),
            QueryField::new("draft").description("Not yet published").values(["true", "false"]),
            QueryField::new("tag").description("A label on the entry").values(["release", "internal", "security"]),
        ])
    });

    // Stands in for the app's own analysis of the expression.
    let unknown = filter
        .split(|c: char| !c.is_alphanumeric() && c != '"')
        .filter(|word| word.chars().next().is_some_and(char::is_alphabetic))
        .find(|word| !KNOWN.contains(word) && !["true", "false"].contains(word))
        .map(|word| format!("Unsupported field: {word}."));

    let on_input = {
        let filter = filter.clone();
        Callback::from(move |next| filter.set(next))
    };

    html! {
        <Field label="Only run for" hint="Available fields: title, draft, tag." error={unknown.map(Html::from)}>
            <QueryInput
                syntax={QuerySyntax::Expression}
                icon={icons::LIST_FILTER}
                value={(*filter).clone()}
                {on_input}
                fields={(*fields).clone()}
                placeholder={r#"title contains "release""#}
            />
        </Field>
    }
}
