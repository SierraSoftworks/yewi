//! A button that opens a [`Menu`] of actions.

use yew::prelude::*;

use crate::{
    Attrs, Button, ButtonVariant, Icon, IconButton, IconData, Menu, MenuFocus, Placement, Popover, PopoverTrigger,
    Size, attrs, icons,
    menu::{focus_menu_within, focus_trigger_within},
};

#[derive(Properties, PartialEq)]
pub struct MenuButtonProps {
    /// The menu's items: `MenuItem`, `MenuGroup` and `MenuSeparator`.
    #[prop_or_default]
    pub children: Html,
    /// The button's label. With `icon_only` it is not shown, and names the
    /// button for assistive technology instead.
    pub label: AttrValue,
    /// An icon shown before the label. With `icon_only` it is the button's
    /// face, and defaults to three vertical dots.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Shows a square button with only the icon, for a row's actions or a
    /// toolbar.
    #[prop_or_default]
    pub icon_only: bool,
    /// Shows a chevron after the label, which says the button opens
    /// something rather than acting.
    #[prop_or(true)]
    pub caret: bool,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: Size,
    /// Unavailable: not focusable, and the menu cannot be opened.
    #[prop_or_default]
    pub disabled: bool,
    /// Where the menu opens. It flips and slides to stay on screen.
    #[prop_or(Placement::BottomStart)]
    pub placement: Placement,
    /// Names the menu for assistive technology. Defaults to `label`.
    #[prop_or_default]
    pub menu_label: Option<AttrValue>,
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

/// A button that opens a menu of actions below it.
///
/// - A press, <kbd>Enter</kbd>, <kbd>Space</kbd> or <kbd>↓</kbd> opens the
///   menu on its first item, and <kbd>↑</kbd> on its last.
/// - Choosing an item, <kbd>Esc</kbd>, a press outside or moving focus away
///   closes it. Focus returns to the button unless it was taken elsewhere.
/// - The menu exists only while open, so a table with one in every row costs
///   a button each.
#[function_component]
pub fn MenuButton(props: &MenuButtonProps) -> Html {
    let root = use_node_ref();
    let open = use_state(|| false);
    let entry = use_state(|| MenuFocus::First);
    let shown = *open && !props.disabled;

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

    let content = html! {
        <Menu label={props.menu_label.clone().unwrap_or_else(|| props.label.clone())} focus={*entry} {on_close}>
            { props.children.clone() }
        </Menu>
    };
    let state = attrs! { "aria-haspopup" => "menu", "aria-expanded" => if shown { "true" } else { "false" } };

    props.attrs.apply(html! {
        <span
            ref={root}
            id={props.id.clone()}
            class={classes!("yi-menu-button", props.class.clone())}
            onkeydown={(!props.disabled).then_some(onkeydown)}
        >
            <Popover
                trigger={PopoverTrigger::Click}
                open={shown}
                on_open_change={set_open}
                placement={props.placement}
                arrow=false
                flush=true
                role="none"
                surface_class="yi-menu-button__surface"
                disabled={props.disabled}
                {content}
            >
                if props.icon_only {
                    <IconButton
                        icon={props.icon.unwrap_or(icons::ELLIPSIS_VERTICAL)}
                        label={props.label.clone()}
                        variant={props.variant}
                        size={props.size}
                        disabled={props.disabled}
                        attrs={state}
                    />
                } else {
                    <Button
                        icon={props.icon}
                        variant={props.variant}
                        size={props.size}
                        disabled={props.disabled}
                        attrs={state}
                    >
                        { props.label.clone() }
                        if props.caret {
                            <Icon icon={icons::CHEVRON_DOWN} class="yi-menu-button__caret" />
                        }
                    </Button>
                }
            </Popover>
        </span>
    })
}

#[cfg(test)]
mod tests;
