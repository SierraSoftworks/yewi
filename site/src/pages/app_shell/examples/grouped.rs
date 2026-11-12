use yew::prelude::*;
use yewi::prelude::*;


const GROUPS: [(&str, &[&str]); 3] = [
    ("Overview", &["Dashboard", "Map", "Activity"]),
    ("Clients", &["Users", "Devices", "Profiles"]),
    ("Settings", &["Account", "Security"]),
];

#[function_component]
pub fn Example() -> Html {
    let page = use_state(|| "Dashboard");

    let bar = html! { <AppBar brand={html! { <AppBarBrand>{ "rustak" }</AppBarBrand> }} /> };
    let sidebar = html! {
        <Sidebar label="Admin sections">
            { for GROUPS.into_iter().map(|(label, pages)| html! {
                <SidebarGroup {label}>
                    { for pages.iter().map(|&name| {
                        let on_click = {
                            let page = page.clone();
                            Callback::from(move |_| page.set(name))
                        };
                        html! { <SidebarItem current={*page == name} {on_click}>{ name }</SidebarItem> }
                    }) }
                </SidebarGroup>
            }) }
        </Sidebar>
    };

    html! {
        <AppShell
            embedded=true
            storage_key="yewi.docs.app-shell.collapsed"
            {bar}
            {sidebar}
            attrs={attrs! { "style" => "--yi-app-shell-height: 27rem" }}
        >
            <Stack>
                <PageHeader title={*page} />
                <span role="status">{ format!("Showing {}", *page) }</span>
            </Stack>
        </AppShell>
    }
}
