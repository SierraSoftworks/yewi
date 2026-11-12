use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let digest = use_state(|| true);
    let toggle = {
        let digest = digest.clone();
        Callback::from(move |_| digest.set(!*digest))
    };

    html! {
        <Stack align={Align::Start}>
            <UserMenu name="Edsger Dijkstra" detail="edsger@example.com" sign_out_href="#signed-out">
                <MenuItem icon={icons::USER} href="#profile">{ "Profile" }</MenuItem>
                <MenuItem icon={icons::EXTERNAL_LINK} href="#docs" aside="docs">{ "Documentation" }</MenuItem>
                <MenuGroup label="Email">
                    <MenuItem checked={*digest} on_select={toggle}>{ "Weekly digest" }</MenuItem>
                </MenuGroup>
            </UserMenu>
            <span role="status">{ if *digest { "Weekly digest on" } else { "Weekly digest off" } }</span>
        </Stack>
    }
}
