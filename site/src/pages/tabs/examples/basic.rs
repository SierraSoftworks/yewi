use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let opened = use_state(|| AttrValue::from("profile"));
    let on_change = {
        let opened = opened.clone();
        Callback::from(move |value: AttrValue| opened.set(value))
    };

    html! {
        <Stack>
            <Tabs label="Account" {on_change}>
                <Tab value="profile" label="Profile">
                    { "Name, callsign and the groups this person belongs to." }
                </Tab>
                <Tab value="devices" label="Devices">
                    { "Phones and radios enrolled under this account." }
                </Tab>
                <Tab value="credentials" label="Credentials">
                    { "Client certificates and when each one expires." }
                </Tab>
            </Tabs>
            <span role="status">{ format!("Showing {}", *opened) }</span>
        </Stack>
    }
}
