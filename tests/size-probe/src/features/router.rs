use yew::prelude::*;
use yew_router::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hosts")]
    Hosts,
}

pub fn view() -> Html {
    html! {
        <BrowserRouter>
            <RouteLink<Route> to={Route::Home}>{ "Home" }</RouteLink<Route>>
            <RouteLink<Route> to={Route::Hosts}>{ "Hosts" }</RouteLink<Route>>
        </BrowserRouter>
    }
}
