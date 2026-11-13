//! The application root and its routes.

use yew::prelude::*;
use yew_router::{history::AnyHistory, prelude::*};

use crate::{gallery::Gallery, shell};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/guide/:slug")]
    Guide { slug: String },
    #[at("/components/:slug")]
    Component { slug: String },
    /// The review gallery for one component: see `gallery.rs`.
    #[at("/_gallery")]
    GalleryHome,
    #[at("/_gallery/:slug")]
    Gallery { slug: String },
    /// One example on a bare page: the fixture the browser tests drive.
    #[at("/_examples/:page/:example")]
    Example { page: String, example: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn component(slug: &str) -> Self {
        Route::Component { slug: slug.to_owned() }
    }

    pub fn guide(slug: &str) -> Self {
        Route::Guide { slug: slug.to_owned() }
    }
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    /// Browser history on the client, a memory history at the requested path
    /// on the server. Both go through `Router`, so the trees match.
    pub history: AnyHistory,
    /// The packed review gallery: every address shows the gallery, so a link
    /// followed inside an example cannot replace it.
    #[prop_or_default]
    pub gallery: bool,
}

#[function_component]
pub fn App(props: &AppProps) -> Html {
    // Tells the browser tests (and anyone watching) that the page is interactive.
    use_effect_with((), |_| {
        #[cfg(target_arch = "wasm32")]
        if let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
            let _ = root.set_attribute("data-hydrated", "");
        }
    });

    let shown = use_mut_ref(|| None::<String>);
    let render = if props.gallery {
        Callback::from(move |route| {
            if let Route::Gallery { slug } = route {
                *shown.borrow_mut() = Some(slug);
            }
            html! { <Gallery slug={shown.borrow().clone()} /> }
        })
    } else {
        Callback::from(switch)
    };

    html! {
        <Router history={props.history.clone()} basename={crate::basename()}>
            <Switch<Route> {render} />
        </Router>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <shell::Home /> },
        Route::Guide { slug } => html! { <shell::GuidePage {slug} /> },
        Route::Component { slug } => html! { <shell::ComponentPage {slug} /> },
        Route::GalleryHome => html! { <Gallery /> },
        Route::Gallery { slug } => html! { <Gallery {slug} /> },
        Route::Example { page, example } => html! { <shell::IsolatedExample {page} {example} /> },
        Route::NotFound => html! { <shell::NotFound /> },
    }
}
