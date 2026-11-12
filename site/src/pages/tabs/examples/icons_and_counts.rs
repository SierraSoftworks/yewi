use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Tabs label="Account" default_selected="devices">
            <Tab value="profile" label="Profile" icon={icons::USER}>
                { "Name, callsign and groups." }
            </Tab>
            <Tab value="devices" label="Devices" icon={icons::SMARTPHONE} count="3">
                { "Three devices are enrolled." }
            </Tab>
            <Tab value="credentials" label="Credentials" icon={icons::KEY_ROUND} count="12">
                { "Twelve certificates have been issued." }
            </Tab>
            <Tab value="package" label="Config package" icon={icons::PACKAGE} disabled=true>
                { "Not available until a device is enrolled." }
            </Tab>
        </Tabs>
    }
}
