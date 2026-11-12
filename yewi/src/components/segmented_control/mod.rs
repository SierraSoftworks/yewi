//! [`SegmentedControl`] and [`Segment`]: two to five views of the same thing,
//! switched in place.

use yew::prelude::*;

use crate::{Attrs, FieldContext, Icon, IconData, Size, checkbox::target_checked, use_field_control, use_id};

/// What a [`SegmentedControl`] tells the [`Segment`]s inside it.
#[derive(Clone, PartialEq)]
struct SegmentedContext {
    name: AttrValue,
    value: AttrValue,
    on_change: Callback<AttrValue>,
    disabled: bool,
}

#[derive(Properties, PartialEq)]
pub struct SegmentedControlProps {
    /// The [`Segment`] options.
    #[prop_or_default]
    pub children: Html,
    /// The `value` of the selected segment. One segment is always selected.
    pub value: AttrValue,
    /// Fired with a segment's `value` when the user selects it.
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    /// What the control switches, as its accessible name. Not needed inside
    /// a `Field`, whose label names it.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub size: Size,
    /// Fills the available width, with every segment the same width.
    #[prop_or_default]
    pub block: bool,
    /// The name the selected value is submitted under in a native form.
    /// Generated when not set, because it is also what ties the segments
    /// together for the keyboard.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Disables every segment.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A compact row of mutually exclusive options, for switching between a few
/// views or modes (Changed / All, Day / Week / Month). It behaves as a radio
/// group: Tab moves to it and the arrow keys move the selection.
///
/// For choices in a form prefer [`RadioGroup`](crate::RadioGroup), which has
/// room for longer labels and descriptions; for moving between pages use
/// tabs.
#[function_component]
pub fn SegmentedControl(props: &SegmentedControlProps) -> Html {
    let control = use_field_control(props.id.clone(), false, false, props.disabled);
    let field = use_context::<FieldContext>();
    let generated_name = use_id();

    let context = SegmentedContext {
        name: props.name.clone().unwrap_or(generated_name),
        value: props.value.clone(),
        on_change: props.on_change.clone(),
        disabled: control.disabled,
    };
    let class = classes!(
        "yi-segmented",
        format!("yi-segmented--{}", props.size.as_str()),
        props.block.then_some("yi-segmented--block"),
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
        >
            <ContextProvider<SegmentedContext> {context}>
                { props.children.clone() }
            </ContextProvider<SegmentedContext>>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct SegmentProps {
    /// The label: a word or two.
    #[prop_or_default]
    pub children: Html,
    /// What the control reports when this segment is selected.
    pub value: AttrValue,
    /// An icon before the label, or on its own with `label` set.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// The accessible name for a segment that shows only an icon.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Makes this one segment unavailable.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    /// The id of the `<input>`.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One option of a [`SegmentedControl`]. Renders nothing useful outside one.
#[function_component]
pub fn Segment(props: &SegmentProps) -> Html {
    let group = use_context::<SegmentedContext>();
    let checked = group.as_ref().is_some_and(|group| group.value == props.value);
    let disabled = props.disabled || group.as_ref().is_some_and(|group| group.disabled);

    let onchange = {
        let value = props.value.clone();
        let on_change = group.as_ref().map(|group| group.on_change.clone()).unwrap_or_default();
        Callback::from(move |event: Event| {
            if target_checked(&event) {
                on_change.emit(value.clone());
            }
        })
    };

    props.attrs.apply(html! {
        <label class={classes!("yi-segmented__option", props.class.clone())}>
            <input
                id={props.id.clone()}
                class="yi-segmented__input"
                type="radio"
                name={group.map(|group| group.name)}
                value={props.value.clone()}
                {checked}
                {disabled}
                aria-label={props.label.clone()}
                {onchange}
            />
            <span class="yi-segmented__label">
                if let Some(icon) = props.icon {
                    <Icon {icon} />
                }
                { props.children.clone() }
            </span>
        </label>
    })
}

#[cfg(test)]
mod tests;
