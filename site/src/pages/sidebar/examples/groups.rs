use yew::prelude::*;
use yewi::prelude::*;


#[function_component]
pub fn Example() -> Html {
    html! {
        <Sidebar label="Admin sections" attrs={attrs! { "style" => "max-inline-size: 14rem" }}>
            <SidebarGroup label="Overview">
                <SidebarItem href="#dashboard" current=true>{ "Dashboard" }</SidebarItem>
                <SidebarItem href="#map">{ "Map" }</SidebarItem>
                <SidebarItem href="#activity">{ "Activity" }</SidebarItem>
            </SidebarGroup>
            <SidebarGroup label="Clients">
                <SidebarItem href="#users">{ "Users" }</SidebarItem>
                <SidebarItem href="#devices">{ "Devices" }</SidebarItem>
            </SidebarGroup>
            <SidebarGroup label="Settings">
                <SidebarItem href="#account">{ "Account" }</SidebarItem>
                <SidebarItem href="#security">{ "Security" }</SidebarItem>
            </SidebarGroup>
        </Sidebar>
    }
}
