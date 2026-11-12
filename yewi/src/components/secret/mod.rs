//! Secrets in both directions: [`SecretInput`] takes one from the user, and
//! [`SecretReveal`] with [`SecretValue`] hands a freshly minted one over.
//!
//! - [`SecretInput`] masks what is typed until the user asks to see it. The
//!   mask is a default, not a wall: a token has to be checked against the
//!   place it came from, and nobody can do that without reading it back.
//! - [`SecretReveal`] is the panel for a secret the server will not show
//!   again. It says so, offers a copy button, and never closes on its own.
//!
//! Neither component logs or stores the value. A concealed [`SecretValue`]
//! keeps its value out of the document until the user reveals it.

use yew::prelude::*;

use crate::{Attrs, Icon, Size, icons, use_field_control};

mod dom;
mod reveal;

pub use reveal::*;

/// The base64url alphabet (RFC 4648 section 5).
const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// `bytes` as unpadded base64url. Generated secrets end up in headers, query
/// strings and config files, where `+`, `/` and `=` do not always survive.
pub(crate) fn encode_base64url(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk.iter().enumerate().fold(0u32, |n, (i, byte)| n | u32::from(*byte) << (16 - 8 * i));
        for i in 0..=chunk.len() {
            out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
        }
    }
    out
}

