use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let labels = use_state(|| vec![KeyValue::new("env", "production"), KeyValue::new("tier", "web")]);
    let on_change = {
        let labels = labels.clone();
        Callback::from(move |rows| labels.set(rows))
    };

    let summary = labels
        .iter()
        .filter(|row| !row.is_blank())
        .map(|row| format!("{}={}", row.key, row.value))
        .collect::<Vec<_>>()
        .join(", ");

    html! {
        <Stack>
            <Field label="Labels" group=true hint="The agent's own labels decide its groups and rollout phase.">
                <KeyValueEditor rows={(*labels).clone()} {on_change} row_label="label" add_label="Add label" mono=true />
            </Field>
            <span role="status">{ if summary.is_empty() { "No labels".to_string() } else { summary } }</span>
        </Stack>
    }
}
