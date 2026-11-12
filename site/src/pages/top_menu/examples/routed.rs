use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    prelude::*,
};
use yewi::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/workflows")]
    Workflows,
    #[at("/connections")]
    Connections,
    #[at("/activity")]
    Activity,
}

fn page(route: Route) -> Html {
    html! { <p role="status">{ format!("Route: {}", route.to_path()) }</p> }
}

#[function_component]
pub fn Example() -> Html {
    // An app has one `BrowserRouter` at its root. The example keeps its
    // routes in memory so that it does not navigate the page it is shown on.
    let history = use_memo((), |_| AnyHistory::from(MemoryHistory::with_entries(["/workflows"])));

    html! {
        <Router history={(*history).clone()}>
            <Stack>
                <TopMenu>
                    <TopMenuItem href={NavTarget::route(Route::Workflows)}>{ "Workflows" }</TopMenuItem>
                    <TopMenuItem href={NavTarget::route(Route::Connections)}>{ "Connections" }</TopMenuItem>
                    <TopMenuItem href={NavTarget::route(Route::Activity)}>{ "Activity" }</TopMenuItem>
                </TopMenu>
                <Switch<Route> render={page} />
            </Stack>
        </Router>
    }
}
