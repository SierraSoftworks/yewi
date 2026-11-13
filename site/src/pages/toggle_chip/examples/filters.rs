use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let failed = use_state(|| true);
    let mine = use_state(|| false);
    let pinned = use_state(|| false);
    let set = |state: &UseStateHandle<bool>| {
        let state = state.clone();
        Callback::from(move |on| state.set(on))
    };

    let shown = [(*failed, "failed"), (*mine, "mine"), (*pinned, "pinned")]
        .into_iter()
        .filter_map(|(on, name)| on.then_some(name))
        .collect::<Vec<_>>();

    html! {
        <Stack>
            <ToggleChipGroup label="Filter runs">
                <ToggleChip pressed={*failed} on_change={set(&failed)}>{ "Failed" }</ToggleChip>
                <ToggleChip pressed={*mine} on_change={set(&mine)}>{ "Started by me" }</ToggleChip>
                <ToggleChip pressed={*pinned} on_change={set(&pinned)} icon={icons::PIN}>{ "Pinned" }</ToggleChip>
            </ToggleChipGroup>
            <span role="status">
                { if shown.is_empty() { "Showing every run".to_string() } else { format!("Showing: {}", shown.join(", ")) } }
            </span>
        </Stack>
    }
}
