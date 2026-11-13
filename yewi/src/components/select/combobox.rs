use yew::prelude::*;

use super::{
    PAGE, SelectOptions, dom, more_label,
    listbox::{Listbox, option_id},
    options::{edge, filter, step},
};
use crate::{
    Attrs, FieldContext, Icon, Placement, Popover, PopoverTrigger, Size, Spinner,
    components::input::{input_classes, target_value},
    icons, use_field_control, use_id,
};

#[derive(Properties, PartialEq)]
pub struct ComboboxProps {
    /// The choices, in display order.
    pub options: SelectOptions,
    /// The `value` of the chosen option, or with `allow_custom` any text.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Fired with an option's value when the user chooses a different one,
    /// or with the typed text when `allow_custom` accepts it.
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    /// Fired with the text on every edit, for loading options from a server.
    /// Pair it with `filter={false}` when the server does the matching.
    #[prop_or_default]
    pub on_input: Callback<String>,
    /// Narrows the list to the options matching the typed text. Turn it off
    /// when `options` already holds only matches.
    #[prop_or(true)]
    pub filter: bool,
    /// The most options rendered at once. The rest are counted under the
    /// list, so a list of thousands stays cheap to open and to type in.
    #[prop_or(50)]
    pub max_results: usize,
    /// Accepts text that matches no option: <kbd>Enter</kbd> or leaving the
    /// field reports it through `on_change`.
    #[prop_or_default]
    pub allow_custom: bool,
    /// Shows a spinner while options are on their way.
    #[prop_or_default]
    pub loading: bool,
    /// Shown in place of the list while `loading` with no options yet.
    #[prop_or(AttrValue::Static("Loading"))]
    pub loading_label: AttrValue,
    /// The line under a list cut off at `max_results`, from the number shown
    /// and the number that match. Defaults to "Showing the first 50 of 2,000.
    /// Type to narrow the list." in English.
    #[prop_or_default]
    pub more_label: Option<Callback<(usize, usize), AttrValue>>,
    /// Shown in place of the list when nothing matches.
    #[prop_or(AttrValue::Static("No matches"))]
    pub empty: AttrValue,
    /// Shown while the field is empty.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub size: Size,
    /// Sizes the control to a compact width instead of filling its
    /// container, for toolbars and filter rows.
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

/// A text input that narrows a list of options as the user types.
///
/// - Keyboard: typing filters, <kbd>↓</kbd> <kbd>↑</kbd> move,
///   <kbd>Enter</kbd> chooses, <kbd>Esc</kbd> closes the list and then
///   restores the text.
/// - Leaving the field without choosing puts the current choice back, unless
///   `allow_custom` accepts the text as typed.
/// - At most `max_results` options are rendered however long the list is.
#[function_component]
pub fn Combobox(props: &ComboboxProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
    let field = use_context::<FieldContext>();
    let base = use_id();
    let anchor = use_node_ref();
    let input = props.input_ref.clone();
    let open = use_state(|| false);
    let active = use_state(|| None::<usize>);
    let width = use_state(|| 0.0_f64);
    // What the user is typing; `None` while the field shows the choice.
    let text = use_state(|| None::<String>);

    let options = &props.options;
    let selected = props.value.as_ref().and_then(|value| options.iter().position(|option| option.value == *value));
    let is_open = *open && !control.disabled;
    let list_id = AttrValue::from(format!("{base}-list"));
    let limit = props.max_results.max(1);
    let filtering = props.filter;
    let allow_custom = props.allow_custom;

    // The settled text: the choice's label, or a custom value as it is.
    let settled: AttrValue = match selected {
        Some(index) => options[index].label.clone(),
        None if allow_custom => props.value.clone().unwrap_or_default(),
        None => AttrValue::default(),
    };
    let shown: AttrValue = text.as_ref().map_or_else(|| settled.clone(), |text| text.clone().into());

    // The options on show: all of them until the user types.
    let visible = {
        let options = options.clone();
        move |text: Option<&str>| filter(&options, text.filter(|_| filtering).unwrap_or(""), limit)
    };
    let (order, total) = if is_open { visible(text.as_deref()) } else { (Vec::new(), 0) };

    let show = {
        let (open, active, width, anchor) = (open.clone(), active.clone(), width.clone(), anchor.clone());
        move |start: Option<usize>| {
            width.set(dom::width_of(&anchor));
            active.set(start);
            open.set(true);
        }
    };
    let pick = {
        let (options, open, text, on_change) = (options.clone(), open.clone(), text.clone(), props.on_change.clone());
        Callback::from(move |index: usize| {
            let Some(option) = options.get(index).filter(|option| !option.disabled) else {
                return;
            };
            open.set(false);
            text.set(None);
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
    // Reports typed text as the value, where that is allowed.
    let accept = {
        let (text, on_change, value) = (text.clone(), props.on_change.clone(), props.value.clone());
        move || {
            if let (true, Some(typed)) = (allow_custom, (*text).clone())
                && value.as_deref() != Some(typed.as_str())
            {
                on_change.emit(typed.into());
            }
            text.set(None);
        }
    };

    let oninput = {
        let (options, text, show, visible, on_input) =
            (options.clone(), text.clone(), show.clone(), visible.clone(), props.on_input.clone());
        Callback::from(move |event: InputEvent| {
            let typed = target_value(&event);
            // With free text allowed, Enter must mean the text, not a match.
            let first = if allow_custom { None } else { edge(&options, &visible(Some(&typed)).0, false) };
            show(first);
            on_input.emit(typed.clone());
            text.set(Some(typed));
        })
    };
    let onkeydown = {
        let (open, active, text, show, pick, accept, visible, options) = (
            open.clone(),
            active.clone(),
            text.clone(),
            show.clone(),
            pick.clone(),
            accept.clone(),
            visible.clone(),
            options.clone(),
        );
        Callback::from(move |event: KeyboardEvent| {
            if event.ctrl_key() || event.meta_key() {
                return;
            }
            let order = visible(text.as_deref()).0;
            let delta = match event.key().as_str() {
                "ArrowDown" => 1,
                "ArrowUp" => -1,
                "PageDown" if *open => PAGE,
                "PageUp" if *open => -PAGE,
                "Enter" => {
                    if let (true, Some(index)) = (*open, *active) {
                        event.prevent_default();
                        pick.emit(index);
                    } else if allow_custom && text.is_some() {
                        event.prevent_default();
                        open.set(false);
                        accept();
                    }
                    return;
                }
                // The first Escape closed the list; this one drops the edit.
                "Escape" if !*open && text.is_some() => {
                    event.prevent_default();
                    text.set(None);
                    return;
                }
                _ => return,
            };
            event.prevent_default();
            if *open {
                active.set(step(&options, &order, *active, delta));
            } else {
                let start = selected.filter(|index| order.contains(index));
                show(start.or_else(|| edge(&options, &order, delta < 0)));
            }
        })
    };
    let onblur = {
        let (open, accept) = (open.clone(), accept.clone());
        Callback::from(move |_: FocusEvent| {
            open.set(false);
            accept();
        })
    };
    let onclick = {
        let (open, show) = (open.clone(), show.clone());
        Callback::from(move |_: MouseEvent| {
            if !*open {
                show(selected);
            }
        })
    };
    // A press on the chevron or the padding belongs to the input.
    let onmousedown = {
        let (open, show, input) = (open.clone(), show.clone(), input.clone());
        Callback::from(move |event: MouseEvent| {
            if dom::is_target(&event, &input) {
                return;
            }
            event.prevent_default();
            dom::focus(&input);
            if *open { open.set(false) } else { show(selected) }
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

    let active_shown = active.filter(|index| is_open && order.contains(index));
    let has_list = is_open && !order.is_empty();
    let content = if has_list {
        Listbox {
            id: list_id.clone(),
            base: &base,
            options,
            order: &order,
            selected: selected.as_slice(),
            multi: false,
            active: active_shown,
            width: *width,
            label: props.label.clone(),
            labelled_by: field.as_ref().map(|field| field.label_id.clone()),
            on_pick: pick,
            on_hover: hover,
        }
        .render()
    } else if is_open {
        let style = format!("--yi-select-anchor-width:{}px", *width);
        let message = if props.loading { props.loading_label.clone() } else { props.empty.clone() };
        html! { <div class="yi-select__status" role="status" {style}>{ message }</div> }
    } else {
        Html::default()
    };
    let foot = if has_list && total > order.len() {
        html! { { more_label(props.more_label.as_ref(), order.len(), total) } }
    } else {
        Html::default()
    };

    let class = classes!(
        input_classes(props.size, false, control.invalid),
        "yi-select__control",
        "yi-select__combobox",
        control.disabled.then_some("yi-input--disabled"),
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
                {foot}
                surface_class="yi-select__surface"
            >
                <span ref={anchor} {class} {onmousedown}>
                    <input
                        ref={input}
                        id={control.id.clone()}
                        class="yi-select__input"
                        type="text"
                        role="combobox"
                        value={shown}
                        placeholder={props.placeholder.clone()}
                        autocomplete="off"
                        autocapitalize="none"
                        spellcheck="false"
                        aria-autocomplete="list"
                        aria-haspopup="listbox"
                        aria-expanded={if is_open { "true" } else { "false" }}
                        aria-controls={has_list.then(|| list_id.clone())}
                        aria-activedescendant={active_shown.map(|index| option_id(&base, index))}
                        aria-label={props.label.clone()}
                        aria-invalid={control.aria_invalid()}
                        aria-describedby={control.described_by.clone()}
                        aria-busy={props.loading.then_some("true")}
                        disabled={control.disabled}
                        required={control.required}
                        {oninput}
                        {onkeydown}
                        {onblur}
                        {onclick}
                    />
                    if props.loading {
                        <Spinner class="yi-select__chevron" />
                    } else {
                        <Icon icon={icons::CHEVRON_DOWN} class="yi-select__chevron" />
                    }
                </span>
            </Popover>
            if let Some(name) = &props.name {
                <input type="hidden" name={name.clone()} value={props.value.clone()} disabled={control.disabled} />
            }
        </div>
    })
}
