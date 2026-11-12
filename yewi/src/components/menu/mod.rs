//! Menus: a list of actions, in groups when they are of different kinds.
//!
//! [`Menu`] is the list itself and holds [`MenuItem`], [`MenuGroup`] and
//! [`MenuSeparator`]. `MenuButton` and `SplitButton` show one from a button;
//! use `Menu` directly to show one from anything else.

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, NavTarget, Tone, icons, nav_target::use_nav_link};

/// Which item a menu moves keyboard focus to when it appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum MenuFocus {
    /// Focus stays where it is, for a menu that is always on the page. The
    /// menu itself is then a tab stop.
    #[default]
    None,
    /// The first item, for a menu that was just opened.
    First,
    /// The last item, for a menu opened with <kbd>↑</kbd>.
    Last,
}

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    /// The items: `MenuItem`, `MenuGroup` and `MenuSeparator`.
    #[prop_or_default]
    pub children: Html,
    /// What the actions apply to, for assistive technology.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Where keyboard focus goes when the menu appears. Whatever opens a
    /// menu passes `First` or `Last`.
    #[prop_or_default]
    pub focus: MenuFocus,
    /// Called after an item is chosen, so whatever opened the menu can close
    /// it. The item's own `on_select` has already run.
    #[prop_or_default]
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A list of actions.
///
/// - <kbd>↑</kbd> and <kbd>↓</kbd> move between items and wrap,
///   <kbd>Home</kbd> and <kbd>End</kbd> jump to the ends, and a letter jumps
///   to the next item starting with it. Disabled items are skipped.
/// - The menu draws no surface of its own: it takes that of the popover or
///   card it is in.
/// - Navigation reads the items from the page, so a menu costs no state per
///   item however long it is.
#[function_component]
pub fn Menu(props: &MenuProps) -> Html {
    let node = use_node_ref();

    {
        let node = node.clone();
        use_effect_with(props.focus, move |focus| dom::enter(&node, *focus));
    }

    let onkeydown = {
        let node = node.clone();
        Callback::from(move |event: KeyboardEvent| dom::navigate(&node, &event))
    };
    let onmouseover = {
        let node = node.clone();
        Callback::from(move |event: MouseEvent| dom::follow_pointer(&node, &event))
    };
    let onclick = {
        let on_close = props.on_close.clone();
        Callback::from(move |event: MouseEvent| {
            if dom::chose_item(&event) {
                on_close.emit(());
            }
        })
    };

    let standalone = props.focus == MenuFocus::None;

    props.attrs.apply(html! {
        <span
            ref={node}
            id={props.id.clone()}
            class={classes!("yi-menu", props.class.clone())}
            role="menu"
            aria-label={props.label.clone()}
            tabindex={if standalone { "0" } else { "-1" }}
            {onkeydown}
            {onmouseover}
            {onclick}
        >
            { props.children.clone() }
        </span>
    })
}

