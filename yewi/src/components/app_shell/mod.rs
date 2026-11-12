//! The chrome around every page of an application: an [`AppBar`](crate::AppBar)
//! across the top, a [`Sidebar`](crate::Sidebar) down the start edge, and the
//! page itself.
//!
//! The shell owns the navigation's state. Beside the page the sidebar can be
//! hidden and shown; on a narrow screen it moves into a
//! [`Drawer`](crate::Drawer). The app bar's menu button does whichever fits,
//! and following a link in the drawer closes it.

use yew::prelude::*;

use crate::{Attrs, Drawer, DrawerSide, Size, hooks::use_id};

/// What an [`AppShell`] shares with the components inside it. [`AppBar`]
/// and [`SidebarItem`] use it already; read it with [`use_app_shell`] to
/// build a custom menu button or navigation.
///
/// [`AppBar`]: crate::AppBar
/// [`SidebarItem`]: crate::SidebarItem
#[derive(Debug, Clone, PartialEq)]
pub struct AppShellContext {
    /// Whether the shell has a sidebar, and so needs a menu button.
    pub has_sidebar: bool,
    /// The id of the sidebar beside the page, for `aria-controls`.
    pub sidebar_id: AttrValue,
    /// Whether the navigation is showing: the drawer on a narrow screen, the
    /// column beside the page on a wide one.
    pub sidebar_shown: bool,
    /// Opens or closes the drawer on a narrow screen; hides or shows the
    /// column on a wide one.
    pub toggle_sidebar: Callback<()>,
    /// Closes the drawer. Call it when the user picks a destination.
    pub close_drawer: Callback<()>,
}

/// The enclosing [`AppShell`]'s state, or `None` outside one.
#[hook]
pub fn use_app_shell() -> Option<AppShellContext> {
    use_context::<AppShellContext>()
}

#[derive(Properties, PartialEq)]
pub struct AppShellProps {
    /// The page.
    #[prop_or_default]
    pub children: Html,
    /// The bar across the top, normally an [`AppBar`](crate::AppBar).
    #[prop_or_default]
    pub bar: Option<Html>,
    /// The navigation, normally a [`Sidebar`](crate::Sidebar). It is rendered
    /// beside the page and again inside the drawer while that is open, so
    /// avoid fixed `id`s within it.
    #[prop_or_default]
    pub sidebar: Option<Html>,
    /// Whether the sidebar starts hidden on a wide screen.
    #[prop_or_default]
    pub collapsed: bool,
    /// A `localStorage` key under which a wide screen remembers that its
    /// sidebar was hidden. Without one the choice lasts until the page reloads.
    #[prop_or_default]
    pub storage_key: Option<AttrValue>,
    /// The shell takes exactly the height of the viewport and the page area
    /// scrolls inside it, for pages that use every pixel, such as a map.
    /// By default the document scrolls and the bar and sidebar stay in view.
    #[prop_or_default]
    pub fill: bool,
    /// The shell is one part of a page, not the page: a preview or a demo.
    /// It is framed, takes `--yi-app-shell-height`, responds to its own width
    /// and adds no page landmarks or skip link. Implies `fill`.
    #[prop_or_default]
    pub embedded: bool,
    /// The page area has no padding, for content that runs edge to edge,
    /// such as a map.
    #[prop_or_default]
    pub flush: bool,
    /// The accessible name of the navigation drawer.
    #[prop_or(AttrValue::Static("Navigation"))]
    pub drawer_label: AttrValue,
    /// The text of the link, shown on keyboard focus, that jumps past the
    /// bar and sidebar.
    #[prop_or(AttrValue::Static("Skip to content"))]
    pub skip_label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The frame of an application: app bar, sidebar and page.
#[function_component]
pub fn AppShell(props: &AppShellProps) -> Html {
    let sidebar_id = use_id();
    let main_id = use_id();
    let body = use_node_ref();
    // The drawer, on a narrow screen.
    let open = use_state_eq(|| false);
    // The column, on a wide one.
    let collapsed = use_state_eq(|| props.collapsed);
    // Which of the two applies. Only known once mounted: the stylesheet
    // decides, so a server render assumes a wide screen.
    let narrow = use_state_eq(|| false);

    {
        let (body, open, collapsed, narrow) = (body.clone(), open.clone(), collapsed.clone(), narrow.clone());
        use_effect_with(props.storage_key.clone(), move |key| {
            if dom::stored(key.as_deref()) {
                collapsed.set(true);
            }
            narrow.set(dom::is_narrow(&body));
            // Crossing the breakpoint shuts a drawer that the wider layout
            // has no use for, and updates what the menu button reports.
            let listener = dom::on_resize(move || {
                let now = dom::is_narrow(&body);
                if !now {
                    open.set(false);
                }
                narrow.set(now);
            });
            move || {
                let _listener = listener;
            }
        });
    }

    let toggle_sidebar = {
        let (body, open, collapsed, key) = (body.clone(), open.clone(), collapsed.clone(), props.storage_key.clone());
        Callback::from(move |()| {
            if dom::is_narrow(&body) {
                open.set(!*open);
            } else {
                let next = !*collapsed;
                dom::store(key.as_deref(), next);
                collapsed.set(next);
            }
        })
    };
    let close_drawer = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };

    let drawer_open = *open && *narrow;
    let context = AppShellContext {
        has_sidebar: props.sidebar.is_some(),
        sidebar_id: sidebar_id.clone(),
        sidebar_shown: if *narrow { drawer_open } else { !*collapsed },
        toggle_sidebar,
        close_drawer: close_drawer.clone(),
    };

    let class = classes!(
        "yi-app-shell",
        (props.fill || props.embedded).then_some("yi-app-shell--fill"),
        props.embedded.then_some("yi-app-shell--embedded"),
        props.flush.then_some("yi-app-shell--flush"),
        collapsed.then_some("yi-app-shell--collapsed"),
        props.class.clone(),
    );
    // Inside another page the page area is not that page's `main`.
    let main = if props.embedded { "div" } else { "main" };
    // A page area that scrolls must be reachable by keyboard to be scrolled;
    // otherwise it only takes focus from the skip link.
    let tabindex = if props.fill || props.embedded { "0" } else { "-1" };

    html! {
        <ContextProvider<AppShellContext> {context}>
            { props.attrs.apply(html! {
                <div id={props.id.clone()} {class}>
                    if !props.embedded {
                        <a class="yi-app-shell__skip" href={format!("#{main_id}")}>{ props.skip_label.clone() }</a>
                    }
                    if let Some(bar) = &props.bar {
                        { bar.clone() }
                    }
                    <div class="yi-app-shell__body" ref={body}>
                        if let Some(sidebar) = &props.sidebar {
                            <div class="yi-app-shell__sidebar" id={sidebar_id}>{ sidebar.clone() }</div>
                        }
                        <@{main} class="yi-app-shell__main" id={main_id} {tabindex}>
                            { props.children.clone() }
                        </@>
                    </div>
                    if let Some(sidebar) = &props.sidebar {
                        <Drawer
                            class="yi-app-shell__drawer"
                            open={drawer_open}
                            label={props.drawer_label.clone()}
                            side={DrawerSide::Start}
                            size={Size::Small}
                            on_close={close_drawer}
                        >
                            { sidebar.clone() }
                        </Drawer>
                    }
                </div>
            }) }
        </ContextProvider<AppShellContext>>
    }
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use wasm_bindgen::{JsCast, closure::Closure};
    use yew::NodeRef;

    /// Whether the stylesheet has moved the sidebar into the drawer. It says
    /// so with a custom property, so the breakpoint lives in one place.
    pub fn is_narrow(body: &NodeRef) -> bool {
        let Some(element) = body.cast::<web_sys::Element>() else { return false };
        web_sys::window()
            .and_then(|window| window.get_computed_style(&element).ok().flatten())
            .and_then(|style| style.get_property_value("--yi-app-shell-narrow").ok())
            .is_some_and(|value| value.trim() == "1")
    }

    fn storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    pub fn stored(key: Option<&str>) -> bool {
        key.and_then(|key| storage()?.get_item(key).ok().flatten()).is_some_and(|value| value == "1")
    }

    /// A browser that refuses storage still collapses; it only forgets.
    pub fn store(key: Option<&str>, collapsed: bool) {
        let Some((key, storage)) = key.zip(storage()) else { return };
        let _ = if collapsed { storage.set_item(key, "1") } else { storage.remove_item(key) };
    }

    pub struct Listener(Closure<dyn FnMut()>);

    impl Drop for Listener {
        fn drop(&mut self) {
            if let Some(window) = web_sys::window() {
                let _ = window.remove_event_listener_with_callback("resize", self.0.as_ref().unchecked_ref());
            }
        }
    }

    pub fn on_resize(on_change: impl FnMut() + 'static) -> Listener {
        let closure = Closure::<dyn FnMut()>::new(on_change);
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
        }
        Listener(closure)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    use yew::NodeRef;

    pub fn is_narrow(_: &NodeRef) -> bool {
        false
    }

    pub fn stored(_: Option<&str>) -> bool {
        false
    }

    pub fn store(_: Option<&str>, _: bool) {}

    pub struct Listener;

    pub fn on_resize(_: impl FnMut() + 'static) -> Listener {
        Listener
    }
}

#[cfg(test)]
mod tests;
