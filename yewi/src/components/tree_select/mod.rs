//! Choosing one entry out of a large hierarchy: [`TreeSelect`] is searched by
//! typing and walked by opening branches.
//!
//! It is controlled: it shows `value` and reports a choice through
//! `on_change`. Inside a [`Field`](crate::Field) it picks up its label, id,
//! description, and its required, disabled and invalid state.

use std::rc::Rc;

use yew::prelude::*;

use crate::{
    Attrs, FieldContext, Icon, Placement, Popover, PopoverTrigger, Size,
    components::{
        input::{input_classes, target_value},
        select::{dom as select_dom, more_label},
    },
    icons, use_field_control, use_id,
};

mod tree;

use tree::{Row, rows};
pub use tree::{TreeOption, TreeOptions};

/// How many rows <kbd>Page Up</kbd> and <kbd>Page Down</kbd> move by.
const PAGE: usize = 10;

#[derive(Properties, PartialEq)]
pub struct TreeSelectProps {
    /// The hierarchy to choose from.
    pub options: TreeOptions,
    /// The `value` of the chosen entry. `None` shows the placeholder; a value
    /// the hierarchy does not hold is shown as it stands.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Fired with an entry's value when the user chooses it, and with `None`
    /// when they choose the `clear_label` row.
    #[prop_or_default]
    pub on_change: Callback<Option<AttrValue>>,
    /// Shown while nothing is chosen.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// Adds a first row, under this name, that returns to nothing chosen.
    #[prop_or_default]
    pub clear_label: Option<AttrValue>,
    /// Lets text that names no entry be used as it stands. Called with the
    /// trimmed search text; return the value it stands for, or `None` when
    /// it is not well formed. The offer appears as the last row.
    #[prop_or_default]
    pub custom: Option<Callback<String, Option<AttrValue>>>,
    /// Draws the picture for an entry's `media`, before its name.
    #[prop_or_default]
    pub render_media: Option<Callback<AttrValue, Html>>,
    /// The most search results rendered at once. Past this, typing another
    /// letter is quicker than scrolling.
    #[prop_or(40)]
    pub max_results: usize,
    /// The line under a list cut off at `max_results`, from the number shown
    /// and the number that match. Defaults to "Showing the first 50 of 2,000.
    /// Type to narrow the list." in English.
    #[prop_or_default]
    pub more_label: Option<Callback<(usize, usize), AttrValue>>,
    /// Shown in place of the list when a search finds nothing.
    #[prop_or(AttrValue::Static("No matches"))]
    pub empty: AttrValue,
    /// The placeholder and accessible name of the search box.
    #[prop_or(AttrValue::Static("Search"))]
    pub search_label: AttrValue,
    /// The name of the top of the hierarchy, in the trail above the list.
    #[prop_or(AttrValue::Static("All"))]
    pub root_label: AttrValue,
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

/// A button showing the current choice, which opens a panel with a search
/// box over one level of the hierarchy.
///
/// - With nothing typed the list is one level: <kbd>→</kbd> opens the
///   highlighted branch, <kbd>←</kbd> goes back up, and the trail above the
///   list jumps to any level on the way. Typing searches the whole tree, best
///   match first, and each result says where it lives.
/// - Keyboard: <kbd>↓</kbd> <kbd>↑</kbd> <kbd>Home</kbd> <kbd>End</kbd> move,
///   <kbd>Enter</kbd> chooses (or opens a branch that cannot be chosen), and
///   <kbd>Esc</kbd> or <kbd>Tab</kbd> closes and returns focus to the button.
/// - Only one level, or at most `max_results` matches, is ever rendered, so a
///   catalogue of thousands of entries stays cheap to open and to search.
#[function_component]
pub fn TreeSelect(props: &TreeSelectProps) -> Html {
    let control = use_field_control(props.id.clone(), props.invalid, props.required, props.disabled);
    let field = use_context::<FieldContext>();
    let base = use_id();
    let (root_ref, trigger, search) = (use_node_ref(), use_node_ref(), use_node_ref());
    let open = use_state(|| false);
    let query = use_state(String::new);
    let branch = use_state(|| None::<usize>);
    let active = use_state(|| 0_usize);
    let width = use_state(|| 0.0_f64);

    let options = &props.options;
    let tree = options.tree();
    let chosen = props.value.as_ref().and_then(|value| tree.find(value));
    let is_open = *open && !control.disabled;
    let searching = !query.trim().is_empty();
    let clearable = props.clear_label.is_some();
    let limit = props.max_results.max(1);
    let custom =
        props.custom.as_ref().filter(|_| is_open && searching).and_then(|custom| custom.emit(query.trim().to_owned()));
    let (shown, total) =
        if is_open { rows(tree, &query, *branch, clearable, custom, limit) } else { (Vec::new(), 0) };
    let shown = Rc::new(shown);
    let list_id = AttrValue::from(format!("{base}-list"));
    let panel_id = AttrValue::from(format!("{base}-panel"));
    let hint_id = AttrValue::from(format!("{base}-hint"));
    let row_id = |index: usize| format!("{base}-row-{index}");

    let close = {
        let (open, query, trigger) = (open.clone(), query.clone(), trigger.clone());
        move |refocus: bool| {
            open.set(false);
            query.set(String::new());
            if refocus {
                select_dom::focus(&trigger);
            }
        }
    };
    // Opens on the level holding what is chosen, so the rows around it are
    // the first thing seen.
    let show = {
        let (open, branch, active, width, trigger, options) =
            (open.clone(), branch.clone(), active.clone(), width.clone(), trigger.clone(), options.clone());
        move || {
            let tree = options.tree();
            let home = chosen.and_then(|at| tree.node(at)).and_then(|node| node.parent);
            let at = chosen.and_then(|at| tree.level(home).iter().position(|row| *row == at));
            let offset = usize::from(clearable && home.is_none());
            width.set(select_dom::width_of(&trigger));
            branch.set(home);
            active.set(at.map_or(0, |at| at + offset));
            open.set(true);
        }
    };
    let enter = {
        let (branch, active, query, search) = (branch.clone(), active.clone(), query.clone(), search.clone());
        Callback::from(move |to: Option<usize>| {
            branch.set(to);
            active.set(0);
            query.set(String::new());
            select_dom::focus(&search);
        })
    };
    let pick = {
        let (on_change, close, enter, options, value) =
            (props.on_change.clone(), close.clone(), enter.clone(), options.clone(), props.value.clone());
        Callback::from(move |row: Row| {
            let next = match row {
                Row::Clear => None,
                Row::Custom(value) => Some(value),
                Row::Node(at) => match options.tree().node(at).and_then(|node| node.value.clone()) {
                    Some(value) => Some(value),
                    // A branch that is not itself an entry: its only use is
                    // what is inside.
                    None => return enter.emit(Some(at)),
                },
            };
            close(true);
            if next != value {
                on_change.emit(next);
            }
        })
    };

    let onclick = {
        let (open, show, close) = (open.clone(), show.clone(), close.clone());
        Callback::from(move |_: MouseEvent| if *open { close(false) } else { show() })
    };
    let ontriggerkey = {
        let (open, show) = (open.clone(), show.clone());
        Callback::from(move |event: KeyboardEvent| {
            if !*open && matches!(event.key().as_str(), "ArrowDown" | "ArrowUp") {
                event.prevent_default();
                show();
            }
        })
    };
    let oninput = {
        let (query, active) = (query.clone(), active.clone());
        Callback::from(move |event: InputEvent| {
            query.set(target_value(&event));
            active.set(0);
        })
    };
    let onkeydown = {
        let (active, close, pick, enter, shown, options) =
            (active.clone(), close.clone(), pick.clone(), enter.clone(), shown.clone(), options.clone());
        let up = branch.and_then(|at| tree.node(at)).map(|node| node.parent);
        Callback::from(move |event: KeyboardEvent| {
            if event.ctrl_key() || event.meta_key() || event.alt_key() {
                return;
            }
            let last = shown.len().saturating_sub(1);
            let inside = match shown.get(*active) {
                Some(Row::Node(at)) if options.tree().node(*at).is_some_and(|node| !node.children.is_empty()) => {
                    Some(*at)
                }
                _ => None,
            };
            match event.key().as_str() {
                "ArrowDown" => active.set((*active + 1).min(last)),
                "ArrowUp" => active.set(active.saturating_sub(1)),
                "PageDown" => active.set((*active + PAGE).min(last)),
                "PageUp" => active.set(active.saturating_sub(PAGE)),
                // Only while nothing is typed: in a search these move the caret.
                "Home" if !searching => active.set(0),
                "End" if !searching => active.set(last),
                "ArrowRight" if !searching && inside.is_some() => enter.emit(inside),
                "ArrowLeft" if !searching && up.is_some() => enter.emit(up.flatten()),
                "Enter" => {
                    if let Some(row) = shown.get(*active) {
                        pick.emit(row.clone());
                    }
                }
                "Escape" => {
                    event.stop_propagation();
                    close(true);
                }
                // Back to the control, from where the next Tab moves on.
                "Tab" => close(true),
                _ => return,
            }
            event.prevent_default();
        })
    };
    let onfocusout = {
        let (close, root_ref) = (close.clone(), root_ref.clone());
        Callback::from(move |event: FocusEvent| {
            // Focus moving to the button is a press on it, which closes anyway.
            if dom::left(&event, &root_ref) {
                close(false);
            }
        })
    };
    let on_open_change = {
        let (close, search) = (close.clone(), search.clone());
        Callback::from(move |next: bool| {
            if !next {
                // Escape hands focus back; a press elsewhere takes it there.
                close(dom::has_focus(&search));
            }
        })
    };
    let onrowclick = {
        let (pick, enter, shown) = (pick.clone(), enter.clone(), shown.clone());
        Callback::from(move |event: MouseEvent| {
            let Some(row) = select_dom::option_index(&event).and_then(|index| shown.get(index)) else {
                return;
            };
            match row {
                Row::Node(at) if dom::on_enter(&event) => enter.emit(Some(*at)),
                row => pick.emit(row.clone()),
            }
        })
    };
    // Movement, not `mouseover`: the list scrolling under a resting pointer
    // during keyboard use must not steal the highlight.
    let onmousemove = {
        let active = active.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(index) = select_dom::option_index(&event).filter(|index| *index != *active) {
                active.set(index);
            }
        })
    };
    // Keeps focus on the search box while the pointer works the panel.
    let keep_focus = {
        let search = search.clone();
        Callback::from(move |event: MouseEvent| {
            if !select_dom::is_target(&event, &search) {
                event.prevent_default();
            }
        })
    };

    {
        let search = search.clone();
        use_effect_with(is_open, move |open| {
            if *open {
                select_dom::focus(&search);
            }
        });
    }
    {
        let at = row_id(*active);
        use_effect_with((is_open, *active, *branch, (*query).clone()), move |(open, ..)| {
            if *open {
                select_dom::scroll_into_view(&at);
            }
        });
    }

    let media = |node: &tree::Node| match (&props.render_media, &node.media, node.icon) {
        (Some(render), Some(media), _) => html! { <span class="yi-tree-select__media">{ render.emit(media.clone()) }</span> },
        (_, _, Some(icon)) => html! { <Icon {icon} class="yi-select__option-icon" /> },
        // Keeps the names in one column where some entries have a picture.
        (Some(_), None, None) => html! { <span class="yi-tree-select__media" /> },
        _ => Html::default(),
    };

    let content = if is_open {
        let style = format!("--yi-select-anchor-width:{}px", *width);
        // Every child of the panel is always rendered, and hidden when it has
        // nothing to show: a child that came and went would make Yew rebuild
        // the search box before it, dropping focus.
        let steps = branch.filter(|_| !searching).map_or_else(Vec::new, |at| {
            let mut steps = tree.trail(at);
            steps.push(at);
            steps
        });
        let last = steps.len().saturating_sub(1);
        let trail = html! {
            <nav class="yi-tree-select__trail" aria-label="Level" hidden={steps.is_empty()}>
                <button type="button" tabindex="-1" onclick={enter.reform(|_: MouseEvent| None)}>
                    { props.root_label.clone() }
                </button>
                { for steps.iter().enumerate().map(|(index, step)| {
                    let step = *step;
                    let label = tree.node(step).map(|node| node.label.clone()).unwrap_or_default();
                    html! {
                        <button
                            key={step}
                            type="button"
                            tabindex="-1"
                            aria-current={(index == last).then_some("true")}
                            onclick={enter.reform(move |_: MouseEvent| Some(step))}
                        >
                            { label }
                        </button>
                    }
                }) }
            </nav>
        };
        let row = |(index, row): (usize, &Row)| {
            let class = classes!(
                "yi-select__option",
                "yi-tree-select__row",
                (index == *active).then_some("yi-select__option--active"),
            );
            let (selected, described_by, body) = match row {
                Row::Clear => (
                    props.value.is_none(),
                    None,
                    html! {
                        <span class="yi-select__option-text">
                            <span class="yi-select__option-label yi-tree-select__clear">
                                { props.clear_label.clone().unwrap_or_default() }
                            </span>
                        </span>
                    },
                ),
                Row::Custom(value) => (
                    false,
                    None,
                    html! {
                        <span class="yi-select__option-text">
                            <span class="yi-select__option-label">
                                { "Use " }<span class="yi-tree-select__detail">{ value.clone() }</span>
                            </span>
                        </span>
                    },
                ),
                Row::Node(at) => {
                    let Some(node) = tree.node(*at) else { return Html::default() };
                    let above: Vec<&str> =
                        tree.trail(*at).into_iter().filter_map(|step| tree.node(step)).map(|n| &*n.label).collect();
                    let has_inside = !node.children.is_empty();
                    (
                        chosen == Some(*at),
                        has_inside.then(|| hint_id.clone()),
                        html! {
                            <>
                                { media(node) }
                                <span class="yi-select__option-text">
                                    <span class="yi-select__option-label">{ node.label.clone() }</span>
                                    if searching && !above.is_empty() {
                                        <span class="yi-select__option-description">{ above.join(" \u{203a} ") }</span>
                                    }
                                </span>
                                if let Some(detail) = &node.detail {
                                    <span class="yi-tree-select__detail">{ detail.clone() }</span>
                                }
                                if chosen == Some(*at) {
                                    <Icon icon={icons::CHECK} class="yi-select__check" />
                                }
                                if has_inside {
                                    <span class="yi-tree-select__enter" data-yi-enter="" aria-hidden="true">
                                        <Icon icon={icons::CHEVRON_RIGHT} />
                                    </span>
                                }
                            </>
                        },
                    )
                }
            };
            html! {
                <div
                    key={index}
                    id={row_id(index)}
                    {class}
                    role="option"
                    aria-selected={if selected { "true" } else { "false" }}
                    aria-describedby={described_by}
                    data-yi-option={index.to_string()}
                >
                    { body }
                </div>
            }
        };
        html! {
            <div id={panel_id.clone()} class="yi-tree-select__panel" {style} onmousedown={keep_focus} {onfocusout}>
                <label class="yi-tree-select__search">
                    <Icon icon={icons::SEARCH} />
                    <input
                        ref={search}
                        class="yi-select__input"
                        type="text"
                        role="combobox"
                        value={(*query).clone()}
                        placeholder={props.search_label.clone()}
                        autocomplete="off"
                        autocapitalize="none"
                        spellcheck="false"
                        aria-label={props.search_label.clone()}
                        aria-autocomplete="list"
                        aria-expanded="true"
                        aria-controls={list_id.clone()}
                        aria-activedescendant={(!shown.is_empty()).then(|| row_id(*active))}
                        {oninput}
                        {onkeydown}
                    />
                </label>
                { trail }
                <div
                    id={list_id}
                    class="yi-tree-select__list"
                    role="listbox"
                    aria-label={props.label.clone()}
                    aria-labelledby={field.as_ref().map(|field| field.label_id.clone())}
                    onclick={onrowclick}
                    {onmousemove}
                >
                    { for shown.iter().enumerate().map(row) }
                </div>
                <div class="yi-select__status" role="status" hidden={!shown.is_empty()}>
                    if shown.is_empty() {
                        { props.empty.clone() }
                    }
                </div>
                <span id={hint_id.clone()} hidden=true>{ "Right arrow shows what is inside" }</span>
            </div>
        }
    } else {
        Html::default()
    };
    let footer = if is_open && searching && total > limit {
        html! { { more_label(props.more_label.as_ref(), limit, total) } }
    } else {
        Html::default()
    };

    let current = chosen.and_then(|at| tree.node(at));
    let class = classes!(input_classes(props.size, false, control.invalid), "yi-select__control", "yi-select__trigger");
    let root =
        classes!("yi-select", "yi-tree-select", props.fit.then_some("yi-select--fit"), props.class.clone());

    props.attrs.apply(html! {
        <div ref={root_ref} class={root}>
            <Popover
                trigger={PopoverTrigger::Manual}
                placement={Placement::BottomStart}
                arrow=false
                flush=true
                role="presentation"
                open={is_open}
                {on_open_change}
                {content}
                {footer}
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
                    aria-controls={is_open.then(|| panel_id.clone())}
                    aria-label={props.label.clone()}
                    aria-invalid={control.aria_invalid()}
                    aria-required={control.required.then_some("true")}
                    aria-describedby={control.described_by.clone()}
                    disabled={control.disabled}
                    {onclick}
                    onkeydown={ontriggerkey}
                >
                    { current.map(&media) }
                    { match (current, &props.value) {
                        (Some(node), _) => html! { <span class="yi-select__value">{ node.label.clone() }</span> },
                        // Chosen, and not something the tree lists: say what it is.
                        (None, Some(value)) => html! { <span class="yi-select__value">{ value.clone() }</span> },
                        (None, None) => html! {
                            <span class="yi-select__value yi-select__value--placeholder">
                                { props.placeholder.clone().unwrap_or_default() }
                            </span>
                        },
                    } }
                    if let Some(detail) = current.and_then(|node| node.detail.as_ref()) {
                        <span class="yi-tree-select__detail">{ detail.clone() }</span>
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
mod dom {
    use wasm_bindgen::JsCast;
    use web_sys::{Element, FocusEvent, MouseEvent, Node};
    use yew::NodeRef;

    /// Whether the press landed on a row's "show what is inside" arrow.
    pub fn on_enter(event: &MouseEvent) -> bool {
        event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .is_some_and(|target| target.closest("[data-yi-enter]").ok().flatten().is_some())
    }

    /// Whether focus moved to an element outside `container`. Focus that
    /// went nowhere (another window) has not left.
    pub fn left(event: &FocusEvent, container: &NodeRef) -> bool {
        let to = event.related_target().and_then(|target| target.dyn_into::<Node>().ok());
        match (to, container.get()) {
            (Some(to), Some(container)) => !container.contains(Some(&to)),
            _ => false,
        }
    }

    pub fn has_focus(element: &NodeRef) -> bool {
        let active = web_sys::window().and_then(|w| w.document()).and_then(|d| d.active_element());
        element.get().zip(active).is_some_and(|(element, active)| element == *active.as_ref())
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    //! Events never happen outside a browser.

    use yew::NodeRef;

    pub fn on_enter<E>(_: &E) -> bool {
        false
    }

    pub fn left<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn has_focus(_: &NodeRef) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
