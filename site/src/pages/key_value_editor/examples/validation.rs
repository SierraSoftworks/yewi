use std::collections::HashSet;

use yew::prelude::*;
use yewi::prelude::*;

/// What is wrong with each row, by index.
fn problems(rows: &[KeyValue]) -> Vec<(usize, AttrValue)> {
    let mut seen = HashSet::new();
    rows.iter()
        .enumerate()
        .filter(|(_, row)| !row.is_blank())
        .filter_map(|(index, row)| {
            let key = row.key.trim();
            let message = if key.is_empty() {
                "A variable needs a name.".to_string()
            } else if !key.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') {
                format!("'{key}' can only use capital letters, digits and '_'.")
            } else if !seen.insert(key.to_string()) {
                format!("'{key}' is set twice.")
            } else {
                return None;
            };
            Some((index, message.into()))
        })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    let variables = use_state(|| {
        vec![KeyValue::new("LOG_LEVEL", "info"), KeyValue::new("region", "eu-west-1"), KeyValue::new("LOG_LEVEL", "debug")]
    });
    let on_change = {
        let variables = variables.clone();
        Callback::from(move |rows| variables.set(rows))
    };
    let errors = problems(&variables);

    html! {
        <Stack>
            <KeyValueEditor
                label="Environment variables"
                rows={(*variables).clone()}
                {on_change}
                errors={errors.clone()}
                row_label="variable"
                key_placeholder="NAME"
                add_label="Add variable"
                mono=true
            />
            <span role="status">
                { match errors.len() {
                    0 => "Ready to save".to_string(),
                    1 => "1 problem".to_string(),
                    count => format!("{count} problems"),
                } }
            </span>
        </Stack>
    }
}
