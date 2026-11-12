use yew::prelude::*;
use yewi::prelude::*;


#[function_component]
fn NavigationSwitch() -> Html {
    let Some(shell) = use_app_shell() else { return html! {} };
    let label = if shell.sidebar_shown { "Hide navigation" } else { "Show navigation" };

    html! {
        <Cluster>
            <Button icon={icons::PANEL_LEFT} on_click={shell.toggle_sidebar.reform(|_| ())}>{ label }</Button>
        </Cluster>
    }
}

#[function_component]
pub fn Example() -> Html {
    let sidebar = html! {
        <Sidebar label="Sections">
            <SidebarItem icon={icons::LAYOUT_DASHBOARD} current=true>{ "Dashboard" }</SidebarItem>
            <SidebarItem icon={icons::BUG}>{ "Exceptions" }</SidebarItem>
            <SidebarItem icon={icons::SETTINGS}>{ "Settings" }</SidebarItem>
        </Sidebar>
    };

    // No `bar`: the sidebar takes the full height and the page owns the toggle.
    html! {
        <AppShell embedded=true {sidebar} attrs={attrs! { "style" => "--yi-app-shell-height: 16rem" }}>
            <Stack>
                <PageHeader title="Dashboard" />
                <NavigationSwitch />
            </Stack>
        </AppShell>
    }
}
