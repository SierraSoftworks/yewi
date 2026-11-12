use yew::prelude::*;
use yewi::prelude::*;


const PAGES: [(&str, IconData); 5] = [
    ("Overview", icons::LAYOUT_DASHBOARD),
    ("Rollouts", icons::ROCKET),
    ("Hosts", icons::SERVER),
    ("Workloads", icons::PACKAGE),
    ("Audit", icons::SCROLL_TEXT),
];

#[function_component]
pub fn Example() -> Html {
    let page = use_state(|| "Overview");

    html! {
        <Stack>
            <Sidebar label="Sections" attrs={attrs! { "style" => "max-inline-size: 14rem" }}>
                { for PAGES.into_iter().map(|(name, icon)| {
                    let on_click = {
                        let page = page.clone();
                        Callback::from(move |_| page.set(name))
                    };
                    html! { <SidebarItem {icon} current={*page == name} {on_click}>{ name }</SidebarItem> }
                }) }
            </Sidebar>
            <span role="status">{ format!("Showing {}", *page) }</span>
        </Stack>
    }
}
