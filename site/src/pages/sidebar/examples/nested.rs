use yew::prelude::*;
use yewi::prelude::*;


#[function_component]
pub fn Example() -> Html {
    let footer = html! {
        <SidebarItem href="https://example.com/docs" icon={icons::BOOK_OPEN} external=true>
            { "Documentation" }
        </SidebarItem>
    };

    html! {
        <Sidebar label="Sections" {footer} attrs={attrs! { "style" => "max-inline-size: 14rem" }}>
            <SidebarItem href="#dashboard" icon={icons::LAYOUT_DASHBOARD}>{ "Dashboard" }</SidebarItem>
            <SidebarItem href="#website" nested=true current=true>{ "Website" }</SidebarItem>
            <SidebarItem href="#docs" nested=true>{ "Documentation site" }</SidebarItem>
            <SidebarItem href="#app" nested=true>{ "Mobile app" }</SidebarItem>
            <SidebarItem href="#exceptions" icon={icons::BUG} trailing={html! { "12" }}>{ "Exceptions" }</SidebarItem>
            <SidebarGroup label="Manage">
                <SidebarItem href="#pixels" icon={icons::RADIO}>{ "Tracking pixels" }</SidebarItem>
                <SidebarItem href="#settings" icon={icons::SETTINGS}>{ "Settings" }</SidebarItem>
            </SidebarGroup>
        </Sidebar>
    }
}
