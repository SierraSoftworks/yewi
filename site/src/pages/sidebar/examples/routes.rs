use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    prelude::*,
};
use yewi::prelude::*;

#[derive(Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Overview,
    #[at("/hosts")]
    Hosts,
    #[at("/rollouts")]
    Rollouts,
}

#[function_component]
fn CurrentPage() -> Html {
    let page = match use_route::<Route>() {
        Some(Route::Hosts) => "Hosts",
        Some(Route::Rollouts) => "Rollouts",
        _ => "Overview",
    };
    html! { <span role="status">{ format!("Showing: {page}") }</span> }
}

#[function_component]
pub fn Example() -> Html {
    // An app uses `<BrowserRouter>`; the example keeps its history in memory
    // so it does not change this page's address.
    let history = use_memo((), |()| AnyHistory::from(MemoryHistory::new()));

    html! {
        <Router history={(*history).clone()}>
            <Stack>
                <Sidebar attrs={attrs! { "style" => "max-inline-size: 15rem" }}>
                    <SidebarItem href={NavTarget::route(Route::Overview)} icon={icons::LAYOUT_DASHBOARD}>
                        { "Overview" }
                    </SidebarItem>
                    <SidebarItem href={NavTarget::route(Route::Hosts)} icon={icons::SERVER}>{ "Hosts" }</SidebarItem>
                    <SidebarItem href={NavTarget::route(Route::Rollouts)} icon={icons::ROCKET}>{ "Rollouts" }</SidebarItem>
                </Sidebar>
                <CurrentPage />
            </Stack>
        </Router>
    }
}
