//! [`Switch`]: a setting that is on or off and applies as soon as it is flipped.

use yew::prelude::*;

use crate::{
    Attrs, Size,
    checkbox::{choice_classes, choice_text, described_by, target_checked},
    use_field_control, use_id,
};

#[derive(Properties, PartialEq)]
pub struct SwitchProps {
    /// The label: what the switch controls, not its state ("Email alerts",
    /// not "On"). Leave it out only inside a `Field`, or give `label`.
    #[prop_or_default]
    pub children: Html,
    /// Whether the switch is on. Update it from `on_change` to keep the
    /// switch controlled.
    #[prop_or_default]
    pub checked: bool,
    /// Fired with the new state when the user flips the switch.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// A second, quieter line under the label.
    #[prop_or_default]
    pub description: Option<Html>,
    /// The accessible name for a switch with no visible label.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub size: Size,
    /// The name the value is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// The value submitted when on. Browsers send `on` by default.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Unavailable: not focusable and not submitted.
    #[prop_or_default]
    pub disabled: bool,
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

/// An on/off control for a setting that takes effect immediately. Where the
/// choice is only applied on submit, use a [`Checkbox`](crate::Checkbox).
///
/// It is a native checkbox with `role="switch"`: Space flips it, and it
/// submits with a form.
#[function_component]
pub fn Switch(props: &SwitchProps) -> Html {
    let control = use_field_control(props.id.clone(), false, false, props.disabled);
    let description_id = use_id();

    let onchange = props.on_change.reform(|event: Event| target_checked(&event));
    let class = classes!(choice_classes(props.size, control.disabled), "yi-switch", props.class.clone());

    props.attrs.apply(html! {
        <label {class}>
            <span class="yi-choice__control">
                <input
                    ref={props.input_ref.clone()}
                    id={control.id.clone()}
                    class="yi-switch__input"
                    type="checkbox"
                    role="switch"
                    checked={props.checked}
                    name={props.name.clone()}
                    value={props.value.clone()}
                    disabled={control.disabled}
                    aria-label={props.label.clone()}
                    aria-describedby={described_by([
                        props.description.as_ref().map(|_| &description_id),
                        control.described_by.as_ref(),
                    ])}
                    {onchange}
                />
            </span>
            { choice_text(&props.children, props.description.as_ref(), &description_id) }
        </label>
    })
}

#[cfg(test)]
mod tests;
