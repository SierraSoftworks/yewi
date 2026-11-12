//! [`Checkbox`]: one independent yes-or-no choice, or one of several.
//!
//! This directory also holds the label-and-description row (`.yi-choice`)
//! that [`Radio`](crate::Radio) and [`Switch`](crate::Switch) share.

use yew::prelude::*;

use crate::{Attrs, Icon, Size, icons, use_field_control, use_id};

/// Whether the checkbox or radio an event came from is now checked.
///
/// Read through reflection so the crate needs no extra `web-sys` features.
pub(crate) fn target_checked(event: &Event) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        event
            .target()
            .and_then(|target| js_sys::Reflect::get(&target, &"checked".into()).ok())
            .and_then(|checked| checked.as_bool())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = event;
        false
    }
}

/// Joins the ids a control is described by, skipping the ones not present.
pub(crate) fn described_by(ids: [Option<&AttrValue>; 2]) -> Option<AttrValue> {
    let ids: Vec<&str> = ids.into_iter().flatten().map(AttrValue::as_str).collect();
    (!ids.is_empty()).then(|| ids.join(" ").into())
}

/// The classes of the row shared by checkboxes, radios and switches.
pub(crate) fn choice_classes(size: Size, disabled: bool) -> Classes {
    classes!("yi-choice", format!("yi-choice--{}", size.as_str()), disabled.then_some("yi-choice--disabled"))
}

/// The label and description beside a checkbox, radio or switch. Renders
/// nothing when there is neither, so a bare control stays just the control.
pub(crate) fn choice_text(label: &Html, description: Option<&Html>, description_id: &AttrValue) -> Html {
    let has_label = *label != Html::default();
    if !has_label && description.is_none() {
        return Html::default();
    }
    html! {
        <span class="yi-choice__text">
            if has_label {
                <span class="yi-choice__label">{ label.clone() }</span>
            }
            if let Some(description) = description {
                <span id={description_id.clone()} class="yi-choice__description">{ description.clone() }</span>
            }
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    /// The label. Leave it out only where the meaning is clear from context
    /// (a table's selection column) and give `label` instead.
    #[prop_or_default]
    pub children: Html,
    /// Whether the box is ticked. Update it from `on_change` to keep the
    /// checkbox controlled.
    #[prop_or_default]
    pub checked: bool,
    /// Shows a dash instead of a tick: some, but not all, of the things this
    /// checkbox stands for are selected. Clicking it reports `true`.
    #[prop_or_default]
    pub indeterminate: bool,
    /// Fired with the new state when the user toggles the box.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// A second, quieter line under the label.
    #[prop_or_default]
    pub description: Option<Html>,
    /// The accessible name for a checkbox with no visible label.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub size: Size,
    /// The name the value is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// The value submitted when ticked. Browsers send `on` by default.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Unavailable: not focusable and not submitted.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    /// Marks the choice as having a problem. A surrounding `Field` with an
    /// `error` sets this for you.
    #[prop_or_default]
    pub invalid: bool,
    /// A reference to the `<input>`, for moving focus.
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    /// The id of the `<input>`. Ignored inside a single-control `Field`,
    /// which assigns the id its label points at.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A box the user ticks. Each checkbox is independent of its neighbours; for
/// one choice out of several use [`RadioGroup`](crate::RadioGroup), and for a
/// setting that applies immediately use [`Switch`](crate::Switch).
#[function_component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
    let description_id = use_id();
    let input_ref = props.input_ref.clone();

    // `indeterminate` exists only as a DOM property, so it is set after render.
    // The dash itself is drawn from a class and needs no script.
    {
        let input_ref = input_ref.clone();
        use_effect_with(props.indeterminate, move |indeterminate| {
            #[cfg(target_arch = "wasm32")]
            if let Some(input) = input_ref.get() {
                let _ = js_sys::Reflect::set(&input, &"indeterminate".into(), &(*indeterminate).into());
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = (input_ref, indeterminate);
        });
    }

    let onchange = props.on_change.reform(|event: Event| target_checked(&event));
    let mixed = props.indeterminate && !props.checked;
    let class = classes!(
        choice_classes(props.size, control.disabled),
        "yi-checkbox",
        mixed.then_some("yi-checkbox--indeterminate"),
        control.invalid.then_some("yi-checkbox--invalid"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <label {class}>
            <span class="yi-choice__control">
                <input
                    ref={input_ref}
                    id={control.id.clone()}
                    class="yi-checkbox__input"
                    type="checkbox"
                    checked={props.checked}
                    name={props.name.clone()}
                    value={props.value.clone()}
                    disabled={control.disabled}
                    required={control.required}
                    aria-label={props.label.clone()}
                    aria-invalid={control.aria_invalid()}
                    aria-describedby={described_by([
                        props.description.as_ref().map(|_| &description_id),
                        control.described_by.as_ref(),
                    ])}
                    {onchange}
                />
                <Icon icon={if mixed { icons::MINUS } else { icons::CHECK }} class="yi-checkbox__mark" />
            </span>
            { choice_text(&props.children, props.description.as_ref(), &description_id) }
        </label>
    })
}

#[cfg(test)]
mod tests;
