//! [`ToggleChip`] and [`ToggleChipGroup`]: a handful of independent filters
//! or options, each switched on and off in place.

use yew::prelude::*;

use crate::{Attrs, FieldContext, Icon, IconData, Size, icons};

#[derive(Properties, PartialEq)]
pub struct ToggleChipProps {
    /// The label: what the chip stands for, the same whether it is on or off.
    #[prop_or_default]
    pub children: Html,
    /// Whether the chip is on. Update it from `on_change` to keep the chip
    /// controlled.
    #[prop_or_default]
    pub pressed: bool,
    /// Fired with the new state when the user presses the chip.
    #[prop_or_default]
    pub on_change: Callback<bool>,
    /// An icon before the label, shown whether the chip is on or off. Without
    /// one, a chip that is on shows a check mark.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// `Small` and `Medium`; `Large` renders as `Medium`.
    #[prop_or(Size::Small)]
    pub size: Size,
    /// Unavailable: not focusable.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A small toggle button. Each chip is independent, so use a few of
/// them for filters or tags that combine; for one choice out of several, use
/// `SegmentedControl` or `RadioGroup`.
///
/// The state is announced as a pressed button, and shown by a check mark as
/// well as by colour.
#[function_component]
pub fn ToggleChip(props: &ToggleChipProps) -> Html {
    let onclick = {
        let (on_change, pressed) = (props.on_change.clone(), props.pressed);
        Callback::from(move |_: MouseEvent| on_change.emit(!pressed))
    };
    let class = classes!(
        "yi-toggle-chip",
        (props.size != Size::Small).then_some("yi-toggle-chip--md"),
        props.class.clone(),
    );
    let icon = props.icon.or(props.pressed.then_some(icons::CHECK));

    props.attrs.apply(html! {
        <button
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            {class}
            type="button"
            aria-pressed={if props.pressed { "true" } else { "false" }}
            disabled={props.disabled}
            {onclick}
        >
            if let Some(icon) = icon {
                <Icon {icon} />
            }
            <span class="yi-toggle-chip__label">{ props.children.clone() }</span>
        </button>
    })
}

#[derive(Properties, PartialEq)]
pub struct ToggleChipGroupProps {
    /// The [`ToggleChip`]s.
    #[prop_or_default]
    pub children: Html,
    /// The accessible name of the group. Not needed inside a `Field`, whose
    /// label names it.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A named row of [`ToggleChip`]s that wraps. Put it in a `Field` with
/// `group=true` to give it a label and hint.
#[function_component]
pub fn ToggleChipGroup(props: &ToggleChipGroupProps) -> Html {
    let field = use_context::<FieldContext>();

    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!("yi-toggle-chip-group", props.class.clone())}
            role={(props.label.is_some() || field.is_some()).then_some("group")}
            aria-label={props.label.clone()}
            aria-labelledby={field.filter(|_| props.label.is_none()).map(|field| field.label_id)}
        >
            { props.children.clone() }
        </div>
    })
}

#[cfg(test)]
mod tests;
