//! [`Field`]: the label, hint and error around one form control, and the
//! context ([`use_field_control`]) every control reads to wire itself up.

use yew::prelude::*;

use crate::is_empty;
use crate::{Attrs, Icon, icons, use_id};

/// Where a field's label sits relative to its control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum FieldLayout {
    /// Label above the control. The default, and the only layout that suits
    /// narrow containers.
    #[default]
    Stacked,
    /// Label in a fixed column before the control, for dense settings forms.
    /// Falls back to stacked when the field is too narrow for both.
    Inline,
}

impl FieldLayout {
    const fn as_str(self) -> &'static str {
        match self {
            FieldLayout::Stacked => "stacked",
            FieldLayout::Inline => "inline",
        }
    }
}

/// What a [`Field`] tells the controls inside it. Read it with
/// [`use_field_control`] rather than directly.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldContext {
    /// The id the label points at. `None` for a group field, whose label
    /// names the whole group instead of one control.
    pub control_id: Option<AttrValue>,
    /// The id of the label element, for `aria-labelledby`.
    pub label_id: AttrValue,
    /// The id of the hint or error currently shown, for `aria-describedby`.
    pub described_by: Option<AttrValue>,
    pub invalid: bool,
    pub required: bool,
    pub disabled: bool,
}

/// How a control should render, after combining its own props with the
/// [`Field`] around it (if any).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FieldControl {
    /// The id for the focusable element.
    pub id: Option<AttrValue>,
    /// The value for `aria-describedby`.
    pub described_by: Option<AttrValue>,
    pub invalid: bool,
    pub required: bool,
    pub disabled: bool,
}

impl FieldControl {
    /// The value for `aria-invalid`: present only when invalid.
    pub fn aria_invalid(&self) -> Option<&'static str> {
        self.invalid.then_some("true")
    }
}

/// Resolves a control's `id`, `invalid`, `required` and `disabled` props
/// against the surrounding [`Field`]. Every form control calls this, so that
/// it works the same inside a field and on its own.
///
/// Inside a field, the field's `control_id` wins over the control's own `id`
/// (the label already points at it) and the flags are OR-ed together.
///
/// ```ignore
/// let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
/// html! {
///     <input
///         id={control.id.clone()}
///         aria-describedby={control.described_by.clone()}
///         aria-invalid={control.aria_invalid()}
///         required={control.required}
///         disabled={control.disabled}
///     />
/// }
/// ```
#[hook]
pub fn use_field_control(id: Option<AttrValue>, invalid: bool, required: bool, disabled: bool) -> FieldControl {
    match use_context::<FieldContext>() {
        Some(field) => FieldControl {
            id: field.control_id.or(id),
            described_by: field.described_by,
            invalid: invalid || field.invalid,
            required: required || field.required,
            disabled: disabled || field.disabled,
        },
        None => FieldControl { id, described_by: None, invalid, required, disabled },
    }
}

#[derive(Properties, PartialEq)]
pub struct FieldProps {
    /// The control: a `TextInput`, `Select`, or any element using
    /// [`use_field_control`].
    #[prop_or_default]
    pub children: Html,
    /// The visible name of the control.
    pub label: Html,
    /// Guidance shown under the control. Replaced by `error` while there is
    /// one, so the field does not grow as the user types.
    #[prop_or_default]
    pub hint: Html,
    /// A problem with the current value. Marks the control invalid and is
    /// announced to assistive technology.
    #[prop_or_default]
    pub error: Html,
    /// Marks the label and the control as required.
    #[prop_or_default]
    pub required: bool,
    /// Disables the control and dims the label.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub layout: FieldLayout,
    /// Keeps the label for assistive technology but hides it visually, for
    /// controls whose purpose is obvious from context (a table filter).
    #[prop_or_default]
    pub hide_label: bool,
    /// The label names a group of controls (radio buttons, a set of
    /// checkboxes) rather than one. Renders `role="group"` and gives the
    /// controls no shared id.
    #[prop_or_default]
    pub group: bool,
    /// The id given to the control. Generated when not set.
    #[prop_or_default]
    pub control_id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A labelled row around one control. It owns the label, the required
/// marker, the hint and the error, and connects them to the control for
/// assistive technology.
///
/// A field has no margin of its own. Put the fields of a form in a `Stack`
/// (or a `Grid`), which sets the space between them.
#[function_component]
pub fn Field(props: &FieldProps) -> Html {
    let base = use_id();
    let label_id = AttrValue::from(format!("{base}-label"));
    let control_id =
        (!props.group).then(|| props.control_id.clone().unwrap_or_else(|| format!("{base}-control").into()));
    let invalid = !is_empty(&props.error);
    let message_id = (invalid || !is_empty(&props.hint)).then(|| AttrValue::from(format!("{base}-message")));

    let context = FieldContext {
        control_id: control_id.clone(),
        label_id: label_id.clone(),
        described_by: message_id.clone(),
        invalid,
        required: props.required,
        disabled: props.disabled,
    };

    let class = classes!(
        "yi-field",
        format!("yi-field--{}", props.layout.as_str()),
        invalid.then_some("yi-field--invalid"),
        props.disabled.then_some("yi-field--disabled"),
        props.class.clone(),
    );
    let label_class = classes!("yi-field__label", props.hide_label.then_some("yi-visually-hidden"));
    let label = html! {
        <>
            { props.label.clone() }
            if props.required {
                <span class="yi-field__required" aria-hidden="true">{ "*" }</span>
            }
        </>
    };

    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            {class}
            role={props.group.then_some("group")}
            aria-labelledby={props.group.then(|| label_id.clone())}
            aria-describedby={props.group.then(|| message_id.clone()).flatten()}
        >
            if props.group {
                <span id={label_id} class={label_class}>{ label }</span>
            } else {
                <label id={label_id} class={label_class} for={control_id}>{ label }</label>
            }
            <div class="yi-field__body">
                <ContextProvider<FieldContext> {context}>
                    { props.children.clone() }
                </ContextProvider<FieldContext>>
                if !is_empty(&props.error) {
                    <p id={message_id} class="yi-field__message yi-field__message--error" role="alert">
                        <Icon icon={icons::CIRCLE_ALERT} />
                        <span>{ props.error.clone() }</span>
                    </p>
                } else if !is_empty(&props.hint) {
                    <p id={message_id} class="yi-field__message">{ props.hint.clone() }</p>
                }
            </div>
        </div>
    })
}

#[cfg(test)]
mod tests;
