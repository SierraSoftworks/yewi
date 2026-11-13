//! History strips: one small cell per period or run, coloured by how it
//! went, with the detail of a cell on hover.

use std::rc::Rc;

use yew::prelude::*;

use crate::{Attrs, Placement, Popover, PopoverTrigger, Size, Tone};

/// How the cells of a [`HistoryStrip`] are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum HistoryStripVariant {
    /// Cells side by side in one bar that fills the width, a hairline apart: a continuous record,
    /// such as availability per hour.
    #[default]
    Continuous,
    /// Separate cells of a fixed width, gathered at the end: discrete events,
    /// such as the last runs of a job.
    Discrete,
}

impl HistoryStripVariant {
    pub const ALL: [HistoryStripVariant; 2] = [HistoryStripVariant::Continuous, HistoryStripVariant::Discrete];

    const fn as_str(self) -> &'static str {
        match self {
            HistoryStripVariant::Continuous => "continuous",
            HistoryStripVariant::Discrete => "discrete",
        }
    }
}

/// One cell of a [`HistoryStrip`].
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryCell {
    /// How the period went. `Tone::Neutral` stands for "no data".
    pub tone: Tone,
    /// The cell's accessible name: "11:20, failing".
    pub label: AttrValue,
}

impl HistoryCell {
    pub fn new(tone: Tone, label: impl Into<AttrValue>) -> Self {
        Self { tone, label: label.into() }
    }
}

/// What the popover of one cell shows. The layout of each part is the
/// caller's.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HistoryDetail {
    /// A heading row: a status dot and a short label.
    pub header: Html,
    /// The body.
    pub content: Html,
    /// A muted row under the body: the cell's time.
    pub footer: Html,
}

impl HistoryDetail {
    pub fn new(content: impl Into<Html>) -> Self {
        Self { content: content.into(), ..Self::default() }
    }

    pub fn header(mut self, header: impl Into<Html>) -> Self {
        self.header = header.into();
        self
    }

