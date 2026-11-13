//! [`NativeSelect`]: the platform's own `<select>`, drawn to match the other
//! form controls.
//!
//! It needs no client runtime: the choice is made and submitted by the
//! browser, so it is the select for server-rendered pages that ship no wasm.
//! `Select` is the richer control for hydrated apps.

use std::rc::Rc;

use yew::prelude::*;

use crate::{Attrs, Icon, Size, icons, use_field_control};

/// Sets the `value` of the element behind `node`.
pub(crate) fn set_value(node: &NodeRef, value: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(element) = node.get() {
        let _ = js_sys::Reflect::set(&element, &"value".into(), &value.into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (node, value);
    }
}

#[derive(Properties, PartialEq)]
pub struct NativeSelectProps {
    /// The choices, as `(value, label)` pairs.
    pub options: Rc<[(AttrValue, AttrValue)]>,
    /// The `value` of the selected option. Without a client runtime this is
    /// the initial choice, and the browser keeps it from there. A value that
    /// is not among `options`, such as a stored choice that has since been
    /// withdrawn, is kept and shown as a choice of its own.
    #[prop_or_default]
    pub value: AttrValue,
    /// Fired with the chosen option's `value`, which is empty when a
    /// `clearable` select is cleared. Needs a client runtime; a native form
    /// submits the choice without it.
    #[prop_or_default]
    pub on_change: Callback<String>,
    /// The accessible name, where no `Field` labels the control: in a
    /// toolbar, or in an `InputGroup` affix.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Shown while `value` is empty, and not itself a choice unless the
    /// select is `clearable`. With `required`, the browser blocks submission
    /// until an option is chosen.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// Keeps the empty choice available, labelled by `placeholder`, so that
    /// an optional value can be unset again.
    #[prop_or_default]
    pub clearable: bool,
    /// Labels a `value` that is not among `options`, given that value:
    /// "eu-west-3 (retired)". Without it the value is its own label.
    #[prop_or_default]
    pub unknown_label: Option<Callback<AttrValue, AttrValue>>,
    /// The name the value is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub size: Size,
    /// Sizes the control to its content instead of filling its container.
    #[prop_or_default]
    pub fit: bool,
    /// Unavailable: not focusable and not submitted.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    /// Marks the value as having a problem. A surrounding `Field` with an
    /// `error` sets this for you.
    #[prop_or_default]
    pub invalid: bool,
    #[prop_or_default]
    pub class: Classes,
    /// Ignored inside a `Field`, which assigns the id its label points at.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Extra attributes for the `<select>`.
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A native `<select>` in the look of the other form controls.
///
/// - It works without a client runtime: the browser opens its own picker and
///   submits the choice under `name`, so it suits server-rendered pages.
/// - `clearable` keeps an empty choice, and a `value` the options no longer
///   offer is shown as it is, not replaced by the first option.
/// - It suits plain text options. For icons, descriptions, groups or
///   filtering, use `Select` or `Combobox`.
/// - In an [`InputGroup`](crate::InputGroup) `prefix` or `suffix` it is drawn
///   as a chip: a protocol, a unit, a search scope.
#[function_component]
pub fn NativeSelect(props: &NativeSelectProps) -> Html {
    let select = use_node_ref();
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);

    // The `selected` attribute only sets the initial choice, so later values
    // from the caller are written to the element.
    {
        let select = select.clone();
        use_effect_with(props.value.clone(), move |value| set_value(&select, value));
    }

    let onchange = props.on_change.reform(|event: Event| crate::components::input::target_value(&event));
    // A value the options do not offer stays what is selected and submitted,
    // rather than giving way to whichever option comes first.
    let chosen = props.options.iter().any(|(value, _)| *value == props.value);
    let unknown = !chosen && !props.value.is_empty();
    let empty = props.placeholder.is_some() || props.clearable;

    let element = props.attrs.apply(html! {
        <select
            ref={select}
            id={control.id.clone()}
            class={classes!(
                crate::components::input::input_classes(props.size, false, control.invalid),
                "yi-native-select__control",
            )}
            name={props.name.clone()}
            disabled={control.disabled}
            required={control.required}
            aria-label={props.label.clone()}
            aria-invalid={control.aria_invalid()}
            aria-describedby={control.described_by.clone()}
            {onchange}
        >
            if empty {
                <option
                    value=""
                    disabled={!props.clearable}
                    hidden={!props.clearable}
                    selected={!chosen && !unknown}
                >
                    { props.placeholder.clone() }
                </option>
            }
            if unknown {
                <option value={props.value.clone()} selected=true>
                    { match &props.unknown_label {
                        Some(label) => label.emit(props.value.clone()),
                        None => props.value.clone(),
                    } }
                </option>
            }
            { for props.options.iter().map(|(value, label)| html! {
                <option key={value.to_string()} value={value.clone()} selected={*value == props.value}>
                    { label.clone() }
                </option>
            }) }
        </select>
    });

    html! {
        <span
            class={classes!(
                "yi-native-select",
                format!("yi-native-select--{}", props.size.as_str()),
                props.fit.then_some("yi-native-select--fit"),
                props.class.clone(),
            )}
        >
            { element }
            <Icon icon={icons::CHEVRON_DOWN} />
        </span>
    }
}

#[cfg(test)]
mod tests;
