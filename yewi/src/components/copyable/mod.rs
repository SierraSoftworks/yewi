//! Copy to the clipboard: a value with a copy button beside it, the button
//! on its own, and the hook both are built on.
//!
//! - [`Copyable`] shows a value (a commit, an id, a token, a URL) and copies
//!   it. What is shown may be shorter than what is copied.
//! - [`CopyButton`] is the trigger alone, for toolbars and headers where the
//!   content is already on screen (a code block, a JSON payload).
//! - [`use_clipboard`] is for a component that needs its own trigger.
//!
//! The async Clipboard API is used where the browser offers it, with the
//! older `execCommand` path as a fallback so copying also works on a page
//! served over plain HTTP. When both are refused the failure is shown and
//! announced, and `Copyable` selects its value so it can be copied by hand.

use yew::prelude::*;

use crate::{Attrs, Button, ButtonVariant, Size, components::tooltip::use_hint, icons, is_empty, use_id};

mod dom;

/// How long the copied or failed state is shown before returning to idle.
const FEEDBACK_MS: i32 = 2_000;

/// Where a copy stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CopyState {
    /// Nothing copied recently.
    #[default]
    Idle,
    /// The value reached the clipboard a moment ago.
    Copied,
    /// The browser refused access to the clipboard a moment ago.
    Failed,
}

impl CopyState {
    /// The value of the `data-yi-state` attribute, `None` when idle.
    pub const fn as_attr(self) -> Option<&'static str> {
        match self {
            CopyState::Idle => None,
            CopyState::Copied => Some("copied"),
            CopyState::Failed => Some("failed"),
        }
    }
}

/// The handle [`use_clipboard`] returns.
#[derive(Clone, PartialEq)]
pub struct Clipboard {
    /// The outcome of the latest copy. Returns to `Idle` after two seconds.
    pub state: CopyState,
    /// Copies the emitted text. Call it from a user gesture such as a click:
    /// browsers refuse clipboard writes at any other time.
    pub copy: Callback<AttrValue>,
}

/// Clipboard access with feedback state, for building a custom copy trigger.
///
/// The first render always reports [`CopyState::Idle`], and nothing touches
/// the browser until `copy` is called, so it is safe in server renders.
#[hook]
pub fn use_clipboard() -> Clipboard {
    let state = use_state_eq(CopyState::default);
    let reset = use_mut_ref(|| None::<dom::Timer>);

    let copy = {
        let state = state.clone();
        use_callback((), move |text: AttrValue, _| {
            let (state, reset) = (state.clone(), reset.clone());
            dom::write(&text, move |copied| {
                state.set(if copied { CopyState::Copied } else { CopyState::Failed });
                let timer = dom::Timer::after(FEEDBACK_MS, move || state.set(CopyState::Idle));
                *reset.borrow_mut() = timer;
            });
        })
    };

    Clipboard { state: *state, copy }
}

#[derive(Properties, PartialEq)]
pub struct CopyButtonProps {
    /// The text put on the clipboard.
    pub value: AttrValue,
    /// A visible label beside the icon. Without one the button is a square
    /// holding only the icon, named by `label`.
    #[prop_or_default]
    pub children: Html,
    /// The accessible name and hint of an icon-only button.
    #[prop_or(AttrValue::Static("Copy"))]
    pub label: AttrValue,
    /// Shown and announced once the value is on the clipboard.
    #[prop_or(AttrValue::Static("Copied"))]
    pub copied_label: AttrValue,
    /// Shown and announced when the browser refuses the clipboard.
    #[prop_or(AttrValue::Static("Copy failed"))]
    pub failed_label: AttrValue,
    #[prop_or(ButtonVariant::Subtle)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub disabled: bool,
    /// Fired with the value once it is on the clipboard.
    #[prop_or_default]
    pub on_copy: Callback<AttrValue>,
    /// Fired when the browser refuses the clipboard.
    #[prop_or_default]
    pub on_error: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A button that copies `value`, then confirms with a check mark for two
/// seconds. The outcome is also announced to assistive technology.
#[function_component]
pub fn CopyButton(props: &CopyButtonProps) -> Html {
    let clipboard = use_clipboard();
    let state = clipboard.state;

    {
        let (value, on_copy, on_error) = (props.value.clone(), props.on_copy.clone(), props.on_error.clone());
        let (copied, failed) = (props.copied_label.clone(), props.failed_label.clone());
        use_effect_with(state, move |state| match state {
            CopyState::Idle => dom::announce(""),
            CopyState::Copied => {
                dom::announce(&copied);
                on_copy.emit(value);
            }
            CopyState::Failed => {
                dom::announce(&failed);
                on_error.emit(());
            }
        });
    }

    let on_click = {
        let (copy, value) = (clipboard.copy.clone(), props.value.clone());
        Callback::from(move |_| copy.emit(value.clone()))
    };

    let icon = match state {
        CopyState::Idle => icons::COPY,
        CopyState::Copied => icons::CHECK,
        CopyState::Failed => icons::X,
    };
    let feedback = match state {
        CopyState::Idle => None,
        CopyState::Copied => Some(&props.copied_label),
        CopyState::Failed => Some(&props.failed_label),
    };
    let icon_only = is_empty(&props.children);

    // The name stays put while the hint reports the outcome: renaming a
    // focused button is announced inconsistently, the live region is not.
    let node_ref = use_node_ref();
    let hint_text = if icon_only { feedback.unwrap_or(&props.label).clone() } else { AttrValue::Static("") };
    let hint = use_hint(&node_ref, &hint_text);

    let mut attrs = Vec::new();
    if icon_only {
        attrs.push(("aria-label", props.label.clone()));
        // The hint where there is no runtime to show the tooltip.
        attrs.push(("title", hint_text));
    }
    if let Some(state) = state.as_attr() {
        attrs.push(("data-yi-state", AttrValue::Static(state)));
    }
    attrs.extend(props.attrs.iter().map(|(key, value)| (key, value.clone())));

    html! {
        <Button
            id={props.id.clone()}
            class={classes!("yi-copy-button", icon_only.then_some("yi-button--icon-only"), props.class.clone())}
            variant={props.variant}
            size={props.size}
            disabled={props.disabled}
            {icon}
            attrs={Attrs::new(attrs)}
            {on_click}
            {node_ref}
        >
            if !icon_only {
                if let Some(feedback) = feedback {
                    { feedback.clone() }
                } else {
                    { props.children.clone() }
                }
            }
            { hint }
        </Button>
    }
}

/// How a [`Copyable`] presents its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CopyableVariant {
    /// Plain text with the button after it, for values in running text,
    /// tables and description lists.
    #[default]
    Inline,
    /// The value and button share a tinted chip, to set a value apart from
    /// the prose around it.
    Chip,
    /// A full-width outlined box, the read-only counterpart of a text input,
    /// for tokens, URLs and commands the user is expected to take away.
    Field,
}

