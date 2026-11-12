//! Breadcrumbs: where the current page sits, and the way back up.

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, NavTarget, nav_target::use_nav_link};

/// What is drawn between crumbs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BreadcrumbSeparator {
    /// A forward slash.
    #[default]
    Slash,
    /// A right-pointing chevron.
    Chevron,
}

impl BreadcrumbSeparator {
    pub const ALL: [BreadcrumbSeparator; 2] = [BreadcrumbSeparator::Slash, BreadcrumbSeparator::Chevron];

    const fn as_str(self) -> &'static str {
        match self {
            BreadcrumbSeparator::Slash => "slash",
            BreadcrumbSeparator::Chevron => "chevron",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbsProps {
    /// The [`Crumb`]s, from the top of the hierarchy down to this page.
    #[prop_or_default]
    pub children: Html,
    /// Accessible name of the navigation landmark.
    #[prop_or(AttrValue::Static("Breadcrumb"))]
    pub label: AttrValue,
    #[prop_or_default]
    pub separator: BreadcrumbSeparator,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A trail of links from the top of a hierarchy down to the current page.
///
/// Separators are drawn in CSS and hidden from assistive technology, so the
/// trail reads as a plain list of links.
#[function_component]
pub fn Breadcrumbs(props: &BreadcrumbsProps) -> Html {
    let class = classes!(
        "yi-breadcrumbs",
        format!("yi-breadcrumbs--{}", props.separator.as_str()),
        props.class.clone()
    );

    props.attrs.apply(html! {
        <nav id={props.id.clone()} {class} aria-label={props.label.clone()}>
            <ol class="yi-breadcrumbs__list">{ props.children.clone() }</ol>
        </nav>
    })
}

#[derive(Properties, PartialEq)]
pub struct CrumbProps {
    /// The label.
    #[prop_or_default]
    pub children: Html,
    /// Where the crumb leads: a URL or, with the `router` feature,
    /// `NavTarget::route(..)`. A crumb without one is plain text.
    #[prop_or_default]
    pub href: NavTarget,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Whether this is the page being shown. Defaults to `true` for a crumb
    /// without `href`.
    #[prop_or_default]
    pub current: Option<bool>,
    /// Fired on click, before any navigation.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One step of a [`Breadcrumbs`] trail.
#[function_component]
pub fn Crumb(props: &CrumbProps) -> Html {
    let link = use_nav_link(&props.href, props.on_click.clone());
    let current = props.current.unwrap_or(link.href.is_none());
    let aria_current = current.then_some("page");

    let content = html! {
        <>
            if let Some(icon) = props.icon {
                <Icon {icon} />
            }
            <span class="yi-breadcrumbs__label">{ props.children.clone() }</span>
        </>
    };

    let crumb = props.attrs.apply(match link.href {
        Some(href) => html! {
            <a
                id={props.id.clone()}
                class={classes!("yi-breadcrumbs__crumb", "yi-breadcrumbs__crumb--link", props.class.clone())}
                {href}
                aria-current={aria_current}
                onclick={link.onclick}
            >
                { content }
            </a>
        },
        None => html! {
            <span
                id={props.id.clone()}
                class={classes!("yi-breadcrumbs__crumb", props.class.clone())}
                aria-current={aria_current}
            >
                { content }
            </span>
        },
    });

    html! { <li class="yi-breadcrumbs__item">{ crumb }</li> }
}

#[cfg(test)]
mod tests;
