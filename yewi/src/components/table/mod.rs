//! Tables: [`Table`] renders typed rows through [`Column`] definitions, with
//! sorting, selection and expandable rows; [`SimpleTable`] is the same frame
//! around markup you write yourself.
//!
//! A table draws every row it is given. Keep `rows` and `columns` in stable
//! `Rc`s (state, `use_memo`, [`use_columns`]) and a change redraws only the
//! rows it touches: sorting moves existing rows, and selecting one row
//! redraws one row. For tens of thousands of rows, paginate or use
//! [`VirtualTable`](crate::VirtualTable), which draws only the rows in view.

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use yew::{prelude::*, virtual_dom::Key};

mod column;
mod simple;
mod sort;

pub use column::*;
pub use simple::*;
pub use sort::*;

use crate::is_empty;
use crate::{
    Attrs, ButtonVariant, Checkbox, EmptyState, IconButton, Size, Skeleton, VirtualWindow, VisuallyHidden, attrs,
    components::server_only::use_server_only, icons,
};

/// The keys of the selected rows of a [`Table`].
pub type Selection = Rc<HashSet<AttrValue>>;

#[derive(Properties, PartialEq)]
pub struct TableProps<T: Clone + PartialEq + 'static> {
    /// The columns, in display order. Build them once with [`use_columns`].
    pub columns: Rc<[Column<T>]>,
    /// The data, one item per row.
    pub rows: Rc<[T]>,
    /// A stable identity for each row: an id, a name. Selection and expanded
    /// rows are tracked by it, so they survive sorting and reloads. Defaults
    /// to the row's position in `rows`.
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
    /// Keeps the header in view while the rows scroll: inside the table when
    /// `max_height` is set, otherwise against the page.
    #[prop_or_default]
    pub sticky_header: bool,
    /// Caps the table's height as a CSS length; the rows scroll within it.
    #[prop_or_default]
    pub max_height: Option<AttrValue>,
    /// Columns take the widths they are given instead of being sized from
    /// their content, which also makes a large table cheaper to lay out.
    #[prop_or_default]
    pub fixed: bool,
    /// The sort order, when the caller owns it. Leave unset and the table
    /// keeps the order itself, starting from `default_sort`.
    #[prop_or_default]
    pub sort: Option<Vec<Sort>>,
    /// The initial sort order when the table keeps it. A single [`Sort`]
    /// converts.
    #[prop_or_default]
    pub default_sort: Vec<Sort>,
    /// Fired with the new sort order when a header is activated. For
    /// server-side sorting, declare columns with `Column::sortable`, fetch
    /// from here and pass the result back as `rows`.
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
    /// The name the ticked rows' keys are submitted under in a native form,
    /// which is how a selection reaches the server on a page without wasm.
    #[prop_or_default]
    pub select_name: Option<AttrValue>,
    /// Renders a row's details. Setting it adds a column of disclosure
    /// buttons; the details appear in a full-width row underneath.
    #[prop_or_default]
    pub detail: Option<RowFn<T, Html>>,
    /// Fired with a row's key and whether its details are now shown, for
    /// loading them on demand.
    #[prop_or_default]
    pub on_expand: Callback<(AttrValue, bool)>,
    /// Fired with the row when it is clicked anywhere but on a control or
    /// link inside it. A convenience for mouse users: also give the row a
    /// link or button that does the same, so the keyboard can reach it.
    #[prop_or_default]
    pub on_row_click: Option<Callback<T>>,
    /// The data is on its way. With no rows yet, placeholder rows are shown;
    /// with rows, they stay visible and dim while the refresh runs.
    #[prop_or_default]
    pub loading: bool,
    /// How many placeholder rows to show while loading with no rows.
    #[prop_or(5)]
    pub skeleton_rows: usize,
    /// Shown instead of rows when there are none. Defaults to a small
    /// [`EmptyState`].
    #[prop_or_default]
    pub empty: Html,
    /// The title of the default empty state, when `empty` is not set.
    #[prop_or(AttrValue::Static("Nothing to show"))]
    pub empty_label: AttrValue,
    /// Names the table for assistive technology.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Content under the table, outside its scrolling area: a row count,
    /// pagination.
    #[prop_or_default]
    pub footer: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// What the row callbacks need from the latest render. They read it from a
