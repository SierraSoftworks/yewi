//! An application's main destinations as a row of links, with the current
//! one underlined: [`TopMenu`], its [`TopMenuItem`]s and, for a set of
//! related destinations, [`TopMenuGroup`].
//!
//! It goes in the start slot of an [`AppBar`](crate::AppBar), or stands on
//! its own in any header. Where there is no room for the row it collapses
//! into one menu button, which opens the bar's actions and the same
//! destinations as a dropdown.

use yew::prelude::*;

use crate::{
    Attrs, ButtonVariant, Icon, IconButton, IconData, Menu, MenuFocus, MenuGroup, MenuItem, NavTarget, Placement,
    Popover, PopoverTrigger,
    app_bar::AppBarContext,
    attrs, icons, is_empty,
    menu::{focus_menu_within, focus_trigger_within},
    nav_target::use_nav_link,
};

/// When a [`TopMenu`] gives up its row of links for a single menu button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TopMenuCollapse {
    /// When the menu has less than about 30rem to itself: a phone, or a
    /// crowded bar. For up to five or six short labels.
    #[default]
    Narrow,
    /// When the menu has less than about 48rem to itself. For a longer row.
    Wide,
    /// Always a menu button.
    Always,
    /// Never: a row too long for its space scrolls sideways instead.
    Never,
}

impl TopMenuCollapse {
    pub const ALL: [TopMenuCollapse; 4] =
        [TopMenuCollapse::Narrow, TopMenuCollapse::Wide, TopMenuCollapse::Always, TopMenuCollapse::Never];

    const fn as_str(self) -> &'static str {
        match self {
            TopMenuCollapse::Narrow => "narrow",
            TopMenuCollapse::Wide => "wide",
            TopMenuCollapse::Always => "always",
            TopMenuCollapse::Never => "never",
        }
    }
}

/// Which of the two renderings of a [`TopMenu`] an item is part of.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Mode {
    /// The row of links.
    #[default]
    Row,
    /// The menu behind the collapsed button.
    Menu,
}