#[derive(Properties, PartialEq)]
pub struct MenuItemProps {
    /// The label.
    #[prop_or_default]
    pub children: Html,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Muted text after the label: where the action comes from, or its
    /// keyboard shortcut.
    #[prop_or_default]
    pub aside: Html,
    /// `Danger` marks an action that ends or undoes something and `Warn` one
    /// to take care with. Other tones look like `Neutral`.
    #[prop_or_default]
    pub tone: Tone,
    /// Makes this an option that is on or off, with a check mark when on.
    /// The caller owns the state and flips it in `on_select`.
    #[prop_or_default]
    pub checked: Option<bool>,
    /// Unavailable: shown, but skipped by the keyboard and not chosen by a
    /// click.
    #[prop_or_default]
    pub disabled: bool,
    /// Makes the item a link: a URL or, with the `router` feature,
    /// `NavTarget::route(..)`.
    #[prop_or_default]
    pub href: NavTarget,
    /// The link `target`. Only used with `href`.
    #[prop_or_default]
    pub target: Option<AttrValue>,
    /// Fired when the item is chosen. Not fired while `disabled`.
    #[prop_or_default]
    pub on_select: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One action in a [`Menu`]: a button, or with `href` a link.
#[function_component]
pub fn MenuItem(props: &MenuItemProps) -> Html {
    let disabled = props.disabled;
    let on_select = {
        let on_select = props.on_select.clone();
        Callback::from(move |event: MouseEvent| {
            if disabled {
                event.prevent_default();
            } else {
                on_select.emit(event);
            }
        })
    };
    let link = use_nav_link(&props.href, on_select.clone());

    let class = classes!(
        "yi-menu__item",
        match props.tone {
            Tone::Danger => Some("yi-menu__item--danger"),
            Tone::Warn => Some("yi-menu__item--warn"),
            _ => None,
        },
        props.class.clone(),
    );
    let role = if props.checked.is_some() { "menuitemcheckbox" } else { "menuitem" };
    let aria_checked = props.checked.map(|checked| if checked { "true" } else { "false" });
    let aria_disabled = disabled.then_some("true");

    let content = html! {
        <>
            if let Some(icon) = props.icon {
                <Icon {icon} class="yi-menu__icon" />
            }
            <span class="yi-menu__label">{ props.children.clone() }</span>
            if props.aside != Html::default() {
                <span class="yi-menu__aside">{ props.aside.clone() }</span>
            }
            if props.checked.is_some() {
                <Icon icon={icons::CHECK} class="yi-menu__check" />
            }
        </>
    };

    props.attrs.apply(match link.href {
        Some(href) => html! {
            <a
                id={props.id.clone()}
                {class}
                {role}
                tabindex="-1"
                href={(!disabled).then_some(href)}
                target={props.target.clone()}
                rel={props.target.as_ref().filter(|t| *t == "_blank").map(|_| "noopener")}
                aria-current={link.current.then_some("page")}
                aria-checked={aria_checked}
                aria-disabled={aria_disabled}
                onclick={link.onclick}
            >
                { content }
            </a>
        },
        None => html! {
            <button
                id={props.id.clone()}
                {class}
                {role}
                type="button"
                tabindex="-1"
                aria-checked={aria_checked}
                aria-disabled={aria_disabled}
                onclick={on_select}
            >
                { content }
            </button>
        },
    })
}

#[derive(Properties, PartialEq)]
pub struct MenuGroupProps {
    /// The group's items.
    #[prop_or_default]
    pub children: Html,
    /// A heading for the group. Without one the group is only set apart by a
    /// rule.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A run of related items in a [`Menu`], with an optional heading. A rule is
/// drawn between a group and its neighbours.
#[function_component]
pub fn MenuGroup(props: &MenuGroupProps) -> Html {
    props.attrs.apply(html! {
        <span
            id={props.id.clone()}
            class={classes!("yi-menu__group", props.class.clone())}
            role="group"
            aria-label={props.label.clone()}
        >
            if let Some(label) = &props.label {
                <span class="yi-menu__heading" aria-hidden="true">{ label.clone() }</span>
            }
            { props.children.clone() }
        </span>
    })
}

#[derive(Properties, PartialEq)]
pub struct MenuSeparatorProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A rule between items of a [`Menu`]. Groups, and a dangerous item that
/// follows gentler ones, get one without asking.
#[function_component]
pub fn MenuSeparator(props: &MenuSeparatorProps) -> Html {
    props.attrs.apply(html! {
        <span id={props.id.clone()} class={classes!("yi-menu__separator", props.class.clone())} role="separator" />
    })
}

/// Moves focus to the first or last item of the menu inside `root`, for a
/// trigger that already has its menu open.
pub(crate) fn focus_menu_within(root: &NodeRef, focus: MenuFocus) {
    dom::focus_within(root, focus);
}

/// Moves focus to the first button or link inside `root`: the trigger a menu
/// was opened from.
pub(crate) fn focus_trigger_within(root: &NodeRef) {
    dom::focus_trigger(root);
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};
    use web_sys::{Element, Event, HtmlElement, KeyboardEvent, MouseEvent, Node};
    use yew::NodeRef;

    use super::MenuFocus;

    const MENU: &str = "[role=\"menu\"]";
    const ITEM: &str = "[role^=\"menuitem\"]";
    const ENABLED_ITEM: &str = "[role^=\"menuitem\"]:not([aria-disabled=\"true\"])";

    /// The items the keyboard can reach, in order. Called by name: `NodeList`
    /// is not among the `web-sys` features the crate enables.
    fn items(menu: &Element) -> Vec<HtmlElement> {
        let found = js_sys::Reflect::get(menu, &"querySelectorAll".into())
            .ok()
            .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
            .and_then(|f| f.call1(menu, &ENABLED_ITEM.into()).ok());
        found
            .map(|list| js_sys::Array::from(&list).iter().filter_map(|item| item.dyn_into().ok()).collect())
            .unwrap_or_default()
    }

    fn active() -> Option<Element> {
        web_sys::window()?.document()?.active_element()
    }

    fn item_of(event: &Event) -> Option<Element> {
        event.target()?.dyn_into::<Element>().ok()?.closest(ITEM).ok().flatten()
    }

    fn focus_edge(menu: &Element, focus: MenuFocus) -> bool {
        let items = items(menu);
        let item = match focus {
            MenuFocus::None => return true,
            MenuFocus::First => items.first(),
            MenuFocus::Last => items.last(),
        };
        let Some(item) = item else { return true };
        let _ = item.focus();
        active().is_some_and(|active| active == **item)
    }

    /// How many frames to keep trying to focus an item of a menu that cannot
    /// take focus yet.
    const FOCUS_FRAMES: u32 = 30;