/// shared cell instead of capturing it, so they keep one identity for the
/// life of the table and rows are not redrawn when the selection changes.
struct Latest<T> {
    rows: Rc<[T]>,
    row_key: Option<RowFn<T, AttrValue>>,
    selected: Option<Selection>,
    expanded: UseStateHandle<Selection>,
    on_select: Callback<Selection>,
    on_expand: Callback<(AttrValue, bool)>,
    on_row_click: Option<Callback<T>>,
}

impl<T> Latest<T> {
    fn key(&self, index: usize) -> Option<AttrValue> {
        self.rows.get(index).map(|row| row_id(self.row_key.as_ref(), row, index))
    }
}

fn row_id<T>(row_key: Option<&RowFn<T, AttrValue>>, row: &T, index: usize) -> AttrValue {
    row_key.map_or_else(|| index.to_string().into(), |key| key.call(row))
}

/// Compares by identity: the memoised sort is redone when the caller hands
/// over different data, not on every render.
struct SameRc<T: ?Sized>(Rc<T>);

impl<T: ?Sized> PartialEq for SameRc<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// A table of typed rows. Columns say how a row renders and sorts; the table
/// adds click-to-sort headers, and optionally row selection, expandable
/// details, and loading and empty states.
///
/// Sorting happens in the table for columns with an ordering (`sort_by*`,
/// `sort_text`, `sort_number`) and is left to the caller for columns marked
/// only `sortable`.
#[function_component]
pub fn Table<T: Clone + PartialEq + 'static>(props: &TableProps<T>) -> Html {
    use_table(props, None)
}

