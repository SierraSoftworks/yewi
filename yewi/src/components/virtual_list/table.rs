//! [`VirtualTable`]: a [`Table`](crate::Table) that draws only the rows in
//! view.

use std::{ops::Range, rc::Rc};

use yew::prelude::*;

use super::{VirtualOptions, use_range_callback, use_virtual_window};
use crate::{
    Attrs, Column, RowFn, Selection, Sort, TableProps, TableVariant,
    components::table::use_table,
};

/// The height in CSS pixels, at the default font size, of a
/// [`Table`](crate::Table) row holding one line of text, so a virtualised
/// table has the rhythm of a plain one. A compact row is a little taller
/// when it holds a checkbox.
const fn default_row_height(compact: bool, selectable: bool) -> f64 {
    match (compact, selectable) {
        (false, _) => 42.0,
        (true, false) => 32.5,
        (true, true) => 34.0,
    }
}

#[derive(Properties, PartialEq)]
pub struct VirtualTableProps<T: Clone + PartialEq + 'static> {
    /// The columns, in display order. Build them once with
    /// [`use_columns`](crate::use_columns). Columns are laid out from their
    /// `width`s, never from their content, and those without one share
    /// what is left. Give every column a width and a table too wide for
    /// its container scrolls sideways instead of squeezing.
    pub columns: Rc<[Column<T>]>,
    /// The data, one item per row.
    pub rows: Rc<[T]>,
    /// A stable identity for each row, which selection is tracked by.
    /// Defaults to the row's position in `rows`.
    #[prop_or_default]
    pub row_key: Option<RowFn<T, AttrValue>>,
    /// What to call a row in the names of its controls ("Select web-01").
    /// Defaults to its number.
    #[prop_or_default]
    pub row_label: Option<RowFn<T, AttrValue>>,
    #[prop_or_default]
    pub variant: TableVariant,
    /// Tighter rows, and compact density for controls inside them.
    #[prop_or_default]
    pub compact: bool,
    /// Caps the table's height as a CSS length; the rows scroll within it
    /// under a header that stays in view.
    #[prop_or(AttrValue::Static("24rem"))]
    pub max_height: AttrValue,
    /// The height of every row in CSS pixels. Defaults to the height of a
    /// table row with one line of text. Set it when cells hold something
    /// taller; if rows still turn out taller, the table measures one and
    /// uses that.
    #[prop_or_default]
    pub row_height: Option<f64>,
    /// Extra rows drawn above and below the visible ones.
    #[prop_or(8)]
    pub overscan: usize,
    /// How many rows the server draws, before the table can be measured.
    /// About what fits in `max_height` avoids a second render on load.
    #[prop_or(12)]
    pub initial_rows: usize,
    /// Fired with the positions (in sorted order) of the rows now drawn
    /// whenever the window moves. When its end reaches the number of rows,
    /// it is time to load more.
    #[prop_or_default]
    pub on_range: Callback<Range<usize>>,
    /// The sort order, when the caller owns it. Leave unset and the table
    /// keeps the order itself, starting from `default_sort`.
    #[prop_or_default]
    pub sort: Option<Vec<Sort>>,
    /// The initial sort order when the table keeps it.
    #[prop_or_default]
    pub default_sort: Vec<Sort>,
    /// Fired with the new sort order when a header is activated.
    #[prop_or_default]
    pub on_sort: Callback<Vec<Sort>>,
    /// Lets Shift-click add a column to the sort instead of replacing it.
    #[prop_or_default]
    pub multi_sort: bool,
    /// The selected rows' keys. Setting it adds a checkbox column; update it
    /// from `on_select`.
    #[prop_or_default]
    pub selected: Option<Selection>,
    /// Fired with the new selection when a row or the header box is ticked.
    #[prop_or_default]
    pub on_select: Callback<Selection>,
    /// Fired with the row when it is clicked anywhere but on a control or
    /// link inside it.
    #[prop_or_default]
    pub on_row_click: Option<Callback<T>>,
    /// The data is on its way: placeholder rows with none yet, dimmed rows
    /// during a refresh.
    #[prop_or_default]
    pub loading: bool,
    /// How many placeholder rows to show while loading with no rows.
    #[prop_or(5)]
    pub skeleton_rows: usize,
    /// Shown instead of rows when there are none.
    #[prop_or_default]
    pub empty: Html,
    /// The title of the default empty state, when `empty` is not set.
    #[prop_or(AttrValue::Static("Nothing to show"))]
    pub empty_label: AttrValue,
    /// Names the table for assistive technology.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Content under the table, outside its scrolling area: a row count.
    #[prop_or_default]
    pub footer: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A [`Table`](crate::Table) for more rows than a page should hold: it looks
/// and sorts the same, but only the rows in view (plus a few either side)
/// are in the document. Sorting orders all the rows; scrolling moves a
/// window over that order.
///
/// To make any row reachable without measuring the ones before it, rows
/// share one height, cells stay on one line (long text is cut short with an
/// ellipsis) and columns have fixed widths. Expandable details are not
/// available; open a row's details beside the table instead. Rows carry
/// their place in the whole table for assistive technology.
#[function_component]
pub fn VirtualTable<T: Clone + PartialEq + 'static>(props: &VirtualTableProps<T>) -> Html {
    let window = use_virtual_window(VirtualOptions {
        count: props.rows.len(),
        row_height: props.row_height.unwrap_or(default_row_height(props.compact, props.selected.is_some())),
        overscan: props.overscan,
        initial_rows: props.initial_rows,
        scroller: ".yi-table__scroll",
        row: ".yi-table__row",
    });
    use_range_callback(&window.range, &props.on_range);

    let table = TableProps {
        columns: props.columns.clone(),
        rows: props.rows.clone(),
        row_key: props.row_key.clone(),
        row_label: props.row_label.clone(),
        variant: props.variant,
        compact: props.compact,
        sticky_header: true,
        max_height: Some(props.max_height.clone()),
        fixed: true,
        sort: props.sort.clone(),
        default_sort: props.default_sort.clone(),
        on_sort: props.on_sort.clone(),
        multi_sort: props.multi_sort,
        selected: props.selected.clone(),
        on_select: props.on_select.clone(),
        detail: None,
        on_expand: Callback::noop(),
        on_row_click: props.on_row_click.clone(),
        loading: props.loading,
        skeleton_rows: props.skeleton_rows,
        empty: props.empty.clone(),
        empty_label: props.empty_label.clone(),
        label: props.label.clone(),
        footer: props.footer.clone(),
        class: classes!("yi-table--virtual", props.class.clone()),
        id: props.id.clone(),
        attrs: props.attrs.clone(),
    };
    use_table(&table, Some(window))
}
