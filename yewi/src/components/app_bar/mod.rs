//! The bar across the top of an application: the brand at the start, the
//! app's own controls after it, and actions such as the theme toggle and the
//! user menu at the end.
//!
//! Inside an [`AppShell`](crate::AppShell) that has a sidebar it also shows
//! the menu button that opens, hides and shows the navigation.

use yew::prelude::*;

use crate::{
    Attrs, ButtonVariant, IconButton, NavTarget, attrs, components::app_shell::use_app_shell, icons, is_empty,
    nav_target::use_nav_link,
};

#[derive(Properties, PartialEq)]
pub struct AppBarProps {
    /// Content after the brand, aligned to the start: a top menu, a
    /// breadcrumb, an environment badge.
    #[prop_or_default]
    pub children: Html,
    /// The brand, normally an [`AppBarBrand`].
    #[prop_or_default]
    pub brand: Html,
    /// Controls at the end: status, notifications, the theme toggle, the
    /// user menu. A [`TopMenu`](crate::TopMenu) among the children takes
    /// them over, so that they fold into its dropdown when it collapses.
    #[prop_or_default]
    pub actions: Html,
    /// The accessible name of the menu button. The button only appears
    /// inside an [`AppShell`](crate::AppShell) that has a sidebar.
    #[prop_or(AttrValue::Static("Navigation"))]
    pub menu_label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// What an [`AppBar`] shares with the navigation in its start slot.
#[derive(Clone, PartialEq)]
pub(crate) struct AppBarContext {
    /// The bar's `actions`.
    pub actions: Html,
}

/// The bar across the top of an application.
#[function_component]
pub fn AppBar(props: &AppBarProps) -> Html {
    let shell = use_app_shell().filter(|shell| shell.has_sidebar);
    let context = AppBarContext { actions: props.actions.clone() };

    props.attrs.apply(html! {
        <header id={props.id.clone()} class={classes!("yi-app-bar", props.class.clone())}>
            if let Some(shell) = shell {
                <IconButton
                    class="yi-app-bar__menu"
                    icon={icons::MENU}
                    label={props.menu_label.clone()}
                    variant={ButtonVariant::Subtle}
                    attrs={if shell.drawer_command.is_some() {
                        // `BEHAVIOURS` reads the marker, and reports the column
                        // as hidden or shown once the button has done that.
                        attrs! { "aria-controls" => shell.sidebar_id.clone(), "data-yi-shell-toggle" => "" }
                    } else {
                        attrs! {
                            "aria-expanded" => if shell.sidebar_shown { "true" } else { "false" },
                            "aria-controls" => shell.sidebar_id.clone(),
                        }
                    }}
                    command={shell.drawer_command.clone()}
                    on_click={shell.toggle_sidebar.reform(|_| ())}
                />
            }
            if !is_empty(&props.brand) {
                { props.brand.clone() }
            }
            <div class="yi-app-bar__start">
                <ContextProvider<AppBarContext> {context}>{ props.children.clone() }</ContextProvider<AppBarContext>>
            </div>
            // A menu that shows the actions marks itself, and the stylesheet
            // then hides this copy: the bar cannot know while it renders.
            if !is_empty(&props.actions) {
                <div class="yi-app-bar__actions">{ props.actions.clone() }</div>
            }
        </header>
    })
}

#[derive(Properties, PartialEq)]
pub struct AppBarBrandProps {
    /// The application's name.
    #[prop_or_default]
    pub children: Html,
    /// The logo, shown before the name: an [`Icon`](crate::Icon), an `<img>`
    /// or an inline SVG. It is sized to the bar.
    #[prop_or_default]
    pub logo: Html,
    /// Where the brand leads, normally the app's home: a URL or, with the
    /// `router` feature, `NavTarget::route(..)`. Without it the brand is
    /// plain text.
    #[prop_or_default]
    pub href: NavTarget,
    /// The accessible name of the link, when the name alone does not say
    /// where it goes.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Fired when the link is followed, before any navigation.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The application's logo and name, at the start of an [`AppBar`].
#[function_component]
pub fn AppBarBrand(props: &AppBarBrandProps) -> Html {
    let class = classes!("yi-app-bar__brand", props.class.clone());
    let content = html! {
        <>
            if !is_empty(&props.logo) {
                <span class="yi-app-bar__logo">{ props.logo.clone() }</span>
            }
            <span class="yi-app-bar__name">{ props.children.clone() }</span>
        </>
    };

    let link = use_nav_link(&props.href, props.on_click.clone());
    props.attrs.apply(match link.href {
        Some(href) => html! {
            <a
                id={props.id.clone()}
                {class}
                {href}
                target={link.target}
                rel={link.rel}
                aria-label={props.label.clone()}
                onclick={link.onclick}
            >
                { content }
            </a>
        },
        None => html! { <span id={props.id.clone()} {class}>{ content }</span> },
    })
}

#[cfg(test)]
mod tests;