    /// Focuses an item as the menu appears. Inside a popover that has not
    /// been placed yet the menu is still hidden and cannot take focus. When
    /// the popover is placed is its own business, and can be a later task
    /// than this effect, so try again once this round of effects is over and
    /// then once a frame until it works.
    pub fn enter(node: &NodeRef, focus: MenuFocus) {
        let Some(menu) = node.cast::<Element>() else { return };
        if focus_edge(&menu, focus) {
            return;
        }
        let retry = Closure::once_into_js(move |_: JsValue| {
            if !focus_edge(&menu, focus) {
                focus_on_frame(menu, focus, FOCUS_FRAMES);
            }
        });
        let settled = js_sys::Promise::resolve(&JsValue::NULL);
        let then = js_sys::Reflect::get(&settled, &"then".into()).ok().and_then(|f| f.dyn_into::<js_sys::Function>().ok());
        if let Some(then) = then {
            let _ = then.call1(&settled, &retry);
        }
    }

    /// Tries again on the next frame, for as long as the menu is on the page
    /// and `frames` remain.
    fn focus_on_frame(menu: Element, focus: MenuFocus, frames: u32) {
        let Some(window) = web_sys::window().filter(|_| frames > 0 && menu.is_connected()) else { return };
        let retry = Closure::once_into_js(move |_: JsValue| {
            if !focus_edge(&menu, focus) {
                focus_on_frame(menu, focus, frames - 1);
            }
        });
        let _ = window.request_animation_frame(retry.unchecked_ref());
    }

    pub fn focus_within(root: &NodeRef, focus: MenuFocus) {
        if let Some(menu) = root.cast::<Element>().and_then(|root| root.query_selector(MENU).ok().flatten()) {
            focus_edge(&menu, focus);
        }
    }

    pub fn focus_trigger(root: &NodeRef) {
        let trigger = root.cast::<Element>().and_then(|root| root.query_selector("button, a[href]").ok().flatten());
        if let Some(trigger) = trigger.and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
            let _ = trigger.focus();
        }
    }

    /// The item to move to for a letter: the next one, after the current,
    /// whose text starts with it.
    fn starting_with(items: &[HtmlElement], current: Option<usize>, letter: &str) -> Option<usize> {
        let letter = letter.to_lowercase();
        let from = current.map_or(0, |i| i + 1);
        (0..items.len()).map(|step| (from + step) % items.len()).find(|&i| {
            items[i].text_content().is_some_and(|text| text.trim_start().to_lowercase().starts_with(&letter))
        })
    }

    pub fn navigate(node: &NodeRef, event: &KeyboardEvent) {
        let Some(menu) = node.cast::<Element>() else { return };
        if event.ctrl_key() || event.meta_key() || event.alt_key() {
            return;
        }
        let items = items(&menu);
        if items.is_empty() {
            return;
        }
        let active = active();
        let current = active.as_ref().and_then(|active| items.iter().position(|item| **item == *active));
        let last = items.len() - 1;
        let key = event.key();

        let next = match key.as_str() {
            "ArrowDown" => Some(current.map_or(0, |i| if i == last { 0 } else { i + 1 })),
            "ArrowUp" => Some(current.map_or(last, |i| if i == 0 { last } else { i - 1 })),
            "Home" => Some(0),
            "End" => Some(last),
            // A link does not follow Space by itself, as a button does.
            " " => {
                if let Some(link) = current.map(|i| &items[i]).filter(|item| item.tag_name() == "A") {
                    event.prevent_default();
                    link.click();
                }
                return;
            }
            letter if letter.chars().count() == 1 => match starting_with(&items, current, letter) {
                Some(found) => Some(found),
                None => return,
            },
            _ => None,
        };
        if let Some(next) = next {
            event.prevent_default();
            let _ = items[next].focus();
        }
    }

    /// Hovering an item moves focus to it once focus is in the menu, so the
    /// pointer and the keyboard never highlight two items at once.
    pub fn follow_pointer(node: &NodeRef, event: &MouseEvent) {
        let (Some(menu), Some(active)) = (node.cast::<Node>(), active()) else { return };
        if !menu.contains(Some(&active)) {
            return;
        }
        let item = item_of(event).filter(|item| *item != active && !item.matches("[aria-disabled=\"true\"]").unwrap_or(true));
        if let Some(item) = item.and_then(|item| item.dyn_into::<HtmlElement>().ok()) {
            let _ = item.focus();
        }
    }

    /// Whether a click landed on an item that can be chosen.
    pub fn chose_item(event: &MouseEvent) -> bool {
        item_of(event).is_some_and(|item| !item.matches("[aria-disabled=\"true\"]").unwrap_or(true))
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    //! Events and effects never run outside a browser.

    use yew::NodeRef;

    use super::MenuFocus;

    pub fn enter(_: &NodeRef, _: MenuFocus) {}

    pub fn focus_within(_: &NodeRef, _: MenuFocus) {}

    pub fn focus_trigger(_: &NodeRef) {}

    pub fn navigate<E>(_: &NodeRef, _: &E) {}

    pub fn follow_pointer<E>(_: &NodeRef, _: &E) {}

    pub fn chose_item<E>(_: &E) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
