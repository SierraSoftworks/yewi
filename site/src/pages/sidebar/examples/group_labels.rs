use yew::prelude::*;
use yewi::prelude::*;


const SENTENCE_CASE: &str = "max-inline-size: 14rem; --yi-sidebar-heading-transform: none; \
     --yi-sidebar-heading-tracking: 0; --yi-sidebar-heading-size: var(--yi-text-sm); \
     --yi-sidebar-heading-weight: 500";

fn groups() -> Html {
    html! {
        <>
            <SidebarGroup label="Overview">
                <SidebarItem href="#dashboard" current=true>{ "Dashboard" }</SidebarItem>
                <SidebarItem href="#map">{ "Map" }</SidebarItem>
            </SidebarGroup>
            <SidebarGroup label="Client access">
                <SidebarItem href="#users">{ "Users" }</SidebarItem>
                <SidebarItem href="#devices">{ "Devices" }</SidebarItem>
            </SidebarGroup>
        </>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="12rem">
            <Sidebar label="Uppercase labels" attrs={attrs! { "style" => "max-inline-size: 14rem" }}>
                { groups() }
            </Sidebar>
            <Sidebar label="Sentence case labels" attrs={attrs! { "style" => SENTENCE_CASE }}>{ groups() }</Sidebar>
        </Grid>
    }
}
