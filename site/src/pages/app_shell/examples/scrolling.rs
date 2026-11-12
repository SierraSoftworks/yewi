use yew::prelude::*;
use yewi::prelude::*;


const HOSTS: [&str; 12] =
    ["pi-1", "pi-2", "pi-3", "pi-4", "nuc-1", "nuc-2", "nas-1", "edge-1", "edge-2", "edge-3", "build-1", "build-2"];

#[function_component]
pub fn Example() -> Html {
    let bar = html! { <AppBar brand={html! { <AppBarBrand>{ "Buckle" }</AppBarBrand> }} /> };
    let sidebar = html! {
        <Sidebar label="Sections">
            <SidebarItem icon={icons::LAYOUT_DASHBOARD}>{ "Overview" }</SidebarItem>
            <SidebarItem icon={icons::SERVER} current=true>{ "Hosts" }</SidebarItem>
            <SidebarItem icon={icons::ROCKET}>{ "Rollouts" }</SidebarItem>
        </Sidebar>
    };

    // `embedded` already fills its frame; a full-page shell would pass `fill`.
    html! {
        <AppShell embedded=true {bar} {sidebar} attrs={attrs! { "style" => "--yi-app-shell-height: 20rem" }}>
            <Stack>
                <PageHeader title="Hosts" subtitle={html! { "12 hosts across 3 clusters." }} />
                { for HOSTS.into_iter().map(|host| html! { <Card>{ host }</Card> }) }
            </Stack>
        </AppShell>
    }
}