    pub fn footer(mut self, footer: impl Into<Html>) -> Self {
        self.footer = footer.into();
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct HistoryStripProps {
    /// The cells, oldest first.
    pub cells: Rc<[HistoryCell]>,
    /// Builds the popover for the cell at an index. It is called only for the
    /// cell being looked at. Without it the strip is not interactive.
    #[prop_or_default]
    pub detail: Option<Callback<usize, HistoryDetail>>,
    /// What the strip records, for assistive technology: "Last 24 hours".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub variant: HistoryStripVariant,
    /// The strip's height.
    #[prop_or_default]
    pub size: Size,
    /// The preferred side for the popover.
    #[prop_or_default]
    pub placement: Placement,
    /// Called with a cell's index when it is pressed, or chosen with
    /// <kbd>Enter</kbd>.
    #[prop_or_default]
    pub on_select: Callback<usize>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The cell <kbd>key</kbd> moves to from `active`, in a strip of `len` cells.
fn step(key: &str, active: Option<usize>, len: usize) -> Option<usize> {
    let last = len.checked_sub(1)?;
    match (key, active) {
        ("Home", _) => Some(0),
        ("End", _) | ("ArrowLeft" | "ArrowRight", None) => Some(last),
        ("ArrowLeft", Some(i)) => Some(i.saturating_sub(1)),
        ("ArrowRight", Some(i)) => Some((i + 1).min(last)),
        _ => None,
    }
}

/// A row of cells, oldest first, each coloured by its tone.
///
/// - With `detail`, hovering or pressing a cell shows its popover, and the
///   strip is one tab stop: the arrow keys, <kbd>Home</kbd> and
///   <kbd>End</kbd> move between cells, <kbd>Esc</kbd> closes the popover.
/// - Only the cell being looked at has a popover, and its content is built on
///   demand, so a strip costs one element per cell however rich the detail.
/// - When there is not room for every cell, the oldest are clipped.
#[function_component]
pub fn HistoryStrip(props: &HistoryStripProps) -> Html {
    let root = use_node_ref();
    let active = use_state(|| None::<usize>);
    let len = props.cells.len();
    let interactive = props.detail.is_some();
    let current = active.filter(|index| interactive && *index < len);

    let handlers = {
        let (root, setter, on_select) = (root.clone(), active.setter(), props.on_select.clone());
        use_memo((len, props.on_select.clone()), move |_| {
            let over = {
                let setter = setter.clone();
                Callback::from(move |event: MouseEvent| {
                    if let Some(index) = dom::cell_index(&event) {
                        setter.set(Some(index));
                    }
                })
            };
            let leave = {
                let setter = setter.clone();
                Callback::from(move |_: MouseEvent| setter.set(None))
            };
            let click = {
                let (setter, on_select) = (setter.clone(), on_select.clone());
                Callback::from(move |event: MouseEvent| {
                    if let Some(index) = dom::cell_index(&event).filter(|_| !dom::in_popover(&event)) {
                        setter.set(Some(index));
                        on_select.emit(index);
                    }
                })
            };
            let focus = {
                let (root, setter) = (root.clone(), setter.clone());
                Callback::from(move |event: FocusEvent| {
                    if dom::is_own(&event, &root) && dom::is_keyboard_focus(&event) {
                        setter.set(len.checked_sub(1));
                    }
                })
            };
            let blur = {
                let (root, setter) = (root.clone(), setter.clone());
                Callback::from(move |event: FocusEvent| {
                    if !dom::moved_inside(&event, &root) {
                        setter.set(None);
                    }
                })
            };
            let close = Callback::from(move |open: bool| {
                if !open {
                    setter.set(None);
                }
            });
            (over, leave, click, focus, blur, close)
        })
    };
    let (over, leave, click, focus, blur, close) = (*handlers).clone();

    let key_down = {
        let (root, setter, on_select) = (root.clone(), active.setter(), props.on_select.clone());
        Callback::from(move |event: KeyboardEvent| {
            if !dom::is_own(&event, &root) {
                return;
            }
            let key = event.key();
            if let Some(next) = step(&key, current, len) {
                event.prevent_default();
                setter.set(Some(next));
            } else if let ("Enter" | " ", Some(index)) = (key.as_str(), current) {
                event.prevent_default();
                on_select.emit(index);
            }
        })
    };

    let class = classes!(
        "yi-history-strip",
        format!("yi-history-strip--{}", props.variant.as_str()),
        format!("yi-history-strip--{}", props.size.as_str()),
        interactive.then_some("yi-history-strip--interactive"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div
            ref={root}
            id={props.id.clone()}
            {class}
            role="group"
            aria-label={props.label.clone()}
            tabindex={interactive.then_some("0")}
            onmouseover={interactive.then_some(over)}
            onmouseleave={interactive.then_some(leave)}
            onclick={interactive.then_some(click)}
            onfocusin={interactive.then_some(focus)}
            onfocusout={interactive.then_some(blur)}
            onkeydown={interactive.then_some(key_down)}
        >
            { for props.cells.iter().enumerate().map(|(index, cell)| {
                let is_active = current == Some(index);
                let named = !cell.label.is_empty();
                html! {
                    <span
                        class={classes!("yi-history-strip__cell", format!("yi-history-strip__cell--{}", cell.tone.as_str()))}
                        data-yi-index={index.to_string()}
                        data-yi-active={is_active.then_some("")}
                        role={named.then_some("img")}
                        aria-label={named.then(|| cell.label.clone())}
                        aria-hidden={(!named).then_some("true")}
                    >
                        if let Some(detail) = props.detail.as_ref().filter(|_| is_active) {
                            { popover(detail.emit(index), props.placement, close.clone()) }
                        }
                    </span>
                }
            }) }
        </div>
    })
}

/// The active cell's popover, anchored to an element that covers the cell.
fn popover(detail: HistoryDetail, placement: Placement, on_open_change: Callback<bool>) -> Html {
    html! {
        <Popover
            class="yi-history-strip__anchor"
            trigger={PopoverTrigger::Manual}
            open=true
            role="status"
            {placement}
            header={detail.header}
            content={detail.content}
            footer={detail.footer}
            {on_open_change}
        />
    }
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use wasm_bindgen::JsCast;
    use web_sys::{Element, Event, FocusEvent, Node};
    use yew::NodeRef;

    fn target(event: &Event) -> Option<Element> {
        event.target().and_then(|target| target.dyn_into::<Element>().ok())
    }

    /// The index of the cell the event happened in. A cell's popover is
    /// inside it, so events there count as the cell's.
    pub fn cell_index(event: &Event) -> Option<usize> {
        let cell = target(event)?.closest("[data-yi-index]").ok().flatten()?;
        cell.get_attribute("data-yi-index")?.parse().ok()
    }

    pub fn in_popover(event: &Event) -> bool {
        target(event).is_some_and(|target| target.closest(".yi-popover__surface").ok().flatten().is_some())
    }

    pub fn is_keyboard_focus(event: &FocusEvent) -> bool {
        target(event).is_some_and(|target| target.matches(":focus-visible").unwrap_or(true))
    }

    /// Whether focus moved to somewhere inside the strip, such as a link in
    /// the popover.
    pub fn moved_inside(event: &FocusEvent, strip: &NodeRef) -> bool {
        let strip = strip.get();
        let next = event.related_target().and_then(|target| target.dyn_into::<Node>().ok());
        strip.zip(next).is_some_and(|(strip, next)| strip.contains(Some(&next)))
    }

    /// Whether the event happened on the strip itself, not in its popover.
    pub fn is_own(event: &Event, strip: &NodeRef) -> bool {
        let target = event.target().and_then(|target| target.dyn_into::<Node>().ok());
        strip.get().zip(target).is_some_and(|(strip, target)| strip == target)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    //! Events never run outside a browser.

    use yew::NodeRef;

    pub fn cell_index<E>(_: &E) -> Option<usize> {
        None
    }

    pub fn in_popover<E>(_: &E) -> bool {
        false
    }

    pub fn is_keyboard_focus<E>(_: &E) -> bool {
        false
    }

    pub fn moved_inside<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn is_own<E>(_: &E, _: &NodeRef) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
