use std::rc::Rc;

use yew::prelude::*;

use super::target_value;
use crate::{Attrs, Icon, Size, icons, use_field_control};

/// Reads what the user has typed. `None` means "not a number yet" (`-`,
/// `1e`), which is left alone so the field does not fight the user
/// mid-keystroke; `Some(None)` means the field is empty.
pub(crate) fn parse(text: &str) -> Option<Option<f64>> {
    let text = text.trim();
    if text.is_empty() {
        return Some(None);
    }
    // Rust also parses `inf` and `nan`, which are not numbers a user types.
    if !text.bytes().all(|b| b.is_ascii_digit() || matches!(b, b'+' | b'-' | b'.' | b',' | b'e' | b'E')) {
        return None;
    }
    text.replace(',', ".").parse::<f64>().ok().filter(|v| v.is_finite()).map(Some)
}

pub(crate) fn format(value: f64) -> String {
    // Avoids `-0`.
    if value == 0.0 { "0".into() } else { value.to_string() }
}

/// Digits after the decimal point in the shortest form of `value`.
fn decimals(value: f64) -> i32 {
    format(value).split_once('.').map_or(0, |(_, fraction)| fraction.len() as i32)
}

pub(crate) fn clamp(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let value = max.map_or(value, |max| value.min(max));
    min.map_or(value, |min| value.max(min))
}

/// `current` moved by `steps` steps and kept within bounds. An empty field
/// counts from zero. The result is rounded to the precision of its inputs, so
/// `0.1 + 0.2` is `0.3`.
pub(crate) fn stepped(current: Option<f64>, steps: f64, step: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let base = current.unwrap_or(0.0);
    let scale = 10f64.powi(decimals(step).max(decimals(base)));
    clamp(((base + steps * step) * scale).round() / scale, min, max)
}

/// What the input shows while the user edits it, which can be ahead of or
/// behind the parsed `value` (`1.` and `1.0` are both 1).
struct Draft {
    text: String,
    value: Option<f64>,
    /// The `value` prop last seen, to tell a change by the caller from an edit.
    prop: Option<f64>,
    /// The value last reported through `on_change`.
    committed: Option<f64>,
}

