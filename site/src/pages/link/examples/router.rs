use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, MemoryHistory},
    prelude::*,
};
// `yew_router` has a `Link` of its own, so name the one that is meant.
use yewi::{Link, prelude::*};

#[derive(Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Overview,
    #[at("/hosts")]
    Hosts,
    #[at("/settings")]
    Settings,
}

#[function_component]
fn CurrentPage() -> Html {
    let page = match use_route::<Route>() {
        Some(Route::Hosts) => "Hosts",
        Some(Route::Settings) => "Settings",
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
                <Cluster gap={Space::Lg}>
                    <Link href={NavTarget::route(Route::Overview)}>{ "Overview" }</Link>
                    <Link href={NavTarget::route(Route::Hosts)}>{ "Hosts" }</Link>
                    <Link href={NavTarget::route(Route::Settings)} icon={icons::SETTINGS}>{ "Settings" }</Link>
                </Cluster>
                <CurrentPage />
            </Stack>
        </Router>
    }
}
