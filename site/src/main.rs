//! The browser entry point, built by Trunk.

#[cfg(target_arch = "wasm32")]
fn main() {
    use yew_router::history::{AnyHistory, BrowserHistory, History, MemoryHistory};
    use yewi_site::{App, AppProps};

    let document = web_sys::window().and_then(|window| window.document()).expect("no document");
    let root = document.get_element_by_id("app").expect("no #app element");

    // The packed review gallery (`e2e/viewer.mjs`) is one file with no address
    // of its own, so it keeps its location in memory and starts at the gallery.
    let gallery = root.get_attribute("data-gallery");
    let props = AppProps {
        gallery: gallery.is_some(),
        history: match gallery {
            Some(slug) => {
                let history = MemoryHistory::new();
                history.push(format!("{}/_gallery/{slug}", yewi_site::basename().unwrap_or_default()));
                AnyHistory::from(history)
            }
            None => AnyHistory::from(BrowserHistory::new()),
        },
    };

    // A prerendered page carries its markup and a stylesheet link; `trunk serve`
    // hands over an empty shell.
    #[cfg(feature = "hydration")]
    if root.has_attribute("data-prerendered") {
        yew::Renderer::<App>::with_root_and_props(root, props).hydrate();
        return;
    }

    yewi::install_styles();
    yew::Renderer::<App>::with_root_and_props(root, props).render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("yewi-site runs in the browser: use `cargo xtask serve`, or the `prerender` binary.");
}
