//! The browser entry point, built by Trunk.

#[cfg(target_arch = "wasm32")]
fn main() {
    use yew_router::history::{AnyHistory, BrowserHistory};
    use yewi_site::{App, AppProps};

    let props = AppProps { history: AnyHistory::from(BrowserHistory::new()) };
    let document = web_sys::window().and_then(|window| window.document()).expect("no document");
    let root = document.get_element_by_id("app").expect("no #app element");

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
