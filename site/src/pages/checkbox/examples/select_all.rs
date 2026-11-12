use yew::prelude::*;
use yewi::prelude::*;

const SERVICES: [&str; 3] = ["checkout-api", "billing", "search"];

#[function_component]
pub fn Example() -> Html {
    let selected = use_state(|| [true, false, false]);
    let count = selected.iter().filter(|on| **on).count();

    let toggle_all = {
        let selected = selected.clone();
        Callback::from(move |checked| selected.set([checked; 3]))
    };

    html! {
        <Stack gap={Space::Sm}>
            <Checkbox
                checked={count == SERVICES.len()}
                indeterminate={count > 0 && count < SERVICES.len()}
                on_change={toggle_all}
            >
                { "All services" }
            </Checkbox>
            <Stack gap={Space::Sm} attrs={attrs! { "style" => "padding-inline-start: var(--yi-space-xl)" }}>
                { for SERVICES.into_iter().enumerate().map(|(index, service)| {
                    let on_change = {
                        let selected = selected.clone();
                        Callback::from(move |checked| {
                            let mut next = *selected;
                            next[index] = checked;
                            selected.set(next);
                        })
                    };
                    html! { <Checkbox checked={selected[index]} {on_change}>{ service }</Checkbox> }
                }) }
            </Stack>
            <span role="status">{ format!("{count} selected") }</span>
        </Stack>
    }
}
