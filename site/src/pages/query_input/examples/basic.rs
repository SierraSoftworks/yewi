use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let query = use_state(String::new);
    let ran = use_state(|| None::<String>);

    let fields = use_memo((), |_| -> Rc<[QueryField]> {
        Rc::from([
            QueryField::new("workflow")
                .description("Match a workflow by name")
                .values(["copper-tiger-canyon", "nightly-budget-sync", "release-notes"]),
            QueryField::new("status").description("Match the last run").values(["succeeded", "failed", "running"]),
            QueryField::new("partition").description("Match a queue partition").values(["cron", "webhooks", "retry queue"]),
        ])
    });

    let on_input = {
        let query = query.clone();
        Callback::from(move |next| query.set(next))
    };
    let on_submit = {
        let ran = ran.clone();
        Callback::from(move |query| ran.set(Some(query)))
    };

    html! {
        <Stack>
            <QueryInput
                value={(*query).clone()}
                {on_input}
                {on_submit}
                fields={(*fields).clone()}
                label="Search activity"
                placeholder="Search activity, e.g. status:failed budget"
            />
            <span role="status">
                { match &*ran {
                    Some(query) if query.is_empty() => "Showing everything".to_string(),
                    Some(query) => format!("Searched for {query}"),
                    None => "Press Enter to search".to_string(),
                } }
            </span>
        </Stack>
    }
}
