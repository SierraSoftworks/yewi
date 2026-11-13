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
            if !props.gallery {
                <ScrollToTop />
            }
            <Switch<Route> {render} />
        </Router>
    }
}

/// Starts each page at its top. A link followed within the site keeps the
/// document, so without this the new page opens wherever the last one was
/// scrolled to. Back and forward are left to the browser, which restores
/// the position the reader left, and so is a link to a place on the same page.
#[function_component]
fn ScrollToTop() -> Html {
    let path = use_location().map(|location| location.path().to_owned());
    let shown = use_mut_ref(|| None::<String>);
    let going_back = use_mut_ref(|| false);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{JsCast, closure::Closure};

        let going_back = going_back.clone();
        use_effect_with((), move |()| {
            let noted = Closure::<dyn Fn()>::new(move || *going_back.borrow_mut() = true);
            let window = web_sys::window();
            if let Some(window) = &window {
                // Capturing, so this runs before the router hears the same event and renders.
                let _ =
                    window.add_event_listener_with_callback_and_bool("popstate", noted.as_ref().unchecked_ref(), true);
            }
            move || {
                if let Some(window) = window {
                    let _ = window.remove_event_listener_with_callback_and_bool(
                        "popstate",
                        noted.as_ref().unchecked_ref(),
                        true,
                    );
                }
            }
        });
    }

    // After every render: the router also renders this for a move within a
    // page, and that must use up the note that the browser moved.
    use_effect(move || {
        let back = going_back.replace(false);
        let moved = shown.borrow().is_some() && *shown.borrow() != path;
        if moved && !back {
            #[cfg(target_arch = "wasm32")]
            if let Some(window) = web_sys::window() {
                window.scroll_to_with_x_and_y(0.0, 0.0);
            }
        }
        shown.replace(path);
    });

    html! {}
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
