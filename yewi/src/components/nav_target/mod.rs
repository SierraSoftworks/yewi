//! Where a navigation item leads: a plain URL or, with the `router` feature,
//! a `yew-router` route that is followed without reloading the page.
//!
//! Every component that can be a link takes its `href` as a [`NavTarget`], so
//! the same component works in a server-rendered page of plain links and in
//! a routed single-page app.

use std::rc::Rc;

use yew::{html::IntoPropValue, prelude::*};

/// Pushes a route, with the given query, onto a router's history.
#[cfg(feature = "router")]
type Follow = Rc<dyn Fn(&yew_router::navigator::Navigator, &[(String, String)])>;

/// The destination of a navigation link.
///
/// - A string is a plain `href`: `href="/settings"`.
/// - With the `router` feature, [`NavTarget::route`] wraps a typed route. The
///   link renders a real `href` (including the router's basename) and an
///   unmodified click navigates client-side.
/// - [`NavTarget::query`] adds query parameters to either kind.
/// - [`NavTarget::external`] marks a link that leaves the application: it
///   opens in a new tab, and components that show a hint for such links do.
#[derive(Clone, Default)]
pub struct NavTarget {
    path: AttrValue,
    query: Rc<[(String, String)]>,
    external: bool,
    #[cfg(feature = "router")]
    route: Option<Follow>,
}

impl NavTarget {
    /// A plain link to `href`.
    pub fn href(href: impl Into<AttrValue>) -> Self {
        Self { path: href.into(), ..Self::default() }
    }

    /// A link to a `yew-router` route. Inside a router an unmodified click
    /// pushes the route; anywhere else it behaves as a plain link.
    #[cfg(feature = "router")]
    pub fn route<R: yew_router::Routable + 'static>(route: R) -> Self {
        Self {
            path: route.to_path().into(),
            query: Rc::default(),
            external: false,
            route: Some(Rc::new(move |navigator, query| {
                if query.is_empty() {
                    navigator.push(&route);
                } else {
                    let _ = navigator.push_with_query(&route, query);
                }
            })),
        }
    }

    /// Adds query parameters, percent-encoding them into the `href`.
    #[must_use]
    pub fn query<K: Into<String>, V: Into<String>>(mut self, pairs: impl IntoIterator<Item = (K, V)>) -> Self {
        self.query = pairs.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    /// Marks the link as leaving the application: it opens in a new tab
    /// (`target="_blank"`, `rel="noopener noreferrer"`) and is never followed
    /// client-side.
    #[must_use]
    pub fn external(mut self) -> Self {
        self.external = true;
        self
    }

    /// Whether the link leaves the application. See [`NavTarget::external`].
    pub fn is_external(&self) -> bool {
        self.external
    }

    /// Whether this target leads nowhere (the default).
    pub fn is_empty(&self) -> bool {
        self.path.is_empty() && self.query.is_empty()
    }

    fn is_route(&self) -> bool {
        #[cfg(feature = "router")]
        {
            self.route.is_some()
        }
        #[cfg(not(feature = "router"))]
        {
            false
        }
    }

    /// The `href` for a link, given the router's basename and whether it
    /// keeps the route in the URL fragment.
    fn url(&self, basename: Option<&str>, hash: bool) -> String {
        let mut url = String::new();
        if hash {
            url.push('#');
        }
        if let Some(base) = basename.filter(|_| self.is_route()) {
            url.push_str(base.trim_end_matches('/'));
        }
        url.push_str(&self.path);
        for (i, (key, value)) in self.query.iter().enumerate() {
            let has_query = i > 0 || self.path.contains('?');
            url.push(if has_query { '&' } else { '?' });
            encode(key, &mut url);
            url.push('=');
            encode(value, &mut url);
        }
        url
    }
}

impl PartialEq for NavTarget {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.query == other.query
            && self.external == other.external
            && self.is_route() == other.is_route()
    }
}

impl std::fmt::Debug for NavTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NavTarget").field("href", &self.url(None, false)).field("route", &self.is_route()).field("external", &self.external).finish()
    }
}

impl From<&'static str> for NavTarget {
    fn from(href: &'static str) -> Self {
        Self::href(href)
    }
}

impl From<String> for NavTarget {
    fn from(href: String) -> Self {
        Self::href(href)
    }
}

impl From<AttrValue> for NavTarget {
    fn from(href: AttrValue) -> Self {
        Self::href(href)
    }
}

impl IntoPropValue<NavTarget> for &'static str {
    fn into_prop_value(self) -> NavTarget {
        self.into()
    }
}

impl IntoPropValue<NavTarget> for String {
    fn into_prop_value(self) -> NavTarget {
        self.into()
    }
}

