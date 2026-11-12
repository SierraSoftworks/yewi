use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <TabNav label="Settings sections">
            <TabLink href="#general" current=true>{ "General" }</TabLink>
            <TabLink href="#members" count="8">{ "Members" }</TabLink>
            <TabLink href="#tokens">{ "Access tokens" }</TabLink>
            <TabLink href="#billing">{ "Billing" }</TabLink>
        </TabNav>
    }
}
