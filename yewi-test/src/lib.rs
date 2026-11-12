//! Server-side render helpers for testing Yew components without a browser.
//!
//! ```ignore
//! let html = yewi_test::render(|| html! { <Button>{ "Save" }</Button> });
//! assert!(html.contains("Save"));
//! ```
//!
//! Assert on what a user or assistive technology would notice (text, roles,
//! ARIA state), not on exact markup.

use yew::{BaseComponent, Html, LocalServerRenderer, function_component, html};

/// Renders `view` to an HTML string, without hydration markers.
pub fn render(view: impl Fn() -> Html + 'static) -> String {
    render_with(view, false)
}

/// Renders `view` as a server would for a page that will be hydrated.
pub fn render_hydratable(view: impl Fn() -> Html + 'static) -> String {
    render_with(view, true)
}

/// Renders the component `C` with `props`, without hydration markers.
pub fn render_component<C>(props: C::Properties) -> String
where
    C: BaseComponent,
    C::Properties: Clone,
{
    render(move || html! { <C ..props.clone() /> })
}

fn render_with(view: impl Fn() -> Html + 'static, hydratable: bool) -> String {
    #[derive(yew::Properties)]
    struct Props {
        view: std::rc::Rc<dyn Fn() -> Html>,
    }
    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            std::rc::Rc::ptr_eq(&self.view, &other.view)
        }
    }

    #[function_component]
    fn Harness(props: &Props) -> Html {
        (props.view)()
    }

    let props = Props { view: std::rc::Rc::new(view) };
    block_on(LocalServerRenderer::<Harness>::with_props(props).hydratable(hydratable).render())
}

/// Runs a future to completion on a single-threaded runtime, as SSR needs.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let runtime = tokio::runtime::Builder::new_current_thread().build().expect("failed to start the test runtime");
    tokio::task::LocalSet::new().block_on(&runtime, future)
}
