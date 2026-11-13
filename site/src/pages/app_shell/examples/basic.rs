use yew::prelude::*;
use yewi::prelude::*;


const PAGES: [(&str, IconData); 6] = [
    ("Overview", icons::LAYOUT_DASHBOARD),
    ("Rollouts", icons::ROCKET),
    ("Hosts", icons::SERVER),
    ("Workloads", icons::PACKAGE),
    ("Clusters", icons::BOXES),
    ("Audit", icons::SCROLL_TEXT),
];

#[function_component]
pub fn Example() -> Html {
    let page = use_state(|| "Overview");

    let bar = html! {
        <AppBar
            brand={html! { <AppBarBrand logo={html! { <Icon icon={icons::BOXES} /> }}>{ "Buckle" }</AppBarBrand> }}
            actions={html! { <UserChip name="Ada Lovelace" detail="admin" size={Size::Small} /> }}
        />
    };

    let footer = html! {
        <SidebarItem href={NavTarget::href("https://buckle.sierrasoftworks.com").external()} icon={icons::BOOK_OPEN}>
            { "Documentation" }
        </SidebarItem>
    };
    let sidebar = html! {
        <Sidebar label="Sections" {footer}>
            { for PAGES.into_iter().map(|(name, icon)| {
                let on_click = {
                    let page = page.clone();
                    Callback::from(move |_| page.set(name))
                };
                html! { <SidebarItem {icon} current={*page == name} {on_click}>{ name }</SidebarItem> }
            }) }
        </Sidebar>
    };

    html! {
        <AppShell embedded=true {bar} {sidebar} attrs={attrs! { "style" => "--yi-app-shell-height: 24rem" }}>
            <Stack>
                <PageHeader title={*page} subtitle={html! { "Everything Buckle manages, in one place." }} />
                <span role="status">{ format!("Showing {}", *page) }</span>
            </Stack>
        </AppShell>
    }
}
