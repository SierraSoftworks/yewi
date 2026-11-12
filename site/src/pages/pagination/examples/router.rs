use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    prelude::*,
};
use yewi::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/events")]
    Events,
}

#[function_component]
fn Events() -> Html {
    // The URL owns the page: read it from the query, link to it with `href`.
    let query = use_location().map(|location| location.query_str().to_owned()).unwrap_or_default();
    let page = query.trim_start_matches("?page=").parse().unwrap_or(1_u32);
    let href = Callback::from(|page: u32| NavTarget::route(Route::Events).query([("page", page.to_string())]));

    html! {
        <Stack>
            <Pagination {page} pages=40 {href} />
            <p role="status">{ format!("Route: /events{query}") }</p>
        </Stack>
    }
}

#[function_component]
pub fn Example() -> Html {
    // An app has one `BrowserRouter` at its root. The example keeps its
    // routes in memory so that it does not navigate the page it is shown on.
    let history = use_memo((), |_| AnyHistory::from(MemoryHistory::with_entries(["/events"])));

    html! {
        <Router history={(*history).clone()}>
            <Events />
        </Router>
    }
}
