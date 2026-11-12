use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <UserMenu name="Alan Turing" detail="admin" show_name=true on_sign_out={Callback::noop()}>
                <MenuItem icon={icons::SETTINGS}>{ "Settings" }</MenuItem>
            </UserMenu>
            <UserMenu name="Barbara Liskov" show_name=true on_sign_out={Callback::noop()}>
                <MenuItem icon={icons::SETTINGS}>{ "Settings" }</MenuItem>
            </UserMenu>
        </Cluster>
    }
}
