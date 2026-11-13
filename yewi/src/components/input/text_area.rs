use yew::prelude::*;

use super::{input_classes, target_value};
use crate::{Attrs, Size, use_field_control};

/// How a [`TextArea`] may change size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TextAreaResize {
    /// The user can drag the corner to change the height.
    #[default]
    Vertical,
    /// Fixed at `rows` lines; longer text scrolls.
    None,
    /// Grows with its content, starting at `rows` lines (CSS
    /// `field-sizing: content`). Cap it with `--yi-input-max-height`.
    Auto,
}

impl TextAreaResize {
    const fn as_str(self) -> &'static str {
        match self {
            TextAreaResize::Vertical => "vertical",
            TextAreaResize::None => "none",
            TextAreaResize::Auto => "auto",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {
    /// The text shown. Update it from `on_input` to keep the control
    /// controlled; leave it unset and the browser keeps the text itself,
    /// which suits a native form.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Fired with the new text on every edit.
    #[prop_or_default]
    pub on_input: Callback<String>,
    /// Fired with the text when the user leaves the control, if it changed.
    #[prop_or_default]
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    /// Fired with the text when focus leaves, whether or not it changed: the
    /// moment to trim or validate what was typed.
    #[prop_or_default]
    pub on_blur_value: Callback<String>,
    #[prop_or_default]
    pub on_keydown: Callback<KeyboardEvent>,
    /// The height in lines of text.
    #[prop_or(3)]
    pub rows: u32,
    #[prop_or_default]
    pub resize: TextAreaResize,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// The name the value is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Renders in the monospaced face and stops lines wrapping, for values
    /// whose alignment carries meaning (a filter expression, a config file).
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
    /// A reference to the `<textarea>`, for moving focus.
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

/// Several lines of text.
#[function_component]
pub fn TextArea(props: &TextAreaProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);

    let oninput = props.on_input.reform(|event: InputEvent| target_value(&event));
    let onchange = props.on_change.reform(|event: Event| target_value(&event));

    props.attrs.apply(html! {
        <textarea
            ref={props.input_ref.clone()}
            id={control.id.clone()}
            class={classes!(
                input_classes(Size::Medium, props.mono, control.invalid),
                "yi-input--multiline",
                format!("yi-input--resize-{}", props.resize.as_str()),
                props.class.clone(),
            )}
            rows={props.rows.to_string()}
            value={props.value.clone()}
            name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            maxlength={props.max_length.map(|n| n.to_string())}
            spellcheck={props.mono.then_some("false")}
            disabled={control.disabled}
            readonly={props.readonly}
            required={control.required}
            aria-invalid={control.aria_invalid()}
            aria-describedby={control.described_by.clone()}
            {oninput}
            {onchange}
            onfocus={props.on_focus.clone()}
            onblur={crate::components::input::blur(&props.on_blur, &props.on_blur_value)}
            onkeydown={props.on_keydown.clone()}
        />
    })
}
