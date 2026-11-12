use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let chosen = use_state(|| "Nothing chosen yet");
    let choose = |action: &'static str| {
        let chosen = chosen.clone();
        Callback::from(move |_| chosen.set(action))
    };

    html! {
        <Stack align={Align::Start}>
            <Card flush=true>
                <Menu label="Host web-2">
                    <MenuItem on_select={choose("Paused web-2")}>{ "Pause" }</MenuItem>
                    <MenuItem on_select={choose("Checked web-2 for drift")}>{ "Check drift" }</MenuItem>
                    <MenuGroup label="Actions">
                        <MenuItem aside="systemd" on_select={choose("Restarted web-2")}>{ "Restart" }</MenuItem>
                        <MenuItem aside="systemd" on_select={choose("Rebooted web-2")}>{ "Reboot" }</MenuItem>
                        <MenuItem aside="nomad" on_select={choose("Drained web-2")}>{ "Drain" }</MenuItem>
                    </MenuGroup>
                    <MenuItem tone={Tone::Danger} on_select={choose("Forgot web-2")}>{ "Forget" }</MenuItem>
                </Menu>
            </Card>
            <span role="status">{ *chosen }</span>
        </Stack>
    }
}
