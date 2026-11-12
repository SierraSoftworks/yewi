//! Choosing one of a list: [`Select`] for a short list read at a glance, and
//! [`Combobox`] for a long one narrowed by typing.
//!
//! Both are controlled: they show `value` and report a choice through
//! `on_change`. Inside a [`Field`](crate::Field) they pick up its label, id,
//! description, and its required, disabled and invalid state.

use yew::prelude::*;

use crate::{
    Attrs, FieldContext, Icon, Placement, Popover, PopoverTrigger, Size, components::input::input_classes, icons,
    use_field_control, use_id,
};

mod combobox;
mod listbox;
mod options;

pub use combobox::*;
use listbox::{Listbox, option_id};
pub use options::{SelectOption, SelectOptions};
use options::{edge, step, typeahead};

/// How many options <kbd>Page Up</kbd> and <kbd>Page Down</kbd> move by.
const PAGE: isize = 10;
/// Milliseconds after which typed letters start a new search.
const TYPEAHEAD_RESET: f64 = 600.0;

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    /// The choices, in display order.
    pub options: SelectOptions,
    /// The `value` of the chosen option. `None`, or a value no option has,
    /// shows the placeholder.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Fired with an option's value when the user chooses a different one.
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    /// Shown while nothing is chosen.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub size: Size,
    /// Sizes the control to its content instead of filling its container, for
    /// toolbars and filter rows.
    #[prop_or_default]
    pub fit: bool,
    /// Names the control for assistive technology when no `Field` or
    /// `<label>` does.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Submits the chosen value under this name in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Unavailable: not focusable and not submitted.
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
    /// The id of the control. Ignored inside a `Field`, which assigns the id
    /// its label points at.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A button showing the current choice, which opens a list with the chosen
