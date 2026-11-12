use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let chosen = use_state(|| "Signed in as Ada Lovelace");
    let choose = |action: &'static str| {
        let chosen = chosen.clone();
        Callback::from(move |_: MouseEvent| chosen.set(action))
    };
    let on_sign_out = {
        let chosen = chosen.clone();
        Callback::from(move |()| chosen.set("Signed out"))
    };

    html! {
        <Stack align={Align::Start}>
            <UserMenu name="Ada Lovelace" detail="admin" {on_sign_out}>
                <MenuItem icon={icons::USER} on_select={choose("Opened the profile")}>{ "Profile" }</MenuItem>
                <MenuItem icon={icons::KEY_ROUND} on_select={choose("Opened access tokens")}>{ "Access tokens" }</MenuItem>
                <MenuItem icon={icons::SETTINGS} on_select={choose("Opened settings")}>{ "Settings" }</MenuItem>
            </UserMenu>
            <span role="status">{ *chosen }</span>
        </Stack>
    }
}