/// Renders a table. With a `window` only that slice of the sorted rows is
/// drawn, between two spacer rows standing in for the rest: this is how
/// [`VirtualTable`](crate::VirtualTable) shares everything else with
/// [`Table`].
#[hook]
pub(crate) fn use_table<T: Clone + PartialEq + 'static>(props: &TableProps<T>, window: Option<VirtualWindow>) -> Html {
    let own_sort = use_state(|| props.default_sort.clone());
    let sort = props.sort.clone().unwrap_or_else(|| (*own_sort).clone());
    let expanded = use_state(Selection::default);

    let order = use_memo(
        (SameRc(props.rows.clone()), SameRc(props.columns.clone()), sort.clone()),
        |(rows, columns, sort)| sort_order(&rows.0, &columns.0, sort),
    );

    let latest = use_mut_ref(|| None::<Latest<T>>);
    *latest.borrow_mut() = Some(Latest {
        rows: props.rows.clone(),
        row_key: props.row_key.clone(),
        selected: props.selected.clone(),
        expanded: expanded.clone(),
        on_select: props.on_select.clone(),
        on_expand: props.on_expand.clone(),
        on_row_click: props.on_row_click.clone(),
    });

    let on_select_row = {
        let latest = latest.clone();
        use_callback((), move |(index, checked): (usize, bool), ()| {
            with_latest(&latest, |latest| {
                let (Some(key), Some(selected)) = (latest.key(index), &latest.selected) else { return };
                let mut next = HashSet::clone(selected);
                if checked {
                    next.insert(key);
                } else {
                    next.remove(&key);
                }
                latest.on_select.emit(Rc::new(next));
            });
        })
    };
    let on_expand_row = {
        let latest = latest.clone();
        use_callback((), move |index: usize, ()| {
            with_latest(&latest, |latest| {
                let Some(key) = latest.key(index) else { return };
                let mut next = HashSet::clone(&latest.expanded);
                let open = next.insert(key.clone());
                if !open {
                    next.remove(&key);
                }
                latest.expanded.set(Rc::new(next));
                latest.on_expand.emit((key, open));
            });
        })
    };
    let on_click_row = {
        let latest = latest.clone();
        use_callback((), move |index: usize, ()| {
            with_latest(&latest, |latest| {
                if let (Some(row), Some(on_row_click)) = (latest.rows.get(index), &latest.on_row_click) {
                    on_row_click.emit(row.clone());
                }
            });
        })
    };

    let selectable = props.selected.is_some();
    let server_only = use_server_only();
    let expandable = props.detail.is_some();
    let keyed = selectable || expandable;
    let span = props.columns.len() + usize::from(selectable) + usize::from(expandable);
    let key_of = |index: usize| row_id(props.row_key.as_ref(), &props.rows[index], index);

    // Counted once per selection, not on every render: a virtualised table
    // renders on scroll.
    let selected_count = use_memo(
        (SameRc(props.rows.clone()), props.selected.clone().map(SameRc), props.row_key.clone()),
        |(rows, selected, row_key)| match selected {
            Some(selected) if !selected.0.is_empty() => rows
                .0
                .iter()
                .enumerate()
                .filter(|&(index, row)| selected.0.contains(&row_id(row_key.as_ref(), row, index)))
                .count(),
            _ => 0,
        },
    );

    let select_all = props.selected.as_ref().map(|_| {
        let count = *selected_count;
        let all = count > 0 && count == props.rows.len();
        let on_change = {
            let (latest, on_select) = (latest.clone(), props.on_select.clone());
            Callback::from(move |checked: bool| {
                with_latest(&latest, |latest| {
                    let mut next = latest.selected.as_deref().cloned().unwrap_or_default();
                    for index in 0..latest.rows.len() {
                        let Some(key) = latest.key(index) else { continue };
                        if checked {
                            next.insert(key);
                        } else {
                            next.remove(&key);
                        }
                    }
                    on_select.emit(Rc::new(next));
                });
            })
        };
        html! {
            <th
                class="yi-table__header yi-table__control"
                scope="col"
                data-yi-select-all={server_only.then_some("")}
            >
                <Checkbox
                    label="Select all rows"
                    checked={all}
                    indeterminate={count > 0 && !all}
                    disabled={props.rows.is_empty()}
                    {on_change}
                />
            </th>
        }
    });

    let headers = props.columns.iter().map(|column| {
        let class = classes!(column.align.class(), column.class.clone());
        if !column.sortable {
            return html! {
                <th key={column.key.to_string()} class={classes!("yi-table__header", class)} scope="col">
                    { column.header.clone() }
                </th>
            };
        }
        let position = sort.iter().position(|sort| sort.column == column.key);
        let on_sort = {
            let (sort, key, multi) = (sort.clone(), column.key.clone(), props.multi_sort);
            let (own_sort, on_sort) = (own_sort.clone(), props.on_sort.clone());
            Callback::from(move |click: SortClick| {
                let next = next_sort(&sort, &key, multi && click.extend);
                own_sort.set(next.clone());
                on_sort.emit(next);
            })
        };
        html! {
            <SortHeader
                key={column.key.to_string()}
                {class}
                align={column.align}
                // Only the leading column carries `aria-sort`; the rest show
                // their direction and place.
                direction={position.map(|i| sort[i].direction)}
                priority={position.filter(|_| sort.len() > 1).map(|i| i + 1)}
                attrs={position.filter(|&i| i > 0).map(|_| attrs! { "aria-sort" => "none" }).unwrap_or_default()}
                {on_sort}
            >
                { column.header.clone() }
            </SortHeader>
        }
    });

    let body = if props.rows.is_empty() && props.loading {
        (0..props.skeleton_rows)
            .map(|_| {
                html! {
                    <tr class="yi-table__row yi-table__row--placeholder">
                        { for (0..span).map(|_| html! { <td class="yi-table__cell"><Skeleton /></td> }) }
                    </tr>
                }
            })
            .collect::<Html>()
    } else if props.rows.is_empty() {
        html! {
            <tr class="yi-table__empty">
                <td colspan={span.to_string()}>
                    if !is_empty(&props.empty) {
                        { props.empty.clone() }
                    } else {
                        <EmptyState title={props.empty_label.clone()} size={Size::Small} />
                    }
                </td>
            </tr>
        }
    } else {
        let range = match &window {
            Some(window) => window.range.start.min(order.len())..window.range.end.min(order.len()),
            None => 0..order.len(),
        };
        let spacer = |key: &'static str, rows: usize| {
            let height = window.as_ref().map_or(0.0, |window| window.row_height) * rows as f64;
            html! {
                <tr {key} class="yi-table__spacer" aria-hidden="true" style={format!("height:{height}px")}>
                    <td colspan={span.to_string()} />
                </tr>
            }
        };
        let before = window.is_some().then(|| spacer("\u{0}before", range.start));
        let after = (window.is_some() && range.end < order.len()).then(|| spacer("\u{0}after", order.len() - range.end));
        let rows = order[range.clone()]
            .iter()
            .enumerate()
            .map(|(at, &index)| {
                let id = keyed.then(|| key_of(index));
                let key = match (&props.row_key, &id) {
                    (Some(_), Some(id)) => Key::from(id.to_string()),
                    (Some(row_key), None) => Key::from(row_key.call(&props.rows[index]).to_string()),
                    (None, _) => Key::from(index),
                };
                html! {
                    <Row<T>
                        {key}
                        rows={props.rows.clone()}
                        {index}
                        position={window.is_some().then_some(range.start + at)}
                        columns={props.columns.clone()}
                        selected={props.selected.as_ref().zip(id.as_ref()).map(|(set, id)| set.contains(id))}
                        expanded={id.as_ref().filter(|_| expandable).map(|id| expanded.contains(id))}
                        select={props.select_name.clone().zip(id.clone())}
                        detail={props.detail.clone()}
                        row_label={props.row_label.clone()}
                        clickable={props.on_row_click.is_some()}
                        {span}
                        on_select={on_select_row.clone()}
                        on_expand={on_expand_row.clone()}
                        on_click={on_click_row.clone()}
                    />
                }
            });
        before.into_iter().chain(rows).chain(after).collect::<Html>()
    };

    let has_widths = props.columns.iter().any(|column| column.width.is_some());

    html! {
        <SimpleTable
            id={props.id.clone()}
            class={classes!(props.loading.then_some("yi-table--loading"), props.class.clone())}
            attrs={props.attrs.clone()}
            variant={props.variant}
            compact={props.compact}
            sticky_header={props.sticky_header}
            max_height={props.max_height.clone()}
            fixed={props.fixed}
            label={props.label.clone()}
            busy={props.loading}
            footer={props.footer.clone()}
            row_count={window.is_some().then(|| props.rows.len() + 1)}
        >
            if has_widths {
                <colgroup>
                    if expandable { <col class="yi-table__control-col" /> }
                    if selectable { <col class="yi-table__control-col" /> }
                    { for props.columns.iter().map(|column| html! {
                        <col style={column.width.as_ref().map(|width| format!("inline-size:{width}"))} />
                    }) }
                </colgroup>
            }
            <thead>
                <tr aria-rowindex={window.is_some().then_some("1")}>
                    if expandable {
                        <th class="yi-table__header yi-table__control" scope="col">
                            <VisuallyHidden>{ "Details" }</VisuallyHidden>
                        </th>
                    }
                    { select_all }
                    { for headers }
                </tr>
            </thead>
            <tbody
                ref={window.as_ref().map(|window| window.anchor.clone()).unwrap_or_default()}
                style={window.as_ref().map(|window| format!("--yi-table-row-height:{}px", window.min_row_height))}
            >
                { body }
            </tbody>
        </SimpleTable>
    }
}

