use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let syncing = use_state(|| false);
    let start = {
        let syncing = syncing.clone();
        Callback::from(move |_| syncing.set(true))
    };
    let cancel = {
        let syncing = syncing.clone();
        Callback::from(move |_| syncing.set(false))
    };

    html! {
        <Stack align={Align::Start}>
            <Cluster>
                <SplitButton label="Sync" icon={icons::REFRESH_CW} loading={*syncing} on_click={start}>
                    <MenuItem disabled={!*syncing} on_select={cancel}>{ "Cancel sync" }</MenuItem>
                    <MenuItem>{ "Sync settings" }</MenuItem>
                </SplitButton>
                <SplitButton label="Disabled" disabled=true>
                    <MenuItem>{ "Never shown" }</MenuItem>
                </SplitButton>
                <SplitButton label="Demote" face_disabled=true menu_label="More actions for this user">
                    <MenuItem>{ "Reset password" }</MenuItem>
                    <MenuItem>{ "Remove" }</MenuItem>
                </SplitButton>
                <SplitButton label="Only one action" />
            </Cluster>
            <span role="status">{ if *syncing { "Syncing" } else { "Idle" } }</span>
        </Stack>
    }
}
