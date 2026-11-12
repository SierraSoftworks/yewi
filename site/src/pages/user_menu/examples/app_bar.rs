use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let signed_in = use_state(|| true);
    let on_sign_out = {
        let signed_in = signed_in.clone();
        Callback::from(move |()| signed_in.set(false))
    };
    let on_sign_in = {
        let signed_in = signed_in.clone();
        Callback::from(move |_: MouseEvent| signed_in.set(true))
    };

    let brand = html! {
        <AppBarBrand href="#" label="Buckle overview" logo={html! { <Icon icon={icons::BOXES} /> }}>
            { "Buckle" }
        </AppBarBrand>
    };
    let actions = html! {
        <>
            <ThemeToggle />
            if *signed_in {
                <UserMenu name="Grace Hopper" detail="read only" {on_sign_out}>
                    <MenuItem icon={icons::KEY_ROUND}>{ "Access tokens" }</MenuItem>
                    <MenuItem icon={icons::LIFE_BUOY}>{ "Help" }</MenuItem>
                </UserMenu>
            } else {
                <Button size={Size::Small} on_click={on_sign_in}>{ "Sign in" }</Button>
            }
        </>
    };

    html! {
        <Stack>
            <AppBar {brand} {actions} />
            <span role="status">{ if *signed_in { "Signed in as Grace Hopper" } else { "Signed out" } }</span>
        </Stack>
    }
}
