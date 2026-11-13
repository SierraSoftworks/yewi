//! Composite text controls: [`InputGroup`] puts affixes, buttons and a
//! dropdown inside the outline of a text control, and [`SearchInput`] is the
//! ready-made search box built from it.
//!
//! Affixes are inset: plain text inside the field, with buttons and
//! dropdowns as small chips. The group draws the outline and the focus ring,
//! so the whole thing reads as one field.

use std::rc::Rc;

use yew::prelude::*;

use crate::is_empty;
use crate::{Attrs, FieldContext, Icon, Size, icons};

mod search_input;

pub use search_input::*;

/// Moves focus to the group's text control when the press lands on its
/// padding or on affix text, as it would in a plain input.
fn focus_control(group: &NodeRef, event: &MouseEvent) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;

        let on_control = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .and_then(|target| target.closest("input, textarea, select, button, a, label, [role], .yi-popover__surface").ok().flatten())
            .is_some();
        if on_control {
            return;
        }
        let control = group
            .cast::<web_sys::Element>()
            .and_then(|group| group.query_selector("input, textarea").ok().flatten())
            .and_then(|control| control.dyn_into::<web_sys::HtmlElement>().ok());
        if let Some(control) = control {
            // Keeps the press from starting a text selection or taking focus
            // back off the control.
            event.prevent_default();
            let _ = control.focus();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (group, event);
    }
}

#[derive(Properties, PartialEq)]
pub struct InputGroupProps {
    /// The text control: a `TextInput` or `NumberInput`. Its disabled,
    /// read-only and invalid states style the whole group.
    #[prop_or_default]
    pub children: Html,
    /// Shown before the control: text such as `https://`, an icon, or a
    /// `Button`, `MenuButton`, `Select` or `InputGroupSelect`, which are
    /// drawn as chips.
    #[prop_or_default]
    pub prefix: Html,
    /// Shown after the control: a unit such as `ms`, or a `Button`,
    /// `MenuButton`, `Select` or `InputGroupSelect`, which are drawn as
    /// chips.
    #[prop_or_default]
    pub suffix: Html,
    /// The size of the whole group. The control and any buttons inside
    /// follow it, whatever their own `size`.
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One outlined field holding a text control and its affixes.
///
/// Affix text is not part of the control's accessible name, so say the unit
/// or format in the `Field` label or hint as well.
#[function_component]
pub fn InputGroup(props: &InputGroupProps) -> Html {
    let group = use_node_ref();
    // The surrounding field labels and describes the text control only: a
    // Select or other control in an affix must not claim the same id.
    let field = use_context::<FieldContext>();
    let affix = |modifier: &'static str, content: &Html| {
        let content = match &field {
            Some(field) => {
                let context = FieldContext {
                    control_id: None,
                    described_by: None,
                    invalid: false,
                    required: false,
                    ..field.clone()
                };
                html! { <ContextProvider<FieldContext> {context}>{ content.clone() }</ContextProvider<FieldContext>> }
            }
            None => content.clone(),
        };
        html! { <span class={classes!("yi-input-group__affix", modifier)}>{ content }</span> }
    };
    let onmousedown = {
        let group = group.clone();
        Callback::from(move |event: MouseEvent| focus_control(&group, &event))
    };

    props.attrs.apply(html! {
        <div
            ref={group}
            id={props.id.clone()}
            class={classes!(
                "yi-input-group",
                format!("yi-input-group--{}", props.size.as_str()),
                props.class.clone(),
            )}
            {onmousedown}
        >
            if !is_empty(&props.prefix) {
                { affix("yi-input-group__affix--prefix", &props.prefix) }
            }
            { props.children.clone() }
            if !is_empty(&props.suffix) {
                { affix("yi-input-group__affix--suffix", &props.suffix) }
            }
        </div>
    })
}

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
pub struct InputGroupSelectProps {
    /// The choices, as `(value, label)` pairs.
    pub options: Rc<[(AttrValue, AttrValue)]>,
    /// The `value` of the selected option.
    #[prop_or_default]
    pub value: AttrValue,
    /// Fired with the chosen option's `value`.
    #[prop_or_default]
    pub on_change: Callback<String>,
    /// The accessible name: what is being chosen, e.g. "Protocol".
    pub label: AttrValue,
    /// The name the value is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Extra attributes for the `<select>`.
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A compact dropdown for an [`InputGroup`] `prefix` or `suffix`: a protocol,
/// a unit, a search scope. It is a native `<select>` drawn as a chip, so it
/// suits a handful of short options and opens the platform's own picker.
/// For icons, descriptions or groups, put a `Select` in the affix instead.
#[function_component]
pub fn InputGroupSelect(props: &InputGroupSelectProps) -> Html {
    let select = use_node_ref();

    // The `selected` attribute only sets the initial choice, so later values
    // from the caller are written to the element.
    {
        let select = select.clone();
        use_effect_with(props.value.clone(), move |value| set_value(&select, value));
    }

    let onchange = props.on_change.reform(|event: Event| crate::components::input::target_value(&event));

    let control = props.attrs.apply(html! {
        <select
            ref={select}
            id={props.id.clone()}
            class="yi-input-group__select-control"
            name={props.name.clone()}
            disabled={props.disabled}
            aria-label={props.label.clone()}
            {onchange}
        >
            { for props.options.iter().map(|(value, label)| html! {
                <option key={value.to_string()} value={value.clone()} selected={*value == props.value}>
                    { label.clone() }
                </option>
            }) }
        </select>
    });

    html! {
        <span class={classes!("yi-input-group__select", props.class.clone())}>
            { control }
            <Icon icon={icons::CHEVRON_DOWN} />
        </span>
    }
}

#[cfg(test)]
mod tests;