impl CopyableVariant {
    pub const ALL: [CopyableVariant; 3] = [CopyableVariant::Inline, CopyableVariant::Chip, CopyableVariant::Field];

    const fn as_str(self) -> &'static str {
        match self {
            CopyableVariant::Inline => "inline",
            CopyableVariant::Chip => "chip",
            CopyableVariant::Field => "field",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CopyableProps {
    /// The text put on the clipboard.
    pub value: AttrValue,
    /// What is shown, when it differs from `value`: a shortened hash, a
    /// formatted number. Defaults to the value itself.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub variant: CopyableVariant,
    /// Sets the value in the monospace face. Turn it off for prose such as a
    /// name or an address.
    #[prop_or(true)]
    pub mono: bool,
    /// Keeps the value on one line and cuts it with an ellipsis. By default
    /// a long value wraps, breaking anywhere, so all of it can be read.
    #[prop_or_default]
    pub truncate: bool,
    /// The accessible name and tooltip of the copy button. The value is
    /// attached as its description, so buttons in a list can be told apart.
    #[prop_or(AttrValue::Static("Copy"))]
    pub label: AttrValue,
    /// Shown and announced once the value is on the clipboard.
    #[prop_or(AttrValue::Static("Copied"))]
    pub copied_label: AttrValue,
    /// Shown and announced when the browser refuses the clipboard.
    #[prop_or(AttrValue::Static("Copy failed"))]
    pub failed_label: AttrValue,
    /// Fired with the value once it is on the clipboard.
    #[prop_or_default]
    pub on_copy: Callback<AttrValue>,
    /// Fired when the browser refuses the clipboard. The value has been
    /// selected by then, ready to copy by hand.
    #[prop_or_default]
    pub on_error: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A value with a button that copies it to the clipboard.
#[function_component]
pub fn Copyable(props: &CopyableProps) -> Html {
    let value_id = use_id();
    let value_ref = use_node_ref();

    let on_error = {
        let (value_ref, on_error) = (value_ref.clone(), props.on_error.clone());
        Callback::from(move |()| {
            dom::select(&value_ref);
            on_error.emit(());
        })
    };

    let class = classes!(
        "yi-copyable",
        format!("yi-copyable--{}", props.variant.as_str()),
        props.mono.then_some("yi-copyable--mono"),
        props.truncate.then_some("yi-copyable--truncate"),
        props.class.clone(),
    );

    let shown = if is_empty(&props.children) { html! { { props.value.clone() } } } else { props.children.clone() };
    // A cut-off value is still readable on hover.
    let title = props.truncate.then(|| props.value.clone());

    props.attrs.apply(html! {
        <span id={props.id.clone()} {class}>
            <span ref={value_ref} id={value_id.clone()} class="yi-copyable__value" {title}>{ shown }</span>
            <CopyButton
                class="yi-copyable__button"
                value={props.value.clone()}
                label={props.label.clone()}
                copied_label={props.copied_label.clone()}
                failed_label={props.failed_label.clone()}
                size={Size::Small}
                on_copy={props.on_copy.clone()}
                {on_error}
                attrs={crate::attrs! { "aria-describedby" => value_id }}
            />
        </span>
    })
}

#[cfg(test)]
mod tests;
