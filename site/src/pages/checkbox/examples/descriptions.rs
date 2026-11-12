use std::collections::BTreeSet;

use yew::prelude::*;
use yewi::prelude::*;

const EVENTS: [(&str, &str, &str); 3] = [
    ("failed", "Failed deploys", "A rollout stopped or was rolled back."),
    ("approval", "Approval requests", "Someone needs you to approve a production deploy."),
    ("succeeded", "Successful deploys", "Every rollout that completes. This can be noisy."),
];

#[function_component]
pub fn Example() -> Html {
    let selected = use_state(|| BTreeSet::from(["failed", "approval"]));

    html! {
        <Stack>
            <Field label="Email me about" hint="Sent to your account address." group=true>
                <Stack gap={Space::Sm}>
                    { for EVENTS.into_iter().map(|(key, label, description)| {
                        let on_change = {
                            let selected = selected.clone();
                            Callback::from(move |checked| {
                                let mut next = (*selected).clone();
                                if checked { next.insert(key); } else { next.remove(key); }
                                selected.set(next);
                            })
                        };
                        html! {
                            <Checkbox checked={selected.contains(key)} {on_change} {description}>{ label }</Checkbox>
                        }
                    }) }
                </Stack>
            </Field>
            <span role="status">{ format!("{} of {} selected", selected.len(), EVENTS.len()) }</span>
        </Stack>
    }
}
