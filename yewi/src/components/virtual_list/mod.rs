//! Long lists and tables that draw only what is in view. [`VirtualList`]
//! renders items your way in rows of one height; [`VirtualTable`] is
//! [`Table`](crate::Table) with the same windowing. Both keep a few dozen
//! rows in the document however many they are given, so a hundred thousand
//! rows cost what thirty do.

use std::{ops::Range, rc::Rc};

use yew::{prelude::*, virtual_dom::Key};

mod table;
mod window;

pub use table::*;
pub use window::*;

use crate::is_empty;
use crate::{Attrs, EmptyState, RowFn, Size};

#[derive(Properties, PartialEq)]
pub struct VirtualListProps<T: PartialEq + 'static> {
    /// The data, one item per row. Keep it in a stable `Rc` (state,
    /// `use_memo`) so rows are redrawn only when their own item changes.
    pub items: Rc<[T]>,
    /// Draws one item. Build it once (`use_memo`), not on every render.
    pub render: RowFn<T, Html>,
    /// A stable identity for each item, so a row keeps its state when items
    /// are added or reordered. Defaults to the item's position.
    #[prop_or_default]
    pub item_key: Option<RowFn<T, AttrValue>>,
    /// The height of every row in CSS pixels. Content that does not fit is
    /// clipped.
    #[prop_or(40.0)]
    pub row_height: f64,
    /// Caps the list's height as a CSS length; the rows scroll within it.
    /// `100%` fills a parent that has a height of its own.
    #[prop_or(AttrValue::Static("24rem"))]
    pub max_height: AttrValue,
    /// A hairline between rows.
    #[prop_or_default]
    pub ruled: bool,
    /// Extra rows drawn above and below the visible ones.
    #[prop_or(8)]
    pub overscan: usize,
    /// How many rows the server draws, before the list can be measured.
    /// About what fits in `max_height` avoids a second render on load.
    #[prop_or(12)]
    pub initial_rows: usize,
    /// Fired with the positions of the rows now drawn whenever the window
    /// moves. When its end reaches the number of items, it is time to load
    /// more.
    #[prop_or_default]
    pub on_range: Callback<Range<usize>>,
    /// Shown instead of rows when there are none. Defaults to a small
    /// [`EmptyState`].
    #[prop_or_default]
    pub empty: Html,
    /// The title of the default empty state, when `empty` is not set.
    #[prop_or(AttrValue::Static("Nothing to show"))]
    pub empty_label: AttrValue,
    /// Names the list for assistive technology.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A scrolling list that draws only the rows in view, plus a few either
/// side. Rows share one height, which is what makes any position reachable
/// without measuring the rows before it.
///
/// Each row is announced with its place in the whole list ("3 of 100,000").
/// The list itself takes keyboard focus so it scrolls with the arrow keys; a
/// focused control inside a row is removed, and loses focus, when its row
/// scrolls far out of view.
#[function_component]
pub fn VirtualList<T: PartialEq + 'static>(props: &VirtualListProps<T>) -> Html {
    let count = props.items.len();
    let window = use_virtual_window(VirtualOptions {
        count,
        row_height: props.row_height,
        overscan: props.overscan,
        initial_rows: props.initial_rows,
        scroller: ".yi-virtual-list",
        row: ".yi-virtual-list__item",
    });
    use_range_callback(&window.range, &props.on_range);

    let class = classes!("yi-virtual-list", props.ruled.then_some("yi-virtual-list--ruled"), props.class.clone());
    let style =
        format!("--yi-virtual-list-max-height:{};--yi-virtual-list-row:{}px", props.max_height, props.row_height);
    let range = window.range.clone();

    props.attrs.apply(html! {
        <div
            ref={window.anchor.clone()}
            id={props.id.clone()}
            {class}
            {style}
            role={(count > 0).then_some("list")}
            aria-label={props.label.clone()}
            tabindex="0"
        >
            if count == 0 {
                <div class="yi-virtual-list__empty">
                    if !is_empty(&props.empty) {
                        { props.empty.clone() }
                    } else {
                        <EmptyState title={props.empty_label.clone()} size={Size::Small} />
                    }
                </div>
            } else {
                <div
                    class="yi-virtual-list__sizer"
                    role="presentation"
                    style={format!("block-size:{}px", window.row_height * count as f64)}
                >
                    <div
                        class="yi-virtual-list__window"
                        role="presentation"
                        style={format!("translate:0 {}px", window.row_height * range.start as f64)}
                    >
                        { for range.map(|index| {
                            let key = match &props.item_key {
                                Some(item_key) => Key::from(item_key.call(&props.items[index]).to_string()),
                                None => Key::from(index),
                            };
                            html! {
                                <VirtualItem<T>
                                    {key}
                                    items={props.items.clone()}
                                    {index}
                                    render={props.render.clone()}
                                />
                            }
                        }) }
                    </div>
                </div>
            }
        </div>
    })
}

/// Reports the drawn rows to `on_range` when they change.
#[hook]
pub(crate) fn use_range_callback(range: &Range<usize>, on_range: &Callback<Range<usize>>) {
    let on_range = on_range.clone();
    use_effect_with(range.clone(), move |range| on_range.emit(range.clone()));
}

#[derive(Properties)]
struct VirtualItemProps<T: PartialEq + 'static> {
    items: Rc<[T]>,
    index: usize,
    render: RowFn<T, Html>,
}

impl<T: PartialEq + 'static> PartialEq for VirtualItemProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.render == other.render
            && (Rc::ptr_eq(&self.items, &other.items) || self.items.get(self.index) == other.items.get(other.index))
    }
}

/// One row. A component of its own so a row that stays in the window is
/// skipped when the window moves.
#[function_component]
fn VirtualItem<T: PartialEq + 'static>(props: &VirtualItemProps<T>) -> Html {
    let Some(item) = props.items.get(props.index) else { return Html::default() };
    html! {
        <div
            class="yi-virtual-list__item"
            role="listitem"
            aria-setsize={props.items.len().to_string()}
            aria-posinset={(props.index + 1).to_string()}
        >
            { props.render.call(item) }
        </div>
    }
}

#[cfg(test)]
mod tests;
