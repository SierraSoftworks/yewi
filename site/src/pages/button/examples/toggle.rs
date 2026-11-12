use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let pinned = use_state(|| false);
    let muted = use_state(|| true);

    let toggle = |state: &UseStateHandle<bool>| {
        let state = state.clone();
        Callback::from(move |_| state.set(!*state))
    };

    html! {
        <Stack>
            <Cluster>
                <IconButton
                    icon={icons::PIN}
                    label="Pin to dashboard"
                    variant={ButtonVariant::Subtle}
                    pressed={*pinned}
                    on_click={toggle(&pinned)}
                />
                <IconButton
                    icon={icons::BELL_OFF}
                    label="Mute alerts"
                    variant={ButtonVariant::Subtle}
                    pressed={*muted}
                    on_click={toggle(&muted)}
                />
                <Button icon={icons::EYE} pressed={*pinned} on_click={toggle(&pinned)}>{ "Watch" }</Button>
            </Cluster>
            <span role="status">
                { format!(
                    "{}, alerts {}",
                    if *pinned { "Pinned" } else { "Not pinned" },
                    if *muted { "muted" } else { "on" },
                ) }
            </span>
        </Stack>
    }
}
