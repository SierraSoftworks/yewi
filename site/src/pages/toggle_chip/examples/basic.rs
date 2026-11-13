use yew::prelude::*;
use yewi::prelude::*;

const TAGS: [&str; 4] = ["tag:server", "tag:edge", "tag:build", "tag:staging"];

#[function_component]
pub fn Example() -> Html {
    let chosen = use_state(|| vec!["tag:server"]);
    let toggle = |tag: &'static str| {
        let chosen = chosen.clone();
        Callback::from(move |on: bool| {
            let mut next: Vec<_> = chosen.iter().copied().filter(|other| *other != tag).collect();
            if on {
                next.push(tag);
            }
            chosen.set(next);
        })
    };

    html! {
        <Stack>
            <Field label="Tailscale tags" group=true hint="The node advertises these, so the server admits it.">
                <ToggleChipGroup>
                    { for TAGS.into_iter().map(|tag| html! {
                        <ToggleChip pressed={chosen.contains(&tag)} on_change={toggle(tag)}>{ tag }</ToggleChip>
                    }) }
                </ToggleChipGroup>
            </Field>
            <span role="status">{ format!("{} of {} tags", chosen.len(), TAGS.len()) }</span>
        </Stack>
    }
}
