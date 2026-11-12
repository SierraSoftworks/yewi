//! The signed-in person at the end of an app bar, as a button that opens
//! their menu: who they are, the app's account actions and sign out.

use yew::prelude::*;

use crate::{
    Attrs, Avatar, Icon, Menu, MenuFocus, MenuGroup, MenuItem, NavTarget, Placement, Popover, PopoverTrigger, Size,
    icons,
    menu::{focus_menu_within, focus_trigger_within},
};

#[derive(Properties, PartialEq)]
pub struct UserMenuProps {
    /// The app's own actions for the person: `MenuItem`, `MenuGroup` and
    /// `MenuSeparator`. Sign out follows them.
    #[prop_or_default]
    pub children: Html,
    /// The person's name. It is shown on the button and at the top of the menu.
    pub name: AttrValue,
    /// A second, quieter line: a role or an email address.
    #[prop_or_default]
    pub detail: Option<AttrValue>,
    /// The avatar's picture. See [`Avatar`].
    #[prop_or_default]
    pub src: Option<AttrValue>,
    /// The person's email address, passed to the [`Avatar`]. It is not shown.
    #[prop_or_default]
    pub email: Option<AttrValue>,
    /// What the avatar's colour is derived from. See [`Avatar`].
    #[prop_or_default]
    pub seed: Option<AttrValue>,
    /// Shows only the avatar on the button. The name and detail stay at the
    /// top of the menu and remain the button's accessible name. Narrow
    /// screens get this without asking.
    #[prop_or_default]
    pub compact: bool,
    /// Adds a sign out item at the end of the menu and fires when it is
    /// chosen.
    #[prop_or_default]
    pub on_sign_out: Option<Callback<()>>,
    /// Adds the sign out item as a link, for apps that sign out by visiting
    /// a URL. `on_sign_out` still fires first when both are set.
    #[prop_or_default]
    pub sign_out_href: NavTarget,
    /// The text of the sign out item.
    #[prop_or(AttrValue::Static("Sign out"))]
    pub sign_out_label: AttrValue,
    /// Names the menu for assistive technology.
    #[prop_or(AttrValue::Static("Account"))]
    pub menu_label: AttrValue,
    /// Where the menu opens. It flips and slides to stay on screen.
    #[prop_or(Placement::BottomEnd)]
    pub placement: Placement,
    /// Called with `true` when the menu opens and `false` when it closes.
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The signed-in person as an avatar button that opens their menu. It
/// belongs in the `actions` of an [`AppBar`](crate::AppBar).
///
/// - The button shows the avatar, the name and an optional `detail`;
///   `compact`, or a narrow screen, leaves only the avatar.
/// - The menu opens with who is signed in, then the children, then sign out
///   when `on_sign_out` or `sign_out_href` is set.
/// - It behaves as a [`MenuButton`](crate::MenuButton): a press,
///   <kbd>Enter</kbd>, <kbd>Space</kbd> or <kbd>↓</kbd> opens it on its first
///   item, and choosing an item, <kbd>Esc</kbd> or a press outside closes it.
/// - The menu exists only while open.
#[function_component]
pub fn UserMenu(props: &UserMenuProps) -> Html {
    let root = use_node_ref();
    let open = use_state(|| false);
    let entry = use_state(|| MenuFocus::First);
    let shown = *open;

    let set_open = {
        let (open, entry, on_open_change) = (open.clone(), entry.clone(), props.on_open_change.clone());
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

    let signs_out = props.on_sign_out.is_some() || !props.sign_out_href.is_empty();
    let on_sign_out = props.on_sign_out.clone().unwrap_or_default().reform(|_: MouseEvent| ());

    let identity = html! {
        <span class="yi-user-menu__identity">
            <span class="yi-user-menu__name">{ props.name.clone() }</span>
            if let Some(detail) = &props.detail {
                <span class="yi-user-menu__detail">{ detail.clone() }</span>
            }
        </span>
    };
    let content = html! {
        <Menu label={props.menu_label.clone()} focus={*entry} {on_close}>
            { props.children.clone() }
            if signs_out {
                <MenuGroup>
                    <MenuItem icon={icons::LOG_OUT} href={props.sign_out_href.clone()} on_select={on_sign_out}>
                        { props.sign_out_label.clone() }
                    </MenuItem>
                </MenuGroup>
            }
        </Menu>
    };

    props.attrs.apply(html! {
        <span
            ref={root}
            id={props.id.clone()}
            class={classes!("yi-user-menu", props.compact.then_some("yi-user-menu--compact"), props.class.clone())}
            {onkeydown}
        >
            <Popover
                trigger={PopoverTrigger::Click}
                open={shown}
                on_open_change={set_open}
                placement={props.placement}
                arrow=false
                flush=true
                role="none"
                surface_class="yi-user-menu__surface"
                head={identity.clone()}
                {content}
            >
                <button
                    class="yi-user-menu__trigger"
                    type="button"
                    aria-haspopup="menu"
                    aria-expanded={if shown { "true" } else { "false" }}
                    title={props.compact.then(|| props.name.clone())}
                >
                    <Avatar
                        name={props.name.clone()}
                        src={props.src.clone()}
                        email={props.email.clone()}
                        seed={props.seed.clone()}
                        size={Size::Small}
                        decorative=true
                    />
                    { identity }
                    if !props.compact {
                        <Icon icon={icons::CHEVRON_DOWN} class="yi-user-menu__caret" />
                    }
                </button>
            </Popover>
        </span>
    })
}

#[cfg(test)]
mod tests;
