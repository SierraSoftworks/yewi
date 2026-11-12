use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    prelude::*,
};
use yewi::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Dashboard,
    #[at("/exceptions")]
    Exceptions,
    #[at("/exceptions/:name")]
    Exception { name: String },
}

#[function_component]
fn Trail() -> Html {
    let route = use_route::<Route>().unwrap_or(Route::Dashboard);
    let location = use_location().map(|location| location.query_str().to_owned()).unwrap_or_default();
    // Links back up the trail keep the filters the reader arrived with.
    let filters = [("period", "7d"), ("env", "production")];

    html! {
        <Stack>
            <Breadcrumbs>
                <Crumb href={NavTarget::route(Route::Dashboard).query(filters)}>{ "Dashboard" }</Crumb>
                if route != Route::Dashboard {
                    <Crumb href={NavTarget::route(Route::Exceptions).query(filters)}>{ "Exceptions" }</Crumb>
                }
                if let Route::Exception { name } = &route {
                    <Crumb>{ name.clone() }</Crumb>
                }
            </Breadcrumbs>
            <p role="status">{ format!("Route: {}{location}", route.to_path()) }</p>
        </Stack>
    }
}

#[function_component]
pub fn Example() -> Html {
    // An app has one `BrowserRouter` at its root. The example keeps its
    // routes in memory so that it does not navigate the page it is shown on.
    let history = use_memo((), |_| AnyHistory::from(MemoryHistory::with_entries(["/exceptions/TimeoutError"])));

    html! {
        <Router history={(*history).clone()}>
            <Trail />
        </Router>
    }
}
