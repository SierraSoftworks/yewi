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
    /// The person's name. It names the button and is shown once: on the
    /// button where `show_name` puts it there, at the top of the menu otherwise.
    /// `.yi-user-menu__name` is the element showing it.
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
    /// Shows the name, the detail and a chevron beside the avatar, where the
    /// bar has room for them. Without it, and on narrow screens, the button
    /// is the avatar alone: the name and detail move to the top of the menu
    /// and remain the button's accessible name.
    #[prop_or_default]
    pub show_name: bool,
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
/// - The button is the person's avatar. `show_name` adds their name, the
///   optional `detail` and a chevron, except on a narrow screen.
/// - The menu opens with who is signed in, unless the button already says
///   so, then the children, then sign out when `on_sign_out` or
///   `sign_out_href` is set.
/// - It behaves as a [`MenuButton`](crate::MenuButton): a press,
///   <kbd>Enter</kbd>, <kbd>Space</kbd> or <kbd>↓</kbd> opens it on its first
///   item, and choosing an item, <kbd>Esc</kbd> or a press outside closes it.
/// - The menu exists only while open.
#[function_component]
pub fn UserMenu(props: &UserMenuProps) -> Html {
    let root = use_node_ref();
    let open = use_state(|| false);
    let entry = use_state(|| MenuFocus::First);
    // Whether the button was showing the name when the menu opened, which
    // only the stylesheet knows. The menu then leaves it out.
    let on_button = use_state_eq(|| false);
    let shown = *open;

    let set_open = {
        let (open, entry, on_open_change) = (open.clone(), entry.clone(), props.on_open_change.clone());
        let (root, on_button) = (root.clone(), on_button.clone());
        Callback::from(move |next: bool| {
            if next {
                on_button.set(dom::shows_name(&root));
            } else {
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

    // What names a button that shows only the avatar. It is kept apart from
    // the identity, so that is in the page once.
    let label = match &props.detail {
        Some(detail) => format!("{}, {detail}", props.name),
        None => props.name.to_string(),
    };
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
            class={classes!("yi-user-menu", props.show_name.then_some("yi-user-menu--named"), props.class.clone())}
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
                header={if *on_button { Html::default() } else { identity.clone() }}
                {content}
            >
                <button
                    class="yi-user-menu__trigger"
                    type="button"
                    aria-haspopup="menu"
                    aria-expanded={if shown { "true" } else { "false" }}
                    title={(!props.show_name).then(|| props.name.clone())}
                >
                    <Avatar
                        name={props.name.clone()}
                        src={props.src.clone()}
                        email={props.email.clone()}
                        seed={props.seed.clone()}
                        size={Size::Small}
                        decorative=true
                    />
                    if props.show_name {
                        { identity }
                    } else {
                        <span class="yi-user-menu__label">{ label }</span>
                    }
                    if props.show_name {
                        <Icon icon={icons::CHEVRON_DOWN} class="yi-user-menu__caret" />
                    }
                </button>
            </Popover>
        </span>
    })
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use yew::NodeRef;

    /// Whether the stylesheet is showing the name on the button. It says so
    /// with a custom property, so the breakpoint lives in one place.
    pub fn shows_name(root: &NodeRef) -> bool {
        let Some(element) = root.cast::<web_sys::Element>() else { return false };
        web_sys::window()
            .and_then(|window| window.get_computed_style(&element).ok().flatten())
            .and_then(|style| style.get_property_value("--yi-user-menu-shows-name").ok())
            .is_some_and(|value| value.trim() == "1")
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    use yew::NodeRef;

    pub fn shows_name(_: &NodeRef) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