#[derive(Properties, PartialEq)]
pub struct TopMenuProps {
    /// The [`TopMenuItem`]s and [`TopMenuGroup`]s, in order.
    #[prop_or_default]
    pub children: Html,
    /// Controls at the end of the row: the theme toggle, the user menu, a
    /// status. They fold into the dropdown with the destinations, as a row
    /// above them. Inside an [`AppBar`](crate::AppBar) leave this empty: the
    /// menu takes the bar's `actions`.
    #[prop_or_default]
    pub actions: Html,
    /// Accessible name of the navigation landmark.
    #[prop_or(AttrValue::Static("Main"))]
    pub label: AttrValue,
    /// When the row gives way to a single menu button. The width measured is
    /// the menu's own, so it must be given room: it fills an `AppBar`'s
    /// start slot or a block by itself.
    #[prop_or_default]
    pub collapse: TopMenuCollapse,
    /// The accessible name of the button the menu collapses into.
    #[prop_or(AttrValue::Static("Menu"))]
    pub menu_label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// An application's main destinations, as a row of links.
///
/// - A navigation landmark of real links, so destinations open in a new tab
///   and work before any script has loaded.
/// - The current destination is underlined; inside an `AppBar` the underline
///   sits on the bar's lower edge.
/// - Without room for the row it becomes one menu button at the end. Its
///   dropdown holds the actions in a row, then each destination as a row
///   of its own. The switch is made by the stylesheet, so it is right on the
///   first paint of a server-rendered page.
/// - The actions are its own `actions` or, when it has none, those of the
///   `AppBar` it is in.
#[function_component]
pub fn TopMenu(props: &TopMenuProps) -> Html {
    // Without actions of its own the menu shows the bar's, and says so with
    // a class, so that the bar hides its own copy.
    let bar = use_context::<AppBarContext>().filter(|bar| is_empty(&props.actions) && !is_empty(&bar.actions));
    let actions = bar.as_ref().map_or_else(|| props.actions.clone(), |bar| bar.actions.clone());
    let has_actions = !is_empty(&actions);

    let class = classes!(
        "yi-top-menu",
        format!("yi-top-menu--collapse-{}", props.collapse.as_str()),
        bar.is_some().then_some("yi-top-menu--bar-actions"),
        props.class.clone()
    );

    props.attrs.apply(html! {
        <nav id={props.id.clone()} {class} aria-label={props.label.clone()}>
            if props.collapse != TopMenuCollapse::Always {
                <ul class="yi-top-menu__list" role="list">
                    <ContextProvider<Mode> context={Mode::Row}>{ props.children.clone() }</ContextProvider<Mode>>
                </ul>
                if has_actions {
                    <div class="yi-top-menu__actions">{ actions.clone() }</div>
                }
            }
            if props.collapse != TopMenuCollapse::Never {
                <Compact label={props.label.clone()} menu_label={props.menu_label.clone()} {actions}>
                    { props.children.clone() }
                </Compact>
            }
        </nav>
    })
}

/// What opening and closing a menu from a trigger needs: shared by the
/// collapsed dropdown and [`TopMenuGroup`].
struct MenuTrigger {
    /// Wraps the trigger and its popover.
    root: NodeRef,
    shown: bool,
    /// The item focused as the menu opens.
    entry: MenuFocus,
    set_open: Callback<bool>,
    /// Closes the menu and returns focus to the trigger.
    on_close: Callback<()>,
    onkeydown: Callback<KeyboardEvent>,
}

#[hook]
fn use_menu_trigger(on_open_change: Callback<bool>) -> MenuTrigger {
    let root = use_node_ref();
    let open = use_state(|| false);
    let entry = use_state(|| MenuFocus::First);
    let shown = *open;

    let set_open = {
        let (open, entry) = (open.clone(), entry.clone());
        Callback::from(move |next: bool| {
            if !next {
                entry.set(MenuFocus::First);
            }
            open.set(next);
            on_open_change.emit(next);
        })
    };
    let on_close = {
        let (root, set_open) = (root.clone(), set_open.clone());
        Callback::from(move |()| {
            focus_trigger_within(&root);
            set_open.emit(false);
        })
    };
    // The arrow keys open the menu from the button, or move into it when it
    // is already open. Inside the menu they belong to the menu.
    let onkeydown = {
        let (root, entry, set_open) = (root.clone(), entry.clone(), set_open.clone());
        Callback::from(move |event: KeyboardEvent| {
            let focus = match event.key().as_str() {
                "ArrowDown" => MenuFocus::First,
                "ArrowUp" => MenuFocus::Last,
                _ => return,
            };
            if event.default_prevented() {
                return;
            }
            event.prevent_default();
            if shown {
                focus_menu_within(&root, focus);
            } else {
                entry.set(focus);
                set_open.emit(true);
            }
        })
    };

    MenuTrigger { root, shown, entry: *entry, set_open, on_close, onkeydown }
}

#[derive(Properties, PartialEq)]
struct CompactProps {
    children: Html,
    actions: Html,
    label: AttrValue,
    menu_label: AttrValue,
}

/// The collapsed form: a menu button whose dropdown holds the actions in a
/// row and the destinations as a menu beneath it.
#[function_component]
fn Compact(props: &CompactProps) -> Html {
    let menu = use_menu_trigger(Callback::noop());

    let header = if is_empty(&props.actions) {
        Html::default()
    } else {
        html! { <span class="yi-top-menu__menu-actions">{ props.actions.clone() }</span> }
    };
    let content = html! {
        <Menu label={props.label.clone()} focus={menu.entry} on_close={menu.on_close}>
            <ContextProvider<Mode> context={Mode::Menu}>{ props.children.clone() }</ContextProvider<Mode>>
        </Menu>
    };

    html! {
        <div ref={menu.root} class="yi-top-menu__compact" onkeydown={menu.onkeydown}>
            <Popover
                trigger={PopoverTrigger::Click}
                open={menu.shown}
                on_open_change={menu.set_open}
                placement={Placement::BottomEnd}
                arrow=false
                flush=true
                label={props.menu_label.clone()}
                surface_class="yi-top-menu__surface"
                {header}
                {content}
            >
                <IconButton
                    icon={icons::MENU}
                    label={props.menu_label.clone()}
                    variant={ButtonVariant::Subtle}
                    attrs={attrs! { "aria-expanded" => if menu.shown { "true" } else { "false" } }}
                />
            </Popover>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TopMenuItemProps {
    /// The label.
    #[prop_or_default]
    pub children: Html,
    /// Where the item leads: a URL or, with the `router` feature,
    /// `NavTarget::route(..)`. A `NavTarget::external` link opens in a new
    /// tab and says so. Without `href` the item is a button.
    #[prop_or_default]
    pub href: NavTarget,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Whether this is the page being shown. A route target works this out
    /// from the router's location; set it for plain links, or to also mark
    /// the item on the pages beneath it.
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

/// One destination of a [`TopMenu`].
#[function_component]
pub fn TopMenuItem(props: &TopMenuItemProps) -> Html {
    let mode = use_context::<Mode>().unwrap_or_default();
    let link = use_nav_link(&props.href, props.on_click.clone());
    let current = props.current.unwrap_or(link.current);

    if mode == Mode::Menu {
        let state = if current { attrs! { "aria-current" => "page" } } else { Attrs::default() };
        return html! {
            <MenuItem
                id={props.id.clone()}
                class={classes!("yi-top-menu__menu-item", props.class.clone())}
                href={props.href.clone()}
                icon={props.icon}
                on_select={props.on_click.clone()}
                attrs={state}
            >
                { props.children.clone() }
            </MenuItem>
        };
    }

    let class = classes!("yi-top-menu__link", props.class.clone());
    let content = html! {
        <>
            if let Some(icon) = props.icon {
                <Icon {icon} />
            }
            <span class="yi-top-menu__label">{ props.children.clone() }</span>
            if link.external {
                <Icon icon={icons::EXTERNAL_LINK} label="opens in a new tab" class="yi-top-menu__external" />
            }
        </>
    };
    let control = props.attrs.apply(match link.href {
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
            <button
                id={props.id.clone()}
                {class}
                type="button"
                aria-current={current.then_some("page")}
                onclick={props.on_click.clone()}
            >
                { content }
            </button>
        },
    });

    html! { <li class="yi-top-menu__item">{ control }</li> }
}

#[derive(Properties, PartialEq)]
pub struct TopMenuGroupProps {
    /// The destinations: `MenuItem`s, normally with `href`.
    #[prop_or_default]
    pub children: Html,
    /// The label of the button that opens the group. In a collapsed menu it
    /// is the heading over the group's items.
    pub label: AttrValue,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Whether the page being shown is one of the group's. The caller knows
    /// its routes; the group does not look inside its items.
    #[prop_or_default]
    pub current: bool,
    /// Called with `true` when the group opens and `false` when it closes.
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Related destinations of a [`TopMenu`] behind one label, which opens them
/// as a menu.
///
/// It behaves as a [`MenuButton`]: a press, <kbd>Enter</kbd>,
/// <kbd>Space</kbd> or <kbd>↓</kbd> opens it, and choosing an item,
/// <kbd>Esc</kbd> or a press outside closes it.
#[function_component]
pub fn TopMenuGroup(props: &TopMenuGroupProps) -> Html {
    let mode = use_context::<Mode>().unwrap_or_default();
    let menu = use_menu_trigger(props.on_open_change.clone());
    let shown = menu.shown;

    if mode == Mode::Menu {
        return props.attrs.apply(html! {
            <MenuGroup id={props.id.clone()} class={props.class.clone()} label={props.label.clone()}>
                { props.children.clone() }
            </MenuGroup>
        });
    }

    let content = html! {
        <Menu label={props.label.clone()} focus={menu.entry} on_close={menu.on_close}>{ props.children.clone() }</Menu>
    };
    let trigger = props.attrs.apply(html! {
        <button
            id={props.id.clone()}
            class={classes!("yi-top-menu__link", props.class.clone())}
            type="button"
            aria-haspopup="menu"
            aria-expanded={if shown { "true" } else { "false" }}
            aria-current={props.current.then_some("true")}
        >
            if let Some(icon) = props.icon {
                <Icon {icon} />
            }
            <span class="yi-top-menu__label">{ props.label.clone() }</span>
            <Icon icon={icons::CHEVRON_DOWN} class="yi-top-menu__caret" />
        </button>
    });

    html! {
        <li ref={menu.root} class="yi-top-menu__item" onkeydown={menu.onkeydown}>
            <Popover
                class="yi-top-menu__popover"
                trigger={PopoverTrigger::Click}
                open={shown}
                on_open_change={menu.set_open}
                placement={Placement::BottomStart}
                arrow=false
                flush=true
                role="none"
                surface_class="yi-top-menu__surface"
                {content}
            >
                { trigger }
            </Popover>
        </li>
    }
}

#[cfg(test)]
mod tests;
