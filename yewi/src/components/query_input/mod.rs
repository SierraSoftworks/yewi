//! [`QueryInput`]: a single-line editor for search and filter queries, with
//! suggestions for the fields a query can use and the values they take.
//!
//! The component knows the shape of a query ([`QuerySyntax`]) well enough to
//! complete it, and nothing about what it means: parsing, validating and
//! running the query stay with the caller.

use std::rc::Rc;

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, InputGroup, Placement, Popover, PopoverTrigger, Size, icons, use_field_control, use_id};

mod complete;

pub use complete::*;

#[derive(Properties, PartialEq)]
pub struct QueryInputProps {
    /// The query shown. Update it from `on_input`.
    #[prop_or_default]
    pub value: AttrValue,
    /// Fired with the new query on every edit, including accepting a
    /// suggestion.
    #[prop_or_default]
    pub on_input: Callback<String>,
    /// Fired with the query when <kbd>Enter</kbd> is pressed with no
    /// suggestion highlighted: run it.
    #[prop_or_default]
    pub on_submit: Callback<String>,
    /// Fired when <kbd>Esc</kbd> is pressed with the suggestions closed.
    #[prop_or_default]
    pub on_cancel: Callback<()>,
    /// The fields a query can be scoped to, with the values to suggest for
    /// each. Without any, the input offers no suggestions.
    #[prop_or_else(|| Rc::from([]))]
    pub fields: Rc<[QueryField]>,
    /// The shape of the query language.
    #[prop_or_default]
    pub syntax: QuerySyntax,
    /// The query currently in effect, for inputs that apply on
    /// <kbd>Enter</kbd> rather than as you type. While `value` differs from
    /// it the input shows how to apply or revert, and <kbd>Esc</kbd> restores
    /// it through `on_input`.
    #[prop_or_default]
    pub applied: Option<AttrValue>,
    /// A problem with the query, shown under the input while it has focus.
    /// Marks the input invalid. Inside a `Field`, use the field's `error`
    /// instead.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// The icon before the text.
    #[prop_or(icons::SEARCH)]
    pub icon: IconData,
    /// Sets the query in the monospaced face, for expressions that read
    /// better as code.
    #[prop_or_default]
    pub mono: bool,
    /// The most suggestions offered at once.
    #[prop_or(8)]
    pub max_suggestions: usize,
    #[prop_or_default]
    pub size: Size,
    /// An example query. Not a substitute for a label.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// Names the input for assistive technology when no `Field` labels it.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// The name the query is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    /// Marks the query as having a problem without saying what.
    #[prop_or_default]
    pub invalid: bool,
    /// A reference to the `<input>`, for moving focus.
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    /// The id of the `<input>`. Ignored inside a `Field`, which assigns the
    /// id its label points at.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Extra attributes for the root element.
    #[prop_or_default]
    pub attrs: Attrs,
}

/// What the user has done since the text last changed.
#[derive(Clone, Copy, PartialEq, Default)]
struct Session {
    focused: bool,
    /// <kbd>Esc</kbd>, <kbd>Enter</kbd> or a press outside closed the list
    /// for this token.
    dismissed: bool,
    /// <kbd>Esc</kbd> or a press outside closed the error.
    error_dismissed: bool,
    highlight: Option<usize>,
    /// The caret, in UTF-16 units as the browser counts. `None` until the
    /// input has been used, which reads as the end of the text.
    caret: Option<u32>,
}