/// option check-marked.
///
/// - Keyboard: <kbd>↓</kbd> <kbd>↑</kbd> <kbd>Home</kbd> <kbd>End</kbd> move,
///   <kbd>Enter</kbd> or <kbd>Space</kbd> chooses, <kbd>Esc</kbd> closes, and
///   typing jumps to the option starting with those letters.
/// - The list exists only while open and floats above everything, so it is
///   never clipped by a scrolling container or a dialog.
/// - Every option is rendered when the list opens. Past a few hundred, use a
///   [`Combobox`], which renders only the best matches.
#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
    let field = use_context::<FieldContext>();
    let base = use_id();
    let trigger = use_node_ref();
    let open = use_state(|| false);
    let active = use_state(|| None::<usize>);
    let width = use_state(|| 0.0_f64);
    // The letters typed so far, and when the last one arrived.
    let typed = use_mut_ref(|| (String::new(), 0.0_f64));

    let options = &props.options;
    let selected = props.value.as_ref().and_then(|value| options.iter().position(|option| option.value == *value));
    let is_open = *open && !control.disabled;
    let list_id = AttrValue::from(format!("{base}-list"));

    let show = {
        let (open, active, width, trigger) = (open.clone(), active.clone(), width.clone(), trigger.clone());
        move |start: Option<usize>| {
            width.set(dom::width_of(&trigger));
            active.set(start);
            open.set(true);
        }
    };
    let pick = {
        let (options, open, on_change) = (options.clone(), open.clone(), props.on_change.clone());
        Callback::from(move |index: usize| {
            let Some(option) = options.get(index).filter(|option| !option.disabled) else {
                return;
            };
            open.set(false);
            if selected != Some(index) {
                on_change.emit(option.value.clone());
            }
        })
    };
    let hover = {
        let (options, active) = (options.clone(), active.clone());
        Callback::from(move |index: usize| {
            if *active != Some(index) && options.get(index).is_some_and(|option| !option.disabled) {
                active.set(Some(index));
            }
        })
    };

    let onclick = {
        let (open, show, options) = (open.clone(), show.clone(), options.clone());
        Callback::from(move |_: MouseEvent| {
            if *open {
                open.set(false);
            } else {
                let order: Vec<usize> = (0..options.len()).collect();
                show(selected.or_else(|| edge(&options, &order, false)));
            }
        })
    };
    let onkeydown = {
        let (open, active, show, pick, options, typed) =
            (open.clone(), active.clone(), show.clone(), pick.clone(), options.clone(), typed.clone());
        Callback::from(move |event: KeyboardEvent| {
            if event.ctrl_key() || event.meta_key() || event.alt_key() {
                return;
            }
            let key = event.key();
            let order: Vec<usize> = (0..options.len()).collect();
            let first = || edge(&options, &order, false);
            let current = if *open { *active } else { selected };

            // A letter continues (or starts) a search by prefix.
            let mut letters = key.chars();
            let letter = letters.next().filter(|_| letters.next().is_none());
            let searching = {
                let (text, at) = &*typed.borrow();
                !text.is_empty() && event.time_stamp() - at < TYPEAHEAD_RESET
            };
            if let Some(letter) = letter.filter(|letter| *letter != ' ' || searching) {
                event.prevent_default();
                let mut typed = typed.borrow_mut();
                if !searching {
                    typed.0.clear();
                }
                typed.0.push(letter);
                typed.1 = event.time_stamp();
                let found = typeahead(&options, &typed.0, current);
                if *open {
                    if found.is_some() {
                        active.set(found);
                    }
                } else {
                    show(found.or(current).or_else(first));
                }
                return;
            }

            let next = match (key.as_str(), *open) {
                ("ArrowDown" | "ArrowUp" | "Enter" | " ", false) => current.or_else(first),
                ("Home", _) => first(),
                ("End", _) => edge(&options, &order, true),
                ("ArrowDown", true) => step(&options, &order, current, 1),
                ("ArrowUp", true) => step(&options, &order, current, -1),
                ("PageDown", true) => step(&options, &order, current, PAGE),
                ("PageUp", true) => step(&options, &order, current, -PAGE),
                ("Enter" | " ", true) => {
                    event.prevent_default();
                    match current {
                        Some(index) => pick.emit(index),
                        None => open.set(false),
                    }
                    return;
                }
                // Focus moves on; the list does not follow it.
                ("Tab", true) => {
                    open.set(false);
                    return;
                }
                _ => return,
            };
            event.prevent_default();
            if *open { active.set(next) } else { show(next) }
        })
    };
    // The browser clicks a button when Space is released; that press was
    // already handled.
    let onkeyup = Callback::from(|event: KeyboardEvent| {
        if event.key() == " " {
            event.prevent_default();
        }
    });
    let onblur = {
        let open = open.clone();
        Callback::from(move |event: FocusEvent| {
            // Focus that went nowhere (another window) is not a reason to close.
            if event.related_target().is_some() {
                open.set(false);
            }
        })
    };
    let on_open_change = {
        let open = open.clone();
        Callback::from(move |next: bool| {
            if !next {
                open.set(false);
            }
        })
    };

    {
        let base = base.clone();
        use_effect_with((is_open, *active), move |(open, active)| {
            if let (true, Some(index)) = (*open, *active) {
                dom::scroll_into_view(&option_id(&base, index));
            }
        });
    }

    let content = if is_open {
        let order: Vec<usize> = (0..options.len()).collect();
        Listbox {
            id: list_id.clone(),
            base: &base,
            options,
            order: &order,
            selected,
            active: *active,
            width: *width,
            label: props.label.clone(),
            labelled_by: field.as_ref().map(|field| field.label_id.clone()),
            on_pick: pick,
            on_hover: hover,
        }
        .render()
    } else {
        Html::default()
    };

    let chosen = selected.map(|index| &options[index]);
    let class = classes!(
        input_classes(props.size, false, control.invalid),
        "yi-select__control",
        "yi-select__trigger",
    );
    let root = classes!("yi-select", props.fit.then_some("yi-select--fit"), props.class.clone());

    props.attrs.apply(html! {
        <div class={root}>
            <Popover
                trigger={PopoverTrigger::Manual}
                placement={Placement::BottomStart}
                arrow=false
                flush=true
                role="presentation"
                open={is_open}
                {on_open_change}
                {content}
                surface_class="yi-select__surface"
            >
                <button
                    ref={trigger}
                    type="button"
                    id={control.id.clone()}
                    {class}
                    role="combobox"
                    aria-haspopup="listbox"
                    aria-expanded={if is_open { "true" } else { "false" }}
                    aria-controls={is_open.then(|| list_id.clone())}
                    aria-activedescendant={active.filter(|_| is_open).map(|index| option_id(&base, index))}
                    aria-label={props.label.clone()}
                    aria-invalid={control.aria_invalid()}
                    aria-required={control.required.then_some("true")}
                    aria-describedby={control.described_by.clone()}
                    disabled={control.disabled}
                    {onclick}
                    {onkeydown}
                    {onkeyup}
                    {onblur}
                >
                    if let Some(option) = chosen {
                        <span class="yi-select__value">
                            if let Some(icon) = option.icon {
                                <Icon {icon} class="yi-select__value-icon" />
                            }
                            { option.label.clone() }
                        </span>
                    } else {
                        <span class="yi-select__value yi-select__value--placeholder">
                            { props.placeholder.clone().unwrap_or_default() }
                        </span>
                    }
                    <Icon icon={icons::CHEVRON_DOWN} class="yi-select__chevron" />
                </button>
            </Popover>
            if let Some(name) = &props.name {
                <input type="hidden" name={name.clone()} value={props.value.clone()} disabled={control.disabled} />
            }
        </div>
    })
}