#[derive(Properties, PartialEq)]
pub struct NumberInputProps {
    /// The number shown. `None` is an empty field, which is distinct from
    /// zero. Update it from `on_input`; whenever it changes, the field shows
    /// the new number.
    #[prop_or_default]
    pub value: Option<f64>,
    /// Fired on every edit that reads as a number, and when the field is
    /// emptied. While typing, the value may be outside `min` and `max`.
    #[prop_or_default]
    pub on_input: Callback<Option<f64>>,
    /// Fired when the user commits a new value: on blur or Enter (after
    /// clamping to `min` and `max`) and on every step.
    #[prop_or_default]
    pub on_change: Callback<Option<f64>>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    /// Fired after the input has handled its own keys (arrows, Page Up and
    /// Down, Home, End).
    #[prop_or_default]
    pub on_keydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub min: Option<f64>,
    #[prop_or_default]
    pub max: Option<f64>,
    /// How far the arrow keys and stepper buttons move the value. Page Up
    /// and Page Down move ten steps.
    #[prop_or(1.0)]
    pub step: f64,
    /// Shows decrease and increase buttons inside the field. Turn it off for
    /// numbers that are typed rather than nudged (a port, an id); the arrow
    /// keys step the value either way.
    #[prop_or(true)]
    pub stepper: bool,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// The name the value is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
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
    /// Accessible name of the decrease button.
    #[prop_or(AttrValue::Static("Decrease"))]
    pub decrease_label: AttrValue,
    /// Accessible name of the increase button.
    #[prop_or(AttrValue::Static("Increase"))]
    pub increase_label: AttrValue,
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

/// A number, typed or stepped with the arrow keys. An empty field is `None`,
/// not zero.
#[function_component]
pub fn NumberInput(props: &NumberInputProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
    let update = use_force_update();
    let draft = use_mut_ref(|| Draft {
        text: props.value.map(format).unwrap_or_default(),
        value: props.value,
        prop: props.value,
        committed: props.value,
    });

    // The caller changed the value to something other than what was typed:
    // show theirs. A prop that merely lags behind the text is left alone, so
    // the field also works when nothing listens to `on_input`.
    {
        let mut draft = draft.borrow_mut();
        if draft.prop != props.value {
            draft.prop = props.value;
            if draft.value != props.value {
                draft.text = props.value.map(format).unwrap_or_default();
                draft.value = props.value;
                draft.committed = props.value;
            }
        }
    }

    let (min, max, step) = (props.min, props.max, props.step);
    let editable = !control.disabled && !props.readonly;

    // Replaces the text with `value`, reporting it if it is new.
    let set: Rc<dyn Fn(Option<f64>)> = {
        let (draft, update) = (draft.clone(), update.clone());
        let (on_input, on_change) = (props.on_input.clone(), props.on_change.clone());
        Rc::new(move |value: Option<f64>| {
            let (edited, changed) = {
                let mut draft = draft.borrow_mut();
                let flags = (draft.value != value, draft.committed != value);
                draft.text = value.map(format).unwrap_or_default();
                draft.value = value;
                draft.committed = value;
                flags
            };
            if edited {
                on_input.emit(value);
            }
            if changed {
                on_change.emit(value);
            }
            update.force_update();
        })
    };
    let commit = {
        let (draft, set) = (draft.clone(), set.clone());
        move || {
            let value = draft.borrow().value.map(|value| clamp(value, min, max));
            set(value);
        }
    };
    let step_by = {
        let (draft, set) = (draft.clone(), set.clone());
        move |steps: f64| {
            let current = draft.borrow().value;
            set(Some(stepped(current, steps, step, min, max)));
        }
    };

    let oninput = {
        let (draft, update, on_input) = (draft.clone(), update.clone(), props.on_input.clone());
        Callback::from(move |event: InputEvent| {
            let text = target_value(&event);
            let parsed = parse(&text);
            let edited = {
                let mut draft = draft.borrow_mut();
                draft.text = text;
                match parsed {
                    Some(value) if value != draft.value => {
                        draft.value = value;
                        true
                    }
                    _ => false,
                }
            };
            if edited && let Some(value) = parsed {
                on_input.emit(value);
                update.force_update();
            }
        })
    };
    let onblur = {
        let (commit, on_blur) = (commit.clone(), props.on_blur.clone());
        Callback::from(move |event: FocusEvent| {
            commit();
            on_blur.emit(event);
        })
    };
    let onkeydown = {
        let (commit, draft, set) = (commit.clone(), draft.clone(), set.clone());
        let on_keydown = props.on_keydown.clone();
        Callback::from(move |event: KeyboardEvent| {
            if editable {
                let current = draft.borrow().value;
                let step_to = |steps: f64| Some(stepped(current, steps, step, min, max));
                let target = match event.key().as_str() {
                    "ArrowUp" => step_to(1.0),
                    "ArrowDown" => step_to(-1.0),
                    "PageUp" => step_to(10.0),
                    "PageDown" => step_to(-10.0),
                    "Home" => min,
                    "End" => max,
                    // Not prevented: Enter still submits the form.
                    "Enter" => {
                        commit();
                        None
                    }
                    _ => None,
                };
                if let Some(target) = target {
                    event.prevent_default();
                    set(Some(target));
                }
            }
            on_keydown.emit(event);
        })
    };

    let value = draft.borrow().value;
    let stepper_button = |label: &AttrValue, icon, steps: f64, at_limit: bool| {
        let step_by = step_by.clone();
        html! {
            <button
                type="button"
                class="yi-number-input__step"
                tabindex="-1"
                aria-label={label.clone()}
                disabled={!editable || at_limit}
                // Keeps focus (and the caret) in the input.
                onmousedown={Callback::from(|event: MouseEvent| event.prevent_default())}
                onclick={Callback::from(move |_| step_by(steps))}
            >
                <Icon {icon} />
            </button>
        }
    };

    let fractional = [Some(step), min, max].into_iter().flatten().any(|n| n.fract() != 0.0);
    // Numeric keypads have no minus key, so only offer one when it is not needed.
    let inputmode = min.filter(|min| *min >= 0.0).map(|_| if fractional { "decimal" } else { "numeric" });

    let input = props.attrs.apply(html! {
        <input
            ref={props.input_ref.clone()}
            id={control.id.clone()}
            class="yi-number-input__input"
            type="text"
            role="spinbutton"
            {inputmode}
            autocomplete="off"
            value={draft.borrow().text.clone()}
            name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            disabled={control.disabled}
            readonly={props.readonly}
            required={control.required}
            aria-valuenow={value.map(format)}
            aria-valuemin={min.map(format)}
            aria-valuemax={max.map(format)}
            aria-invalid={control.aria_invalid()}
            aria-describedby={control.described_by.clone()}
            {oninput}
            {onblur}
            {onkeydown}
            onfocus={props.on_focus.clone()}
        />
    });

    html! {
        <div
            class={classes!(
                "yi-input",
                "yi-number-input",
                format!("yi-input--{}", props.size.as_str()),
                control.invalid.then_some("yi-input--invalid"),
                control.disabled.then_some("yi-input--disabled"),
                props.readonly.then_some("yi-input--readonly"),
                props.class.clone(),
            )}
        >
            { input }
            if props.stepper {
                <span class="yi-number-input__steps">
                    { stepper_button(
                        &props.decrease_label,
                        icons::MINUS,
                        -1.0,
                        value.zip(min).is_some_and(|(value, min)| value <= min),
                    ) }
                    { stepper_button(
                        &props.increase_label,
                        icons::PLUS,
                        1.0,
                        value.zip(max).is_some_and(|(value, max)| value >= max),
                    ) }
                </span>
            }
        </div>
    }
}
