//! Text entry: [`TextInput`], [`TextArea`] and [`NumberInput`].
//!
//! All three are controlled: they show `value` and report edits through
//! `on_input`, leaving the state with the caller. Inside a
//! [`Field`](crate::Field) they pick up its id, description, and its
//! required, disabled and invalid state.

use yew::prelude::*;

use crate::{Attrs, Size, use_field_control};

mod number_input;
mod text_area;

pub use number_input::*;
pub use text_area::*;

/// The native `type` of a [`TextInput`], which selects the on-screen keyboard
/// and the browser's autofill behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TextInputKind {
    #[default]
    Text,
    Email,
    Url,
    Tel,
    /// Shows dots instead of the value.
    Password,
    /// A search box: Escape clears it in most browsers.
    Search,
}

impl TextInputKind {
    pub const ALL: [TextInputKind; 6] = [
        TextInputKind::Text,
        TextInputKind::Email,
        TextInputKind::Url,
        TextInputKind::Tel,
        TextInputKind::Password,
        TextInputKind::Search,
    ];

    const fn as_str(self) -> &'static str {
        match self {
            TextInputKind::Text => "text",
            TextInputKind::Email => "email",
            TextInputKind::Url => "url",
            TextInputKind::Tel => "tel",
            TextInputKind::Password => "password",
            TextInputKind::Search => "search",
        }
    }
}

/// The current value of the `<input>` or `<textarea>` an event came from.
///
/// Read through reflection so the crate needs no extra `web-sys` features.
pub(crate) fn target_value(event: &Event) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        event
            .target()
            .and_then(|target| js_sys::Reflect::get(&target, &"value".into()).ok())
            .and_then(|value| value.as_string())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = event;
        String::new()
    }
}

/// The classes shared by every text control.
pub(crate) fn input_classes(size: Size, monospace: bool, invalid: bool) -> Classes {
    classes!(
        "yi-input",
        format!("yi-input--{}", size.as_str()),
        monospace.then_some("yi-input--mono"),
        invalid.then_some("yi-input--invalid"),
    )
}

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    /// The text shown. Update it from `on_input` to keep the input
    /// controlled; leave it unset and the browser keeps the text itself,
    /// which suits a native form.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Fired with the new text on every edit.
    #[prop_or_default]
    pub on_input: Callback<String>,
    /// Fired with the text when the user commits it: on blur or Enter, if it
    /// changed.
    #[prop_or_default]
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    /// For shortcuts such as Enter to submit or Escape to cancel.
    #[prop_or_default]
    pub on_keydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub kind: TextInputKind,
    #[prop_or_default]
    pub size: Size,
    /// An example of the expected value. Not a substitute for a label.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// The name the value is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// The native `autocomplete` hint, e.g. `email`, `off` or `new-password`.
    #[prop_or_default]
    pub autocomplete: Option<AttrValue>,
    /// Renders in the monospaced face, for identifiers, tokens and paths.
    #[prop_or_default]
    pub mono: bool,
    /// The most characters the browser will accept.
    #[prop_or_default]
    pub max_length: Option<u32>,
    /// Unavailable: not focusable and not submitted.
    #[prop_or_default]
    pub disabled: bool,
    /// Focusable and selectable, but not editable.
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub required: bool,
    /// Marks the value as having a problem. A surrounding `Field` with an
    /// `error` sets this for you.
    #[prop_or_default]
    pub invalid: bool,
    /// A reference to the `<input>`, for moving focus.
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    /// Ignored inside a `Field`, which assigns the id its label points at.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A single line of text.
#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);

    let oninput = props.on_input.reform(|event: InputEvent| target_value(&event));
    let onchange = props.on_change.reform(|event: Event| target_value(&event));
    let secret = props.kind == TextInputKind::Password;

    props.attrs.apply(html! {
        <input
            ref={props.input_ref.clone()}
            id={control.id.clone()}
            class={classes!(input_classes(props.size, props.mono, control.invalid), props.class.clone())}
            type={props.kind.as_str()}
            value={props.value.clone()}
            name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            autocomplete={props.autocomplete.clone()}
            maxlength={props.max_length.map(|n| n.to_string())}
            spellcheck={(secret || props.mono).then_some("false")}
            disabled={control.disabled}
            readonly={props.readonly}
            required={control.required}
            aria-invalid={control.aria_invalid()}
            aria-describedby={control.described_by.clone()}
            {oninput}
            {onchange}
            onfocus={props.on_focus.clone()}
            onblur={props.on_blur.clone()}
            onkeydown={props.on_keydown.clone()}
        />
    })
}

#[cfg(test)]
mod tests;
