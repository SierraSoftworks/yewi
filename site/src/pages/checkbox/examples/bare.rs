use yew::prelude::*;
use yewi::prelude::*;

const HOSTS: [&str; 3] = ["web-01", "web-02", "web-03"];

#[function_component]
pub fn Example() -> Html {
    let selected = use_state(|| [false, true, false]);

    html! {
        <Stack gap={Space::Sm}>
            { for HOSTS.into_iter().enumerate().map(|(index, host)| {
                let on_change = {
                    let selected = selected.clone();
                    Callback::from(move |checked| {
                        let mut next = *selected;
                        next[index] = checked;
                        selected.set(next);
                    })
                };
                html! {
                    <Cluster>
                        <Checkbox
                            size={Size::Small}
                            label={format!("Select {host}")}
                            checked={selected[index]}
                            {on_change}
                        />
                        <span>{ host }</span>
                    </Cluster>
                }
            }) }
            <span role="status">{ format!("{} selected", selected.iter().filter(|on| **on).count()) }</span>
        </Stack>
    }
}