#[derive(Properties, PartialEq)]
pub struct SecretInputProps {
    /// The secret. Update it from `on_input` to keep the input controlled;
    /// leave it unset and the browser keeps the text itself, which suits a
    /// native form.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Fired with the new text on every edit, and with a generated secret.
    #[prop_or_default]
    pub on_input: Callback<String>,
    /// Fired with the text when the user commits it: on blur or Enter, if it
    /// changed, and with a generated secret.
    #[prop_or_default]
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    /// For shortcuts such as Enter to submit or Escape to cancel.
    #[prop_or_default]
    pub on_keydown: Callback<KeyboardEvent>,
    /// Whether the text is readable. Leave it unset and the input keeps
    /// track itself, starting masked; set it to control the mask from
    /// `on_reveal`.
    #[prop_or_default]
    pub revealed: Option<bool>,
    /// Fired with the new state when the user shows or hides the text, and
    /// with `true` when a secret is generated.
    #[prop_or_default]
    pub on_reveal: Callback<bool>,
    /// Offers the show and hide button. Turn it off for a secret that must
    /// never be read back from this screen.
    #[prop_or(true)]
    pub revealable: bool,
    /// Offers a button that fills the field with a random secret from the
    /// browser's cryptographic generator, and reveals it.
    #[prop_or_default]
    pub generator: bool,
    /// How many random bytes the generator draws. The secret is their
    /// base64url encoding, so 32 bytes make 43 characters.
    #[prop_or(32)]
    pub generator_bytes: usize,
    #[prop_or_default]
    pub size: Size,
    /// An example of the expected value. Not a substitute for a label.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// The name the value is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// The native `autocomplete` hint. The default keeps password managers
    /// from offering a saved login for an API token; use `current-password`
    /// or `new-password` for an account password.
    #[prop_or(AttrValue::Static("off"))]
    pub autocomplete: AttrValue,
    /// Renders in the monospaced face, for tokens and keys.
    #[prop_or_default]
    pub monospace: bool,
    /// The most characters the browser will accept.
    #[prop_or_default]
    pub max_length: Option<u32>,
    /// Unavailable: not focusable and not submitted.
    #[prop_or_default]
    pub disabled: bool,
    /// Focusable and revealable, but not editable.
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub required: bool,
    /// Marks the value as having a problem. A surrounding `Field` with an
    /// `error` sets this for you.
    #[prop_or_default]
    pub invalid: bool,
    /// Accessible name of the show and hide button. It is a toggle, so the
    /// name stays the same in both states.
    #[prop_or(AttrValue::Static("Show secret"))]
    pub reveal_label: AttrValue,
    /// Accessible name of the generator button.
    #[prop_or(AttrValue::Static("Generate a new secret"))]
    pub generate_label: AttrValue,
    /// A reference to the `<input>`, for moving focus.
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    /// The id of the `<input>`. Ignored inside a `Field`, which assigns the
    /// id its label points at.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Extra attributes for the `<input>` (not the wrapper), so `aria-label`
    /// and the like land on the control.
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A single-line secret (a password, token or key), masked until the user
/// asks to see it.
#[function_component]
pub fn SecretInput(props: &SecretInputProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
    let own_revealed = use_state_eq(|| false);
    let revealed = props.revealed.unwrap_or(*own_revealed);
    let editable = !control.disabled && !props.readonly;

    let set_revealed = {
        let (own, on_reveal) = (own_revealed.clone(), props.on_reveal.clone());
        move |next: bool| {
            own.set(next);
            on_reveal.emit(next);
        }
    };

    let on_toggle = {
        let set_revealed = set_revealed.clone();
        Callback::from(move |_: MouseEvent| set_revealed(!revealed))
    };

    let on_generate = {
        let (on_input, on_change) = (props.on_input.clone(), props.on_change.clone());
        let (input_ref, bytes, controlled) = (props.input_ref.clone(), props.generator_bytes, props.value.is_some());
        Callback::from(move |_: MouseEvent| {
            // Without a source of randomness nothing is filled in: the
            // alternative to a generated secret is one the user chooses, not
            // a predictable one.
            let Some(random) = dom::random_bytes(bytes) else { return };
            let secret = encode_base64url(&random);
            if !controlled {
                dom::set_value(&input_ref, &secret);
            }
            set_revealed(true);
            on_input.emit(secret.clone());
            on_change.emit(secret);
        })
    };

    // Keeps focus (and the caret) in the input when a chip is clicked.
    let keep_focus = Callback::from(|event: MouseEvent| event.prevent_default());

    let input = props.attrs.apply(html! {
        <input
            ref={props.input_ref.clone()}
            id={control.id.clone()}
            class="yi-secret-input__input"
            type={if revealed { "text" } else { "password" }}
            value={props.value.clone()}
            name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            autocomplete={props.autocomplete.clone()}
            autocapitalize="off"
            spellcheck="false"
            maxlength={props.max_length.map(|n| n.to_string())}
            disabled={control.disabled}
            readonly={props.readonly}
            required={control.required}
            aria-invalid={control.aria_invalid()}
            aria-describedby={control.described_by.clone()}
            oninput={props.on_input.reform(|event: InputEvent| crate::components::input::target_value(&event))}
            onchange={props.on_change.reform(|event: Event| crate::components::input::target_value(&event))}
            onfocus={props.on_focus.clone()}
            onblur={props.on_blur.clone()}
            onkeydown={props.on_keydown.clone()}
        />
    });

    html! {
        <div
            class={classes!(
                "yi-input",
                "yi-secret-input",
                format!("yi-input--{}", props.size.as_str()),
                props.monospace.then_some("yi-input--mono"),
                control.invalid.then_some("yi-input--invalid"),
                control.disabled.then_some("yi-input--disabled"),
                props.readonly.then_some("yi-input--readonly"),
                props.class.clone(),
            )}
        >
            { input }
            if props.revealable || props.generator {
                <span class="yi-secret-input__actions">
                    if props.revealable {
                        <button
                            type="button"
                            class="yi-secret-input__action"
                            aria-label={props.reveal_label.clone()}
                            title={props.reveal_label.clone()}
                            aria-pressed={if revealed { "true" } else { "false" }}
                            disabled={control.disabled}
                            onmousedown={keep_focus.clone()}
                            onclick={on_toggle}
                        >
                            <Icon icon={if revealed { icons::EYE_OFF } else { icons::EYE }} />
                        </button>
                    }
                    if props.generator {
                        <button
                            type="button"
                            class="yi-secret-input__action"
                            aria-label={props.generate_label.clone()}
                            title={props.generate_label.clone()}
                            disabled={!editable}
                            onmousedown={keep_focus}
                            onclick={on_generate}
                        >
                            <Icon icon={icons::REFRESH_CW} />
                        </button>
                    }
                </span>
            }
        </div>
    }
}

#[cfg(test)]
mod tests;