impl IntoPropValue<NavTarget> for AttrValue {
    fn into_prop_value(self) -> NavTarget {
        self.into()
    }
}

/// `None` is the empty target, so an optional URL can be passed as it is.
impl<T: Into<NavTarget>> From<Option<T>> for NavTarget {
    fn from(href: Option<T>) -> Self {
        href.map(Into::into).unwrap_or_default()
    }
}

impl<T: Into<NavTarget>> IntoPropValue<NavTarget> for Option<T> {
    fn into_prop_value(self) -> NavTarget {
        self.into()
    }
}

/// Percent-encodes a query key or value.
fn encode(text: &str, out: &mut String) {
    use std::fmt::Write as _;

    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(byte as char),
            _ => write!(out, "%{byte:02X}").unwrap(),
        }
    }
}

/// A [`NavTarget`] resolved against the surrounding router, if any.
pub(crate) struct NavLink {
    /// `None` for an empty target.
    pub href: Option<AttrValue>,
    /// Calls the caller's handler, then follows a route client-side.
    pub onclick: Callback<MouseEvent>,
    /// Whether a routed target is the page being shown. Always `false` for a
    /// plain `href`, which the caller marks as current itself.
    pub current: bool,
    /// Whether the link leaves the application, for showing a hint.
    pub external: bool,
    /// `_blank` for an external link.
    pub target: Option<AttrValue>,
    /// What goes with `target` for an external link.
    pub rel: Option<AttrValue>,
}

impl NavLink {
    fn new(target: &NavTarget, href: Option<AttrValue>, onclick: Callback<MouseEvent>, current: bool) -> Self {
        let external = target.external && href.is_some();
        Self {
            href,
            onclick,
            current,
            external,
            target: external.then_some(AttrValue::Static("_blank")),
            rel: external.then_some(AttrValue::Static(NEW_TAB_REL)),
        }
    }

    /// The `target` and `rel` attributes for a component that also takes
    /// its own: a `target` given by the caller wins, and whatever opens a
    /// new tab gets the protective `rel` added to the caller's.
    pub fn target_and_rel(
        &self,
        target: Option<&AttrValue>,
        rel: Option<&AttrValue>,
    ) -> (Option<AttrValue>, Option<AttrValue>) {
        let target = target.cloned().or_else(|| self.target.clone());
        let new_tab = target.as_ref().is_some_and(|target| target == "_blank");
        let rel = match (rel, new_tab) {
            (Some(rel), true) => Some(AttrValue::from(format!("{rel} {NEW_TAB_REL}"))),
            (None, true) => Some(AttrValue::Static(NEW_TAB_REL)),
            (rel, false) => rel.cloned(),
        };
        (target, rel)
    }
}

/// Keeps the opened page from reaching back to, or learning of, this one.
const NEW_TAB_REL: &str = "noopener noreferrer";

/// Resolves `target` for rendering as an `<a>`.
#[hook]
pub(crate) fn use_nav_link(target: &NavTarget, on_click: Callback<MouseEvent>) -> NavLink {
    #[cfg(feature = "router")]
    {
        use yew_router::{
            navigator::NavigatorKind,
            prelude::{use_location, use_navigator},
        };

        let navigator = use_navigator();
        let location = use_location();

        let basename = navigator.as_ref().and_then(|n| n.basename().map(str::to_owned));
        let hash = navigator.as_ref().is_some_and(|n| n.kind() == NavigatorKind::Hash);
        let href = (!target.is_empty()).then(|| AttrValue::from(target.url(basename.as_deref(), hash)));

        let own_path = target.url(basename.as_deref(), false);
        let own_path = own_path.split(['?', '#']).next().unwrap_or_default().trim_end_matches('/');
        let current =
            target.is_route() && location.is_some_and(|location| location.path().trim_end_matches('/') == own_path);

        // A link that opens a new tab is the browser's to follow.
        let follow = navigator.zip(target.route.clone()).filter(|_| !target.external);
        let query = target.query.clone();
        let onclick = Callback::from(move |event: MouseEvent| {
            on_click.emit(event.clone());
            let plain = event.button() == 0
                && !(event.meta_key() || event.ctrl_key() || event.shift_key() || event.alt_key());
            if let Some((navigator, route)) = follow.as_ref().filter(|_| plain && !event.default_prevented()) {
                event.prevent_default();
                route(navigator, &query);
            }
        });

        NavLink::new(target, href, onclick, current)
    }
    #[cfg(not(feature = "router"))]
    {
        let href = (!target.is_empty()).then(|| AttrValue::from(target.url(None, false)));
        NavLink::new(target, href, on_click, false)
    }
}

#[cfg(test)]
mod tests;
