use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <UserMenu name="Alan Turing" detail="alan@example.com" compact=true on_sign_out={Callback::noop()}>
                <MenuItem icon={icons::SETTINGS}>{ "Settings" }</MenuItem>
            </UserMenu>
            <UserMenu name="Barbara Liskov" on_sign_out={Callback::noop()}>
                <MenuItem icon={icons::SETTINGS}>{ "Settings" }</MenuItem>
            </UserMenu>
        </Cluster>
    }
}
