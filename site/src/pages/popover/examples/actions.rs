use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let paused = use_state(|| false);

    let on_open_change = {
        let open = open.clone();
        Callback::from(move |next| open.set(next))
    };
    let confirm = {
        let (open, paused) = (open.clone(), paused.clone());
        Callback::from(move |_| {
            paused.set(!*paused);
            open.set(false);
        })
    };
    let cancel = {
        let open = open.clone();
        Callback::from(move |_| open.set(false))
    };

    let verb = if *paused { "Resume" } else { "Pause" };
    let content = html! {
        <Stack gap={Space::Sm}>
            <span>
                { if *paused { "Probes will start running again at once." } else { "No probes will run until it is resumed." } }
            </span>
            <Cluster gap={Space::Sm}>
                <Button size={Size::Small} variant={ButtonVariant::Primary} on_click={confirm}>{ verb }</Button>
                <Button size={Size::Small} variant={ButtonVariant::Subtle} on_click={cancel}>{ "Cancel" }</Button>
            </Cluster>
        </Stack>
    };

    html! {
        <Stack align={Align::Start}>
            <Popover
                open={*open}
                {on_open_change}
                head={format!("{verb} monitoring?")}
                {content}
                label="Confirm"
                placement={Placement::BottomStart}
            >
                <Button icon={if *paused { icons::PLAY } else { icons::PAUSE }}>{ format!("{verb} monitoring") }</Button>
            </Popover>
            <span role="status">{ if *paused { "Monitoring is paused" } else { "Monitoring is running" } }</span>
        </Stack>
    }
}
