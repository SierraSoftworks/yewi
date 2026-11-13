//! A button with a menu: the likeliest action on its face, the rest behind
//! a chevron.

use yew::prelude::*;

use crate::{Attrs, Button, ButtonKind, ButtonVariant, IconData, MenuButton, Placement, Size, icons, is_empty};

#[derive(Properties, PartialEq)]
pub struct SplitButtonProps {
    /// The other actions, as the menu's items: `MenuItem`, `MenuGroup` and
    /// `MenuSeparator`. With none there is no chevron, only the button.
    #[prop_or_default]
    pub children: Html,
    /// The label of the action on the button's face.
    pub label: AttrValue,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: Size,
    /// Unavailable: neither the action nor the menu can be used.
    #[prop_or_default]
    pub disabled: bool,
    /// Only the action on the face is unavailable; the menu still opens. For
    /// a row where the usual action does not apply but the others do.
    #[prop_or_default]
    pub face_disabled: bool,
    /// The action is in progress: the face shows a spinner and ignores
    /// clicks. The menu stays available.
    #[prop_or_default]
    pub loading: bool,
    /// The native button type of the face.
    #[prop_or_default]
    pub kind: ButtonKind,
    /// What the chevron opens, for assistive technology and as its hover
    /// hint.
    #[prop_or(AttrValue::Static("More actions"))]
    pub menu_label: AttrValue,
    /// Where the menu opens. It flips and slides to stay on screen.
    #[prop_or(Placement::BottomEnd)]
    pub placement: Placement,
    /// Fired when the face is activated. Not fired while `disabled` or
    /// `loading`.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
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

/// Several actions on one thing as a single control: the one most likely
/// wanted on the button's face, the rest in a menu behind the chevron.
///
/// The face and the chevron are separate tab stops, and the menu behaves as
/// `MenuButton`'s does.
#[function_component]
pub fn SplitButton(props: &SplitButtonProps) -> Html {
    let has_menu = !is_empty(&props.children);

    props.attrs.apply(html! {
        <span id={props.id.clone()} class={classes!("yi-split-button", props.class.clone())} role="group">
            <Button
                icon={props.icon}
                variant={props.variant}
                size={props.size}
                disabled={props.disabled || props.face_disabled}
                loading={props.loading}
                kind={props.kind}
                on_click={props.on_click.clone()}
                class="yi-split-button__face"
            >
                { props.label.clone() }
            </Button>
            if has_menu {
                <MenuButton
                    label={props.menu_label.clone()}
                    icon={icons::CHEVRON_DOWN}
                    icon_only=true
                    variant={props.variant}
                    size={props.size}
                    disabled={props.disabled}
                    placement={props.placement}
                    on_open_change={props.on_open_change.clone()}
                    class="yi-split-button__toggle"
                >
                    { props.children.clone() }
                </MenuButton>
            }
        </span>
    })
}

#[cfg(test)]
mod tests;