fn with_latest<T>(latest: &RefCell<Option<Latest<T>>>, f: impl FnOnce(&Latest<T>)) {
    if let Some(latest) = latest.borrow().as_ref() {
        f(latest);
    }
}

#[derive(Properties)]
struct RowProps<T: Clone + PartialEq + 'static> {
    rows: Rc<[T]>,
    index: usize,
    /// The row's place in the sorted order, when not every row is drawn.
    position: Option<usize>,
    columns: Rc<[Column<T>]>,
    selected: Option<bool>,
    expanded: Option<bool>,
    /// The name and value the row's checkbox submits.
    select: Option<(AttrValue, AttrValue)>,
    detail: Option<RowFn<T, Html>>,
    row_label: Option<RowFn<T, AttrValue>>,
    clickable: bool,
    span: usize,
    on_select: Callback<(usize, bool)>,
    on_expand: Callback<usize>,
    on_click: Callback<usize>,
}

impl<T: Clone + PartialEq + 'static> PartialEq for RowProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.position == other.position
            && self.selected == other.selected
            && self.expanded == other.expanded
            && self.select == other.select
            && self.clickable == other.clickable
            && self.span == other.span
            && (Rc::ptr_eq(&self.columns, &other.columns) || self.columns == other.columns)
            && (Rc::ptr_eq(&self.rows, &other.rows) || self.rows.get(self.index) == other.rows.get(other.index))
            && self.detail == other.detail
            && self.row_label == other.row_label
            && self.on_select == other.on_select
            && self.on_expand == other.on_expand
            && self.on_click == other.on_click
    }
}

