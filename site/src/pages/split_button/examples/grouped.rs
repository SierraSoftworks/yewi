use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let done = use_state(|| "Nothing done yet");
    let act = |action: &'static str| {
        let done = done.clone();
        Callback::from(move |_| done.set(action))
    };

    html! {
        <Stack align={Align::Start}>
            <SplitButton label="Pause" size={Size::Small} on_click={act("Paused web-2")}>
                <MenuItem on_select={act("Checked web-2 for drift")}>{ "Check drift" }</MenuItem>
                <MenuGroup label="Actions">
                    <MenuItem aside="systemd" on_select={act("Restarted web-2")}>{ "Restart" }</MenuItem>
                    <MenuItem aside="systemd" on_select={act("Rebooted web-2")}>{ "Reboot" }</MenuItem>
                    <MenuItem aside="nomad" on_select={act("Drained web-2")}>{ "Drain" }</MenuItem>
                </MenuGroup>
                <MenuItem tone={Tone::Danger} on_select={act("Forgot web-2")}>{ "Forget" }</MenuItem>
            </SplitButton>
            <span role="status">{ *done }</span>
        </Stack>
    }
}