#[cfg(target_arch = "wasm32")]
pub(crate) mod dom {
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::{Element, HtmlElement, MouseEvent, Node};
    use yew::NodeRef;

    /// The option a pointer event happened on, from its `data-yi-option`.
    pub fn option_index(event: &MouseEvent) -> Option<usize> {
        let target = event.target()?.dyn_into::<Element>().ok()?;
        let option = target.closest("[data-yi-option]").ok()??;
        option.get_attribute("data-yi-option")?.parse().ok()
    }

    pub fn width_of(element: &NodeRef) -> f64 {
        element.cast::<Element>().map_or(0.0, |element| element.get_bounding_client_rect().width())
    }

    /// Scrolls the list just far enough to show the element with this id.
    pub fn scroll_into_view(id: &str) {
        let element = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(id));
        let Some(element) = element else { return };
        let options = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&options, &"block".into(), &"nearest".into());
        if let Ok(scroll) = js_sys::Reflect::get(&element, &"scrollIntoView".into())
            && let Some(scroll) = scroll.dyn_ref::<js_sys::Function>()
        {
            let _ = scroll.call1(&element, &JsValue::from(options));
        }
    }

    pub fn focus(element: &NodeRef) {
        if let Some(element) = element.cast::<HtmlElement>() {
            let _ = element.focus();
        }
    }

    /// Whether the event happened on `element` itself, not around it.
    pub fn is_target(event: &MouseEvent, element: &NodeRef) -> bool {
        let target = event.target().and_then(|t| t.dyn_into::<Node>().ok());
        element.get().zip(target).is_some_and(|(element, target)| element == target)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod dom {
    //! Events and effects never run outside a browser.

    use yew::NodeRef;

    pub fn option_index<E>(_: &E) -> Option<usize> {
        None
    }

    pub fn width_of(_: &NodeRef) -> f64 {
        0.0
    }

    pub fn scroll_into_view(_: &str) {}

    pub fn focus(_: &NodeRef) {}

    pub fn is_target<E>(_: &E, _: &NodeRef) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