/// One data row and, when open, its detail row. A component of its own so an
/// unchanged row is skipped when the table re-renders.
#[function_component]
fn Row<T: Clone + PartialEq + 'static>(props: &RowProps<T>) -> Html {
    let Some(row) = props.rows.get(props.index) else { return Html::default() };
    let index = props.index;
    let name = || {
        props.row_label.as_ref().map_or_else(|| AttrValue::from(format!("row {}", index + 1)), |label| label.call(row))
    };
    let expanded = props.expanded.unwrap_or_default();
    // Without a client the details are in the page, for `BEHAVIOURS` to show.
    let server_only = use_server_only();
    let toggle = if server_only {
        attrs! { "aria-expanded" => if expanded { "true" } else { "false" }, "data-yi-row-toggle" => "" }
    } else {
        attrs! { "aria-expanded" => if expanded { "true" } else { "false" } }
    };

    let onclick = props.clickable.then(|| {
        let on_click = props.on_click.clone();
        Callback::from(move |event: MouseEvent| {
            if !from_control(&event) {
                on_click.emit(index);
            }
        })
    });
    let class = classes!(
        "yi-table__row",
        props.selected.unwrap_or_default().then_some("yi-table__row--selected"),
        props.clickable.then_some("yi-table__row--clickable"),
        expanded.then_some("yi-table__row--expanded"),
    );

    html! {
        <>
            // The header is row 1.
            <tr key="row" {class} {onclick} aria-rowindex={props.position.map(|at| (at + 2).to_string())}>
                if props.expanded.is_some() {
                    <td class="yi-table__cell yi-table__control">
                        <span class="yi-table__toggle-box">
                            <IconButton
                                class="yi-table__toggle"
                                icon={icons::CHEVRON_RIGHT}
                                label={format!("Details for {}", name())}
                                variant={ButtonVariant::Subtle}
                                size={Size::Small}
                                attrs={toggle}
                                on_click={props.on_expand.reform(move |_| index)}
                            />
                        </span>
                    </td>
                }
                if let Some(selected) = props.selected {
                    <td class="yi-table__cell yi-table__control">
                        <Checkbox
                            label={format!("Select {}", name())}
                            checked={selected}
                            name={props.select.as_ref().map(|(name, _)| name.clone())}
                            value={props.select.as_ref().map(|(_, value)| value.clone())}
                            on_change={props.on_select.reform(move |checked| (index, checked))}
                        />
                    </td>
                }
                { for props.columns.iter().map(|column| {
                    let class = classes!(
                        "yi-table__cell",
                        column.align.class(),
                        column.wrap.then_some("yi-table__cell--wrap"),
                        column.class.clone(),
                    );
                    let content = column.cell.call(row);
                    if column.row_header {
                        html! { <th key={column.key.to_string()} {class} scope="row">{ content }</th> }
                    } else {
                        html! { <td key={column.key.to_string()} {class}>{ content }</td> }
                    }
                }) }
            </tr>
            if let (true, Some(detail)) = (expanded || (server_only && props.expanded.is_some()), &props.detail) {
                <tr key="detail" class="yi-table__detail" hidden={!expanded}>
                    <td colspan={props.span.to_string()}>{ detail.call(row) }</td>
                </tr>
            }
        </>
    }
}

/// Whether a click landed on something inside the row that handles clicks
/// itself, so the row should not also act on it.
fn from_control(event: &MouseEvent) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        event
            .target_dyn_into::<web_sys::Element>()
            .and_then(|target| target.closest("a, button, input, label, select, textarea, summary").ok().flatten())
            .is_some()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = event;
        false
    }
}

#[cfg(test)]
mod tests;
