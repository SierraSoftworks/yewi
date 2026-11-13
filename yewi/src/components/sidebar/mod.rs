//! The navigation down the side of an application: a column of
//! [`SidebarItem`]s, optionally gathered into labelled [`SidebarGroup`]s.
//!
//! It is usually the `sidebar` of an [`AppShell`](crate::AppShell), which
//! places it beside the page or in a drawer, but it works in any column.

use yew::prelude::*;

use crate::is_empty;
use crate::{Attrs, Icon, IconData, NavTarget, components::app_shell::use_app_shell, icons, nav_target::use_nav_link};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    /// [`SidebarItem`]s and [`SidebarGroup`]s.
    #[prop_or_default]
    pub children: Html,
    /// Content pinned to the bottom, set apart from the destinations: a
    /// documentation link, a version.
    #[prop_or_default]
    pub footer: Html,
    /// The accessible name of the navigation landmark.
    #[prop_or(AttrValue::Static("Navigation"))]
    pub label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A vertical list of destinations.
#[function_component]
pub fn Sidebar(props: &SidebarProps) -> Html {
    props.attrs.apply(html! {
        <nav id={props.id.clone()} class={classes!("yi-sidebar", props.class.clone())} aria-label={props.label.clone()}>
            <div class="yi-sidebar__items">{ props.children.clone() }</div>
            if !is_empty(&props.footer) {
                <div class="yi-sidebar__footer">{ props.footer.clone() }</div>
            }
        </nav>
    })
}

#[derive(Properties, PartialEq)]
pub struct SidebarGroupProps {
    /// What the destinations have in common.
    pub label: AttrValue,
    /// The group's [`SidebarItem`]s.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Destinations under a shared label, for when there are too many to scan
/// as one list.
#[function_component]
pub fn SidebarGroup(props: &SidebarGroupProps) -> Html {
    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!("yi-sidebar__group", props.class.clone())}
            role="group"
            aria-label={props.label.clone()}
        >
            // The group's name already says this to assistive technology.
            <div class="yi-sidebar__heading" aria-hidden="true">{ props.label.clone() }</div>
            { props.children.clone() }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct SidebarItemProps {
    /// The destination's name.
    #[prop_or_default]
    pub children: Html,
    /// Where it goes: a URL or, with the `router` feature,
    /// `NavTarget::route(..)`. A `NavTarget::external` link opens in a new
    /// tab and says so. Without `href` the item is a button and `on_click`
    /// does the work.
    #[prop_or_default]
    pub href: NavTarget,
    /// An icon before the name. Give every item in a list one, or none.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Whether this is where the user is now. A route target works this out
    /// from the router's location; set it for plain links, or to also mark
    /// the section's item on the pages beneath it.
    #[prop_or_default]
    pub current: Option<bool>,
    /// One of a set beneath the item above it, such as the projects under a
    /// dashboard: indented, with a dot in place of an icon.
    #[prop_or_default]
    pub nested: bool,
    /// Something at the end of the row: a count, a status dot.
    #[prop_or_default]
    pub trailing: Html,
    /// Fired on activation, before any navigation.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One destination in a [`Sidebar`].
#[function_component]
pub fn SidebarItem(props: &SidebarItemProps) -> Html {
    let shell = use_app_shell();

    // Picking a destination closes the drawer it was picked from, so the
    // page it leads to can be seen.
    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |event: MouseEvent| {
            on_click.emit(event);
            if let Some(shell) = &shell {
                shell.close_drawer.emit(());
            }
        })
    };
    let link = use_nav_link(&props.href, onclick.clone());
    let current = props.current.unwrap_or(link.current);

    let class = classes!(
        "yi-sidebar__item",
        props.nested.then_some("yi-sidebar__item--nested"),
        current.then_some("yi-sidebar__item--current"),
        props.class.clone(),
    );

    let content = html! {
        <>
            if let Some(icon) = props.icon {
                <span class="yi-sidebar__icon"><Icon {icon} /></span>
            } else if props.nested {
                <span class="yi-sidebar__dot" aria-hidden="true" />
            }
            <span class="yi-sidebar__label">{ props.children.clone() }</span>
            if !is_empty(&props.trailing) {
                <span class="yi-sidebar__trailing">{ props.trailing.clone() }</span>
            }
            if link.external {
                <span class="yi-sidebar__hint"><Icon icon={icons::EXTERNAL_LINK} label="opens in a new tab" /></span>
            }
        </>
    };

    props.attrs.apply(match link.href {
        Some(href) => html! {
            <a
                id={props.id.clone()}
                {class}
                {href}
                target={link.target}
                rel={link.rel}
                aria-current={current.then_some("page")}
                onclick={link.onclick}
            >
                { content }
            </a>
        },
        None => html! {
            <button id={props.id.clone()} {class} type="button" aria-current={current.then_some("true")} {onclick}>
                { content }
            </button>
        },
    })
}

#[cfg(test)]
mod tests;
