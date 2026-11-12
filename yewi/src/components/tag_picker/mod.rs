//! Choosing several of a list: [`TagPicker`] shows each choice as a tag
//! inside the field and narrows the remaining options as the user types.
//!
//! It is controlled: it shows `values` and reports the whole new list through
//! `on_change`. Inside a [`Field`](crate::Field) it picks up its label, id,
//! description, and its required, disabled and invalid state.

use yew::prelude::*;

use crate::{
    Attrs, FieldContext, Icon, Placement, Popover, PopoverTrigger, SelectOptions,
    components::{
        input::{input_classes, target_value},
        select::{
            dom as select_dom,
            listbox::{Listbox, option_id},
            options::{edge, filter, step},
        },
    },
    icons, use_field_control, use_id,
};

mod values;

use values::{add, parse, toggle};

/// How many options <kbd>Page Up</kbd> and <kbd>Page Down</kbd> move by.
const PAGE: isize = 10;

#[derive(Properties, PartialEq)]
pub struct TagPickerProps {
    /// The values chosen, in the order they were added.
    #[prop_or_default]
    pub values: Vec<AttrValue>,
    /// Fired with the whole new list whenever a value is added or removed.
    #[prop_or_default]
    pub on_change: Callback<Vec<AttrValue>>,
    /// The choices on offer. Leave empty, with `allow_custom`, for a field of
    /// free-form tags.
    #[prop_or_default]
    pub options: SelectOptions,
    /// Accepts text that matches no option: <kbd>Enter</kbd>, a comma, or
    /// leaving the field adds it as a tag. Pasted text is split on commas.
    #[prop_or_default]
    pub allow_custom: bool,
    /// The most options rendered at once. The rest are counted under the
    /// list, so a list of thousands stays cheap to open and to type in.
    #[prop_or(50)]
    pub max_results: usize,
    /// Shown in place of the list when nothing matches.
    #[prop_or(AttrValue::Static("No matches"))]
    pub empty: AttrValue,
    /// Shown while nothing is chosen.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// Sets the tags and the text in the monospace face, for identifiers.
    #[prop_or_default]
    pub mono: bool,
    /// The verb in each tag's remove button, which is named this followed by
    /// the tag's text ("Remove production").
    #[prop_or(AttrValue::Static("Remove"))]
    pub remove_label: AttrValue,
    /// Names the control for assistive technology when no `Field` or
    /// `<label>` does.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Submits each chosen value under this name in a native form.
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
    /// A reference to the `<input>`, for moving focus.
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    /// The id of the input. Ignored inside a `Field`, which assigns the id
    /// its label points at.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A field holding several values as tags, with a list of options that
/// narrows as the user types.
///
/// - Keyboard: typing filters, <kbd>↓</kbd> <kbd>↑</kbd> move,
///   <kbd>Enter</kbd> adds or removes the highlighted option,
///   <kbd>Backspace</kbd> in an empty field removes the last tag, and
///   <kbd>Esc</kbd> closes the list. Each tag's remove button is a tab stop.
/// - The list stays open after a choice, with chosen options check-marked,
///   so several can be picked in a row.
/// - At most `max_results` options are rendered however long the list is.
#[function_component]
pub fn TagPicker(props: &TagPickerProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
    let field = use_context::<FieldContext>();
    let base = use_id();
    let anchor = use_node_ref();
    let input = props.input_ref.clone();
    let open = use_state(|| false);
    let active = use_state(|| None::<usize>);
    let width = use_state(|| 0.0_f64);
    let text = use_state(String::new);

    let options = &props.options;
    let values = &props.values;
    let allow_custom = props.allow_custom;
    let limit = props.max_results.max(1);
    let list_id = AttrValue::from(format!("{base}-list"));
    let selected: Vec<usize> =
        options.iter().enumerate().filter(|(_, option)| values.contains(&option.value)).map(|(index, _)| index).collect();

    // A field of free-form tags has no list to show until something is typed.
    let has_choices = !options.is_empty();
    let is_open = *open && !control.disabled && (has_choices || !text.trim().is_empty());
    let (order, total) = if is_open && has_choices { filter(options, &text, limit) } else { (Vec::new(), 0) };

    let show = {
        let (open, active, width, anchor) = (open.clone(), active.clone(), width.clone(), anchor.clone());
        move |start: Option<usize>| {
            width.set(select_dom::width_of(&anchor));
            active.set(start);
            open.set(true);
        }
    };
    let pick = {
        let (options, values, text, on_change) =
            (options.clone(), values.clone(), text.clone(), props.on_change.clone());
        Callback::from(move |index: usize| {
            let Some(option) = options.get(index).filter(|option| !option.disabled) else {
                return;
            };
            text.set(String::new());
            on_change.emit(toggle(&values, &option.value));
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
    // Adds what was typed, where free text is allowed, and empties the field.
    let accept = {
        let (options, values, text, on_change) =
            (options.clone(), values.clone(), text.clone(), props.on_change.clone());
        move |typed: &str| {
            if allow_custom {
                let next = parse(typed).fold(None::<Vec<AttrValue>>, |next, value| {
                    add(next.as_deref().unwrap_or(&values), &options, value).or(next)
                });
                if let Some(next) = next {
                    on_change.emit(next);
                }
            }
            text.set(String::new());
        }
    };
    let remove = {
        let (values, input, on_change) = (values.clone(), input.clone(), props.on_change.clone());
        Callback::from(move |index: usize| {
            let mut next = values.clone();
            if index < next.len() {
                next.remove(index);
                on_change.emit(next);
            }
            // The button that had focus is gone.
            select_dom::focus(&input);
        })
    };

    let oninput = {
        let (options, text, show, accept) = (options.clone(), text.clone(), show.clone(), accept.clone());
        Callback::from(move |event: InputEvent| {
            let typed = target_value(&event);
            if allow_custom && typed.contains(',') {
                accept(&typed);
                return;
            }
            // With free text allowed, Enter must mean the text, not a match.
            let first = if allow_custom { None } else { edge(&options, &filter(&options, &typed, limit).0, false) };
            show(first);
            text.set(typed);
        })
    };
    let onkeydown = {
        let (active, text, show, pick, accept, remove, options, count) = (
            active.clone(),
            text.clone(),
            show.clone(),
            pick.clone(),
            accept.clone(),
            remove.clone(),
            options.clone(),
            values.len(),
        );
        Callback::from(move |event: KeyboardEvent| {
            if event.ctrl_key() || event.meta_key() {
                return;
            }
            let order = filter(&options, &text, limit).0;
            let delta = match event.key().as_str() {
                "ArrowDown" => 1,
                "ArrowUp" => -1,
                "PageDown" if is_open => PAGE,
                "PageUp" if is_open => -PAGE,
                "Enter" => {
                    if let (true, Some(index)) = (is_open, active.filter(|index| order.contains(index))) {
                        event.prevent_default();
                        pick.emit(index);
                    } else if allow_custom && !text.trim().is_empty() {
                        event.prevent_default();
                        accept(&text);
                    }
                    return;
                }
                "Backspace" if text.is_empty() && count > 0 => {
                    event.prevent_default();
                    remove.emit(count - 1);
                    return;
                }
                _ => return,
            };
            if !has_choices {
                return;
            }
            event.prevent_default();
            if is_open {
                active.set(step(&options, &order, *active, delta));
            } else {
                show(edge(&options, &order, delta < 0));
            }
        })
    };
    let onblur = {
        let (open, accept, text) = (open.clone(), accept.clone(), text.clone());
        Callback::from(move |_: FocusEvent| {
            open.set(false);
            accept(&text);
        })
    };
    let onclick = {
        let (open, show) = (open.clone(), show.clone());
        Callback::from(move |_: MouseEvent| {
            if !*open {
                show(None);
            }
        })
    };
    // A press on the padding between the tags belongs to the input.
    let onmousedown = {
        let (open, show, input) = (open.clone(), show.clone(), input.clone());
        Callback::from(move |event: MouseEvent| {
            if select_dom::is_target(&event, &input) || dom::on_button(&event) {
                return;
            }
            event.prevent_default();
            select_dom::focus(&input);
            if *open { open.set(false) } else { show(None) }
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
                select_dom::scroll_into_view(&option_id(&base, index));
            }
        });
    }

    let active_shown = active.filter(|index| is_open && order.contains(index));
    let has_list = is_open && !order.is_empty();
    let typed = text.trim();
    let addable = allow_custom && !typed.is_empty() && add(values, options, typed).is_some();
    let content = if has_list {
        Listbox {
            id: list_id.clone(),
            base: &base,
            options,
            order: &order,
            selected: &selected,
            multi: true,
            active: active_shown,
            width: *width,
            label: props.label.clone(),
            labelled_by: field.as_ref().map(|field| field.label_id.clone()),
            on_pick: pick,
            on_hover: hover,
        }
        .render()
    } else if is_open && !addable {
        let style = format!("--yi-select-anchor-width:{}px", *width);
        let message = if allow_custom && !typed.is_empty() {
            AttrValue::from(format!("{typed} is already added"))
        } else {
            props.empty.clone()
        };
        html! { <div class="yi-select__status" role="status" {style}>{ message }</div> }
    } else {
        Html::default()
    };
    // Enter means the highlighted option while there is one.
    let foot = if is_open && addable && active_shown.is_none() {
        html! { <span class="yi-tag-picker__hint">{ "Press Enter to add " }<strong>{ typed.to_owned() }</strong></span> }
    } else if has_list && total > order.len() {
        html! { { format!("Showing the first {} of {total}. Type to narrow the list.", order.len()) } }
    } else {
        Html::default()
    };

    let label_of = |value: &AttrValue| {
        options.iter().find(|option| option.value == *value).map_or_else(|| value.clone(), |option| option.label.clone())
    };
    let tags = values.iter().enumerate().map(|(index, value)| {
        let label = label_of(value);
        let onclick = remove.reform(move |_: MouseEvent| index);
        html! {
            <span key={value.to_string()} class="yi-tag yi-tag--neutral yi-tag-picker__tag" role="listitem">
                <span class="yi-tag__text"><span class="yi-tag__value">{ label.clone() }</span></span>
                <button
                    class="yi-tag__remove"
                    type="button"
                    disabled={control.disabled}
                    aria-label={format!("{} {label}", props.remove_label)}
                    {onclick}
                >
                    <Icon icon={icons::X} />
                </button>
            </span>
        }
    });

    let class = classes!(
        input_classes(crate::Size::default(), props.mono, control.invalid),
        "yi-select__control",
        "yi-select__combobox",
        "yi-tag-picker__control",
        control.disabled.then_some("yi-input--disabled"),
    );
    let root = classes!("yi-select", "yi-tag-picker", props.class.clone());

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
                {foot}
                surface_class="yi-select__surface"
            >
                <span ref={anchor} {class} {onmousedown}>
                    if !values.is_empty() {
                        <span class="yi-tag-picker__tags" role="list">{ for tags }</span>
                    }
                    <input
                        ref={input}
                        id={control.id.clone()}
                        class="yi-select__input yi-tag-picker__input"
                        type="text"
                        role="combobox"
                        value={(*text).clone()}
                        placeholder={values.is_empty().then(|| props.placeholder.clone()).flatten()}
                        autocomplete="off"
                        autocapitalize="none"
                        spellcheck="false"
                        aria-autocomplete="list"
                        aria-haspopup="listbox"
                        aria-expanded={if has_list { "true" } else { "false" }}
                        aria-controls={has_list.then(|| list_id.clone())}
                        aria-activedescendant={active_shown.map(|index| option_id(&base, index))}
                        aria-label={props.label.clone()}
                        aria-invalid={control.aria_invalid()}
                        aria-required={control.required.then_some("true")}
                        aria-describedby={control.described_by.clone()}
                        disabled={control.disabled}
                        {oninput}
                        {onkeydown}
                        {onblur}
                        {onclick}
                    />
                </span>
            </Popover>
            if let Some(name) = &props.name {
                { for values.iter().map(|value| html! {
                    <input type="hidden" name={name.clone()} value={value.clone()} disabled={control.disabled} />
                }) }
            }
        </div>
    })
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use wasm_bindgen::JsCast;
    use web_sys::{Element, MouseEvent};

    /// Whether the press landed on a button, such as a tag's remove button.
    pub fn on_button(event: &MouseEvent) -> bool {
        event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .is_some_and(|target| target.closest("button").ok().flatten().is_some())
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    //! Events never happen outside a browser.

    pub fn on_button<E>(_: &E) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
