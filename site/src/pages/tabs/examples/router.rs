use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    prelude::*,
};
use yewi::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/missions/alpha")]
    Overview,
    #[at("/missions/alpha/members")]
    Members,
    #[at("/missions/alpha/logs")]
    Logs,
}

fn page(route: Route) -> Html {
    html! { <p role="status">{ format!("Route: {}", route.to_path()) }</p> }
}

#[function_component]
pub fn Example() -> Html {
    // An app has one `BrowserRouter` at its root. The example keeps its
    // routes in memory so that it does not navigate the page it is shown on.
    let history = use_memo((), |_| AnyHistory::from(MemoryHistory::with_entries(["/missions/alpha"])));

    html! {
        <Router history={(*history).clone()}>
            <Stack>
                <TabNav label="Mission sections">
                    <TabLink href={NavTarget::route(Route::Overview)}>{ "Overview" }</TabLink>
                    <TabLink href={NavTarget::route(Route::Members)} count="6">{ "Members" }</TabLink>
                    <TabLink href={NavTarget::route(Route::Logs)}>{ "Logs" }</TabLink>
                </TabNav>
                <Switch<Route> render={page} />
            </Stack>
        </Router>
    }
}
