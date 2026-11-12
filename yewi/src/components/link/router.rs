//! The router-aware link, behind the `router` feature.

use yew::prelude::*;
use yew_router::prelude::*;

use super::{LinkUnderline, LinkVariant, link_classes};
use crate::{Attrs, Icon, IconData};

#[derive(Properties, PartialEq)]
pub struct RouteLinkProps<R: Routable + 'static> {
    /// The link text.
    #[prop_or_default]
    pub children: Html,
    /// The route to navigate to.
    pub to: R,
    #[prop_or_default]
    pub variant: LinkVariant,
    #[prop_or_default]
    pub underline: LinkUnderline,
    /// An icon shown before the text.
    #[prop_or_default]
    pub icon: Option<IconData>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A [`Link`](super::Link) to a `yew-router` route: `<RouteLink<Route> to={Route::Hosts}>`.
///
/// It renders a real `href`, so the link works before the app loads and can
/// be opened in a new tab; a plain click navigates without a page load. The
/// link to the current route carries `aria-current="page"`.
#[function_component]
pub fn RouteLink<R: Routable + 'static>(props: &RouteLinkProps<R>) -> Html {
    let navigator = use_navigator();
    let current = use_route::<R>();

    let path = props.to.to_path();
    let href = match navigator.as_ref().map(Navigator::basename) {
        Some(Some(base)) if path == "/" => base.to_owned(),
        Some(Some(base)) => format!("{base}{path}"),
        _ => path,
    };

    let onclick = {
        let to = props.to.clone();
        Callback::from(move |event: MouseEvent| {
            // Leave modified and non-primary clicks to the browser, so
            // "open in new tab" and friends keep working.
            let modified = event.ctrl_key() || event.meta_key() || event.shift_key() || event.alt_key();
            if event.button() != 0 || modified || event.default_prevented() {
                return;
            }
            if let Some(navigator) = &navigator {
                event.prevent_default();
                navigator.push(&to);
            }
        })
    };

    props.attrs.apply(html! {
        <a
            id={props.id.clone()}
            class={link_classes(props.variant, props.underline, &props.class)}
            {href}
            aria-current={(current.as_ref() == Some(&props.to)).then_some("page")}
            {onclick}
        >
            if let Some(icon) = props.icon {
                <Icon {icon} class="yi-link__icon" />
            }
            { props.children.clone() }
        </a>
    })
}