/// A single-line editor for search and filter queries.
///
/// - Suggests `fields` where the syntax allows one and a field's `values`
///   once a term is scoped to it, replacing the token at the caret.
/// - <kbd>↓</kbd> and <kbd>↑</kbd> move through the suggestions,
///   <kbd>Enter</kbd> or <kbd>Tab</kbd> accepts the highlighted one and
///   <kbd>Esc</kbd> closes the list. Nothing is highlighted until asked for,
///   so <kbd>Enter</kbd> always runs the query as typed.
/// - It is an ARIA combobox: the list is announced, and the highlighted
///   entry is the input's active descendant while focus stays in the text.
/// - Filtering is a linear scan that stops at `max_suggestions`, and the
///   list exists only while open.
#[function_component]
pub fn QueryInput(props: &QueryInputProps) -> Html {
    let base = use_id();
    let input_ref = props.input_ref.clone();
    let control = use_field_control(props.id.clone(), props.invalid, false, props.disabled);
    let session = use_state(Session::default);
    // Where to put the caret once an accepted suggestion has rendered.
    let pending_caret = use_mut_ref(|| None::<u32>);

    let completion = {
        let caret = session.caret.map_or(props.value.len(), |units| dom::byte_offset(&props.value, units));
        use_memo(
            (props.value.clone(), caret, props.fields.clone(), props.syntax, props.max_suggestions),
            |(value, caret, fields, syntax, max)| Rc::new(complete_query(value, *caret, fields, *syntax, *max)),
        )
    };
    let completion = Rc::clone(&completion);

    {
        let (input_ref, pending_caret) = (input_ref.clone(), pending_caret.clone());
        use_effect_with(props.value.clone(), move |_| {
            if let Some(caret) = pending_caret.borrow_mut().take() {
                dom::set_caret(&input_ref, caret);
            }
        });
    }

    let active = session.focused && !control.disabled;
    let error = props.error.clone().filter(|_| active && !session.error_dismissed);
    let listed = active && !session.dismissed && props.error.is_none() && !completion.items.is_empty();
    let highlight = session.highlight.filter(|&index| listed && index < completion.items.len());
    let dirty = props.applied.as_ref().is_some_and(|applied| *applied != props.value);

    let accept = {
        let (session, completion, pending_caret) = (session.clone(), completion.clone(), pending_caret.clone());
        let (value, on_input) = (props.value.clone(), props.on_input.clone());
        Callback::from(move |index: usize| {
            let Some((text, caret)) = completion.apply(&value, index) else { return };
            let caret = dom::utf16_offset(&text, caret);
            *pending_caret.borrow_mut() = Some(caret);
            session.set(Session { focused: true, caret: Some(caret), ..Session::default() });
            on_input.emit(text);
        })
    };

    let oninput = {
        let (session, on_input) = (session.clone(), props.on_input.clone());
        Callback::from(move |event: InputEvent| {
            session.set(Session { focused: true, caret: dom::caret(&event), ..Session::default() });
            on_input.emit(crate::components::input::target_value(&event));
        })
    };
    // The caret moved without the text changing: the token under it may not
    // be the same one.
    let moved = {
        let session = session.clone();
        move |event: &Event| {
            let caret = dom::caret(event);
            if session.caret != caret || !session.focused {
                session.set(Session { focused: true, caret, highlight: None, ..*session });
            }
        }
    };
    let onfocus = {
        let session = session.clone();
        Callback::from(move |event: FocusEvent| {
            session.set(Session { focused: true, caret: dom::caret(&event), ..Session::default() });
        })
    };
    let onblur = {
        let session = session.clone();
        Callback::from(move |_: FocusEvent| session.set(Session { focused: false, highlight: None, ..*session }))
    };
    let onkeyup = {
        let moved = moved.clone();
        Callback::from(move |event: KeyboardEvent| {
            if matches!(event.key().as_str(), "ArrowLeft" | "ArrowRight" | "Home" | "End") {
                moved(&event);
            }
        })
    };
    let onclick = Callback::from(move |event: MouseEvent| moved(&event));

    let onkeydown = {
        let (session, accept) = (session.clone(), accept.clone());
        let (value, applied) = (props.value.clone(), props.applied.clone());
        let (on_input, on_submit, on_cancel) =
            (props.on_input.clone(), props.on_submit.clone(), props.on_cancel.clone());
        let count = completion.items.len();
        Callback::from(move |event: KeyboardEvent| {
            let step = |forward: bool| match (highlight, forward) {
                (None, true) => 0,
                (None, false) => count - 1,
                (Some(index), true) => (index + 1) % count,
                (Some(index), false) => (index + count - 1) % count,
            };
            match event.key().as_str() {
                key @ ("ArrowDown" | "ArrowUp") if count > 0 && !event.alt_key() => {
                    event.prevent_default();
                    // The first press reopens a dismissed list.
                    let highlight = Some(if listed { step(key == "ArrowDown") } else { 0 });
                    session.set(Session { dismissed: false, highlight, ..*session });
                }
                "Enter" | "Tab" if highlight.is_some() => {
                    event.prevent_default();
                    accept.emit(highlight.unwrap_or_default());
                }
                "Enter" => {
                    session.set(Session { dismissed: true, error_dismissed: false, highlight: None, ..*session });
                    on_submit.emit(value.to_string());
                }
                // With the list open, the popover takes Escape to close it.
                "Escape" => {
                    if let Some(applied) = applied.as_ref().filter(|applied| **applied != value) {
                        on_input.emit(applied.to_string());
                    }
                    on_cancel.emit(());
                }
                _ => {}
            }
        })
    };

    let on_open_change = {
        let session = session.clone();
        Callback::from(move |open: bool| {
            if !open {
                session.set(Session { dismissed: true, error_dismissed: true, highlight: None, ..*session });
            }
        })
    };

    let list_id = AttrValue::from(format!("{base}-list"));
    let error_id = AttrValue::from(format!("{base}-error"));
    let option_id = |index: usize| format!("{base}-option-{index}");

    let content = if let Some(error) = &error {
        html! {
            <span id={error_id.clone()} class="yi-query-input__error" role="alert">
                <Icon icon={icons::CIRCLE_ALERT} />
                <span>{ error.clone() }</span>
            </span>
        }
    } else if listed {
        let options = completion.items.iter().enumerate().map(|(index, item)| {
            let selected = highlight == Some(index);
            // Keeps focus, and so the caret, in the input.
            let onmousedown = Callback::from(|event: MouseEvent| event.prevent_default());
            let onclick = accept.reform(move |_: MouseEvent| index);
            let onmouseenter = {
                let session = session.clone();
                Callback::from(move |_: MouseEvent| {
                    if session.highlight != Some(index) {
                        session.set(Session { highlight: Some(index), ..*session });
                    }
                })
            };
            html! {
                <span
                    key={item.insert.clone()}
                    id={option_id(index)}
                    class={classes!("yi-query-input__option", selected.then_some("yi-query-input__option--active"))}
                    role="option"
                    aria-selected={if selected { "true" } else { "false" }}
                    {onmousedown}
                    {onclick}
                    {onmouseenter}
                >
                    <span class="yi-query-input__token">{ item.label.clone() }</span>
                    if let Some(description) = &item.description {
                        <span class="yi-query-input__description">{ description.clone() }</span>
                    }
                </span>
            }
        });
        html! {
            <span id={list_id.clone()} class="yi-query-input__list" role="listbox" aria-label="Suggestions">
                { for options }
            </span>
        }
    } else {
        Html::default()
    };

    let invalid = control.invalid || props.error.is_some();
    let described_by = match (&control.described_by, error.is_some()) {
        (Some(ids), true) => Some(AttrValue::from(format!("{ids} {error_id}"))),
        (None, true) => Some(error_id),
        (ids, false) => ids.clone(),
    };

    let prefix = html! { <Icon icon={props.icon} class="yi-query-input__icon" /> };
    let suffix = (dirty && active).then(|| {
        html! {
            <span class="yi-query-input__hint">
                <kbd>{ "Enter" }</kbd>{ " to apply" }
                <span aria-hidden="true">{ " · " }</span>
                <kbd>{ "Esc" }</kbd>{ " to revert" }
            </span>
        }
    });

    html! {
        <Popover
            class={classes!("yi-query-input", props.class.clone())}
            surface_class="yi-query-input__surface"
            attrs={props.attrs.clone()}
            trigger={PopoverTrigger::Manual}
            placement={Placement::BottomStart}
            arrow=false
            flush=true
            role="presentation"
            open={listed || error.is_some()}
            {on_open_change}
            {content}
        >
            <InputGroup class="yi-query-input__box" size={props.size} {prefix} {suffix}>
                <input
                    ref={input_ref}
                    id={control.id.clone()}
                    class={classes!(
                        crate::components::input::input_classes(props.size, props.mono, invalid),
                        "yi-query-input__control",
                    )}
                    type="text"
                    role="combobox"
                    value={props.value.clone()}
                    name={props.name.clone()}
                    placeholder={props.placeholder.clone()}
                    autocomplete="off"
                    autocapitalize="off"
                    spellcheck="false"
                    enterkeyhint="search"
                    disabled={control.disabled}
                    aria-label={props.label.clone()}
                    aria-autocomplete="list"
                    aria-expanded={if listed { "true" } else { "false" }}
                    aria-controls={listed.then_some(list_id)}
                    aria-activedescendant={highlight.map(option_id)}
                    aria-invalid={invalid.then_some("true")}
                    aria-describedby={described_by}
                    {oninput}
                    {onkeydown}
                    {onkeyup}
                    {onclick}
                    {onfocus}
                    {onblur}
                />
            </InputGroup>
        </Popover>
    }
}

