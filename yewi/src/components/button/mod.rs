//! Buttons: the reference implementation every other component follows.

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, Size};

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
    /// Renders a link that looks like a button.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// The link `target`. Only used with `href`.
    #[prop_or_default]
    pub target: Option<AttrValue>,
    /// The native button type. Ignored with `href`.
    #[prop_or_default]
    pub kind: ButtonKind,
    /// Fired on activation. Not fired while `disabled` or `loading`.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
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
        "yw-button",
        format!("yw-button--{}", props.variant.as_str()),
        format!("yw-button--{}", props.size.as_str()),
        props.loading.then_some("yw-button--loading"),
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
                <span class="yw-button__spinner" aria-hidden="true" />
            } else if let Some(icon) = props.icon {
                <Icon {icon} />
            }
            { props.children.clone() }
        </>
    };

    let aria_busy = props.loading.then_some("true");

    props.attrs.apply(match &props.href {
        Some(href) => html! {
            <a
                id={props.id.clone()}
                {class}
                href={(!inert).then(|| href.clone())}
                target={props.target.clone()}
                rel={props.target.as_ref().filter(|t| *t == "_blank").map(|_| "noopener")}
                role={inert.then_some("link")}
                aria-disabled={inert.then_some("true")}
                aria-busy={aria_busy}
                {onclick}
            >
                { content }
            </a>
        },
        None => html! {
            <button
                id={props.id.clone()}
                {class}
                type={props.kind.as_str()}
                disabled={props.disabled}
                aria-disabled={(props.loading && !props.disabled).then_some("true")}
                aria-busy={aria_busy}
                {onclick}
            >
                { content }
            </button>
        },
    })
}

#[cfg(test)]
mod tests;
