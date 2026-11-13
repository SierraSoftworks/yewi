//! Buttons: the reference implementation every other component follows.
//!
//! [`Button`] is the control, [`IconButton`] is a button whose face is only an
//! icon, and [`ButtonGroup`] joins several buttons into one control.

use yew::prelude::*;

mod group;
mod icon_button;

pub use group::*;
pub use icon_button::*;

use crate::{Attrs, Icon, IconData, NavTarget, Size, Spinner, nav_target::use_nav_link};

/// How much visual weight a button carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ButtonVariant {
    /// A quiet outlined button, for most actions.
    #[default]
    Default,
    /// The one main action of a view, filled with the accent.
    Primary,
    /// No chrome until hovered, for toolbars and repeated row actions.
    Subtle,
    /// A destructive action.
    Danger,
}

impl ButtonVariant {
    pub const ALL: [ButtonVariant; 4] =
        [ButtonVariant::Default, ButtonVariant::Primary, ButtonVariant::Subtle, ButtonVariant::Danger];

    const fn as_str(self) -> &'static str {
        match self {
            ButtonVariant::Default => "default",
            ButtonVariant::Primary => "primary",
            ButtonVariant::Subtle => "subtle",
            ButtonVariant::Danger => "danger",
        }
    }
}

/// What a button does to another element by itself, with no event handler:
/// the native `command` and `commandfor` attributes. This is how a page
/// without a client runtime opens and closes dialogs, drawers and popovers.
///
/// ```ignore
/// <Button command={Command::show_modal("confirm-delete")}>{ "Delete" }</Button>
/// <ConfirmDialog id="confirm-delete" form="delete" title="Delete project?" confirm_label="Delete" />
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Command {
    action: &'static str,
    target: AttrValue,
}

impl Command {
    fn new(action: &'static str, target: impl Into<AttrValue>) -> Self {
        Self { action, target: target.into() }
    }

    /// Opens the `<dialog>` with this id (a `Dialog`, `ConfirmDialog` or `Drawer`) as a modal.
    pub fn show_modal(id: impl Into<AttrValue>) -> Self {
        Self::new("show-modal", id)
    }

    /// Closes the `<dialog>` with this id.
    pub fn close(id: impl Into<AttrValue>) -> Self {
        Self::new("close", id)
    }

    /// Shows the popover with this id if it is hidden, and hides it if it is showing.
    pub fn toggle_popover(id: impl Into<AttrValue>) -> Self {
        Self::new("toggle-popover", id)
    }

    /// Shows the popover with this id.
    pub fn show_popover(id: impl Into<AttrValue>) -> Self {
        Self::new("show-popover", id)
    }

    /// Hides the popover with this id.
    pub fn hide_popover(id: impl Into<AttrValue>) -> Self {
        Self::new("hide-popover", id)
    }
}

/// The native `type` of a `<button>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ButtonKind {
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonKind {
    const fn as_str(self) -> &'static str {
        match self {
            ButtonKind::Button => "button",
            ButtonKind::Submit => "submit",
            ButtonKind::Reset => "reset",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// The label.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: Size,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Unavailable: not focusable and not clickable.
    #[prop_or_default]
    pub disabled: bool,
    /// Work is in progress: shows a spinner and ignores clicks, but stays
    /// focusable so the busy state is announced.
    #[prop_or_default]
    pub loading: bool,
    /// Makes this a toggle button and says whether it is on. Ignored with
    /// `href`. The caller owns the state and flips it in `on_click`.
    #[prop_or_default]
    pub pressed: Option<bool>,
    /// Renders a link that looks like a button: a URL or, with the `router`
    /// feature, `NavTarget::route(..)`.
    #[prop_or_default]
    pub href: NavTarget,
    /// The link `target`. Only used with `href`; an external `href` sets
    /// `_blank`.
    #[prop_or_default]
    pub target: Option<AttrValue>,
    /// The native button type. Ignored with `href`.
    #[prop_or_default]
    pub kind: ButtonKind,
    /// Fired on activation. Not fired while `disabled` or `loading`.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    /// Opens or closes another element natively. Ignored on a link.
    #[prop_or_default]
    pub command: Option<Command>,
    /// A reference to the `<button>` or `<a>`, for moving focus or anchoring
    /// something to it.
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A button, or with `href` a link styled as one.
#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let inert = props.disabled || props.loading;

    let class = classes!(
        "yi-button",
        format!("yi-button--{}", props.variant.as_str()),
        format!("yi-button--{}", props.size.as_str()),
        props.loading.then_some("yi-button--loading"),
        props.class.clone(),
    );

    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |event: MouseEvent| {
            if inert {
                event.prevent_default();
            } else {
                on_click.emit(event);
            }
        })
    };

    let content = html! {
        <>
            if props.loading {
                <Spinner />
            } else if let Some(icon) = props.icon {
                <Icon {icon} />
            }
            { props.children.clone() }
        </>
    };

    let aria_busy = props.loading.then_some("true");
    let link = use_nav_link(&props.href, onclick.clone());
    let (target, rel) = link.target_and_rel(props.target.as_ref(), None);

    props.attrs.apply(match link.href {
        Some(href) => html! {
            <a
                ref={props.node_ref.clone()}
                id={props.id.clone()}
                {class}
                href={(!inert).then_some(href)}
                {target}
                {rel}
                role={inert.then_some("link")}
                aria-disabled={inert.then_some("true")}
                aria-busy={aria_busy}
                onclick={link.onclick}
            >
                { content }
            </a>
        },
        None => html! {
            <button
                ref={props.node_ref.clone()}
                id={props.id.clone()}
                {class}
                type={props.kind.as_str()}
                disabled={props.disabled}
                aria-disabled={(props.loading && !props.disabled).then_some("true")}
                aria-busy={aria_busy}
                aria-pressed={props.pressed.map(|pressed| if pressed { "true" } else { "false" })}
                command={props.command.as_ref().map(|command| command.action)}
                commandfor={props.command.as_ref().map(|command| command.target.clone())}
                {onclick}
            >
                { content }
            </button>
        },
    })
}

#[cfg(test)]
mod tests;