/// Caret positions are UTF-16 offsets in the browser and byte offsets in
/// Rust.
mod dom {
    use yew::NodeRef;

    pub fn byte_offset(text: &str, units: u32) -> usize {
        let mut seen = 0;
        for (at, c) in text.char_indices() {
            if seen >= units {
                return at;
            }
            seen += c.len_utf16() as u32;
        }
        text.len()
    }

    pub fn utf16_offset(text: &str, bytes: usize) -> u32 {
        text.char_indices().take_while(|(at, _)| *at < bytes).map(|(_, c)| c.len_utf16() as u32).sum()
    }

    /// Where the caret is in the input an event came from.
    #[cfg(target_arch = "wasm32")]
    pub fn caret(event: &web_sys::Event) -> Option<u32> {
        let target = event.target()?;
        js_sys::Reflect::get(&target, &"selectionStart".into()).ok()?.as_f64().map(|units| units as u32)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_caret(input: &NodeRef, units: u32) {
        use wasm_bindgen::JsCast;

        let Some(input) = input.get() else { return };
        let set = js_sys::Reflect::get(&input, &"setSelectionRange".into()).ok();
        if let Some(set) = set.and_then(|set| set.dyn_into::<js_sys::Function>().ok()) {
            let _ = set.call2(&input, &units.into(), &units.into());
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn caret<E>(_: &E) -> Option<u32> {
        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_caret(_: &NodeRef, _: u32) {}
}

#[cfg(test)]
mod tests;
