//! [`RadioGroup`] and [`Radio`]: one choice out of a few, all visible at once.

use yew::prelude::*;

use crate::{
    Attrs, FieldContext, Orientation, Size,
    checkbox::{choice_classes, choice_text, target_checked},
    use_field_control, use_id,
};

/// What a [`RadioGroup`] tells the [`Radio`]s inside it.
#[derive(Clone, PartialEq)]
struct RadioGroupContext {
    name: AttrValue,
    value: Option<AttrValue>,
    on_change: Callback<AttrValue>,
    size: Size,
    disabled: bool,
    required: bool,
    invalid: bool,
}

#[derive(Properties, PartialEq)]
pub struct RadioGroupProps {
    /// The [`Radio`] options.
    #[prop_or_default]
    pub children: Html,
    /// The `value` of the selected option, or `None` while nothing is chosen.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Fired with an option's `value` when the user selects it.
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    /// The accessible name of the group. Not needed inside a `Field`, whose
    /// label names it.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or(Orientation::Vertical)]
    pub orientation: Orientation,
    #[prop_or_default]
    pub size: Size,
    /// The name the selected value is submitted under in a native form.
    /// Generated when not set, because it is also what ties the options
    /// together for the keyboard.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Disables every option.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    /// Marks the choice as having a problem. A surrounding `Field` with an
    /// `error` sets this for you.
    #[prop_or_default]
    pub invalid: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A set of [`Radio`] options of which exactly one can be selected. Tab moves
/// to the group and the arrow keys move the selection within it.
///
/// Put it in a `Field` with `group=true` to give it a label, hint and error.
#[function_component]
pub fn RadioGroup(props: &RadioGroupProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
    let field = use_context::<FieldContext>();
    let generated_name = use_id();

    let context = RadioGroupContext {
        name: props.name.clone().unwrap_or(generated_name),
        value: props.value.clone(),
        on_change: props.on_change.clone(),
        size: props.size,
        disabled: control.disabled,
        required: control.required,
        invalid: control.invalid,
    };
    let class = classes!(
        "yi-radio-group",
        format!("yi-radio-group--{}", props.orientation.as_str()),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div
            id={control.id.clone()}
            {class}
            role="radiogroup"
            aria-label={props.label.clone()}
            aria-labelledby={field.filter(|_| props.label.is_none()).map(|field| field.label_id)}
            aria-describedby={control.described_by.clone()}
            aria-invalid={control.aria_invalid()}
            aria-required={control.required.then_some("true")}
        >
            <ContextProvider<RadioGroupContext> {context}>
                { props.children.clone() }
            </ContextProvider<RadioGroupContext>>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct RadioProps {
    /// The label.
    #[prop_or_default]
    pub children: Html,
    /// What the group reports when this option is selected.
    pub value: AttrValue,
    /// A second, quieter line under the label, read out with it.
    #[prop_or_default]
    pub description: Html,
    /// The accessible name for a radio with no visible label.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Makes this one option unavailable.
    #[prop_or_default]
    pub disabled: bool,
    /// Whether the radio is selected. Only for a radio outside a
    /// `RadioGroup`; inside one, the group's `value` decides.
    #[prop_or_default]
    pub checked: bool,
    /// Fired with `value` when the user selects the radio. Only for a radio
    /// outside a `RadioGroup`.
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    /// Ties radios outside a `RadioGroup` together. Inside one, the group's
    /// name is used.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Only for a radio outside a `RadioGroup`.
    #[prop_or_default]
    pub size: Size,
    /// A reference to the `<input>`, for moving focus.
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    /// The id of the `<input>`.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One option of a [`RadioGroup`]. It can also stand alone, with `checked`,
/// `name` and `on_change` set by hand, where the options cannot share a
/// parent (a single-select column in a table).
#[function_component]
pub fn Radio(props: &RadioProps) -> Html {
    let group = use_context::<RadioGroupContext>();

    let (checked, name, on_change, size) = match &group {
        Some(group) => (
            group.value.as_ref() == Some(&props.value),
            Some(group.name.clone()),
            group.on_change.clone(),
            group.size,
        ),
        None => (props.checked, props.name.clone(), props.on_change.clone(), props.size),
    };
    let disabled = props.disabled || group.as_ref().is_some_and(|group| group.disabled);
    let required = group.as_ref().is_some_and(|group| group.required);
    let invalid = group.as_ref().is_some_and(|group| group.invalid);

    let onchange = {
        let value = props.value.clone();
        Callback::from(move |event: Event| {
            if target_checked(&event) {
                on_change.emit(value.clone());
            }
        })
    };
    let class = classes!(
        choice_classes(size, disabled),
        "yi-radio",
        invalid.then_some("yi-radio--invalid"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <label {class}>
            <span class="yi-choice__control">
                <input
                    ref={props.input_ref.clone()}
                    id={props.id.clone()}
                    class="yi-radio__input"
                    type="radio"
                    {name}
                    value={props.value.clone()}
                    {checked}
                    {disabled}
                    {required}
                    aria-label={props.label.clone()}
                    {onchange}
                />
            </span>
            { choice_text(&props.children, &props.description) }
        </label>
    })
}

#[cfg(test)]
mod tests;
