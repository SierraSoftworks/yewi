//! Typed column definitions for [`Table`](super::Table).

use std::{cmp::Ordering, rc::Rc};

use yew::prelude::*;

use super::{Sort, natural_cmp};

/// Where a column's header and cells sit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CellAlign {
    #[default]
    Start,
    Center,
    /// For numbers, so digits line up.
    End,
}

impl CellAlign {
    pub(super) const fn class(self) -> Option<&'static str> {
        match self {
            CellAlign::Start => None,
            CellAlign::Center => Some("yi-table__cell--center"),
            CellAlign::End => Some("yi-table__cell--end"),
        }
    }
}

/// A function of one row, compared by identity so props holding it stay
/// cheap to compare. Build it once (in `use_memo` or outside the component)
/// rather than on every render.
pub struct RowFn<T, O>(Rc<dyn Fn(&T) -> O>);

impl<T, O> RowFn<T, O> {
    pub fn new(f: impl Fn(&T) -> O + 'static) -> Self {
        Self(Rc::new(f))
    }

    pub fn call(&self, row: &T) -> O {
        (self.0)(row)
    }
}

impl<T, O> Clone for RowFn<T, O> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, O> PartialEq for RowFn<T, O> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

type Compare<T> = Rc<dyn Fn(&T, &T) -> Ordering>;

/// One column of a [`Table`](super::Table): its header, how a row renders in
/// it and, optionally, how rows are ordered by it.
///
/// ```ignore
/// Column::new("memory", "Memory", |host: &Host| format!("{} GB", host.memory))
///     .sort_by_key(|host| host.memory)
///     .numeric()
/// ```
pub struct Column<T> {
    pub(super) key: AttrValue,
    pub(super) header: Html,
    pub(super) cell: RowFn<T, Html>,
    pub(super) compare: Option<Compare<T>>,
    pub(super) sortable: bool,
    pub(super) align: CellAlign,
    pub(super) width: Option<AttrValue>,
    pub(super) row_header: bool,
    pub(super) wrap: bool,
    pub(super) class: Classes,
}

impl<T> Column<T> {
    /// A column identified by `key` (the name used in [`Sort`]), titled
    /// `header`, whose cells are whatever `cell` returns for a row: text or
    /// `html!`.
    pub fn new<R: Into<Html>>(
        key: impl Into<AttrValue>,
        header: impl Into<Html>,
        cell: impl Fn(&T) -> R + 'static,
    ) -> Self {
        Self {
            key: key.into(),
            header: header.into(),
            cell: RowFn::new(move |row| cell(row).into()),
            compare: None,
            sortable: false,
            align: CellAlign::Start,
            width: None,
            row_header: false,
            wrap: false,
            class: Classes::new(),
        }
    }

    /// The key [`Sort`] refers to this column by.
    pub fn key(&self) -> &AttrValue {
        &self.key
    }

    /// Whether the header can be activated to sort.
    pub fn is_sortable(&self) -> bool {
        self.sortable
    }

    /// Sortable with a comparison of two rows, for anything the other
    /// `sort_*` methods do not cover. It borrows, so it is the cheapest
    /// option on large data.
    pub fn sort_by(mut self, compare: impl Fn(&T, &T) -> Ordering + 'static) -> Self {
        self.compare = Some(Rc::new(compare));
        self.sortable = true;
        self
    }

    /// Sortable by a value taken from each row: a number, a timestamp, or an
    /// enum whose `Ord` is the order statuses should appear in.
    pub fn sort_by_key<K: Ord>(self, key: impl Fn(&T) -> K + 'static) -> Self {
        self.sort_by(move |a, b| key(a).cmp(&key(b)))
    }

    /// Sortable as text: case-insensitive, with digit runs compared by value
    /// (see [`natural_cmp`]).
    pub fn sort_text(self, text: impl for<'a> Fn(&'a T) -> &'a str + 'static) -> Self {
        self.sort_by(move |a, b| natural_cmp(text(a), text(b)))
    }

    /// Sortable by a floating-point value. `NaN` sorts last.
    pub fn sort_number(self, number: impl Fn(&T) -> f64 + 'static) -> Self {
        self.sort_by(move |a, b| number(a).total_cmp(&number(b)))
    }

    /// Sortable, but the table leaves the rows in the order given: the
    /// header reports the request through `on_sort` and whoever owns the
    /// data (usually a server) returns it sorted.
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    pub fn align(mut self, align: CellAlign) -> Self {
        self.align = align;
        self
    }

    /// Right-aligns the column, for numbers. Figures are tabular everywhere
    /// in a table, so digits line up.
    pub fn numeric(self) -> Self {
        self.align(CellAlign::End)
    }

    /// The column's width as a CSS length or percentage. Columns without
    /// one share what is left.
    pub fn width(mut self, width: impl Into<AttrValue>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Marks the cells as the header of their row (`<th scope="row">`): the
    /// column that names the thing each row is about.
    pub fn row_header(mut self) -> Self {
        self.row_header = true;
        self
    }

    /// Lets the cells wrap onto several lines, for prose such as titles
    /// and descriptions. Other cells stay on one line and the table scrolls
    /// sideways when it runs out of room.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// Extra classes for the column's header and cells.
    pub fn class(mut self, class: impl Into<Classes>) -> Self {
        self.class = class.into();
        self
    }

    /// Compares two rows by this column, ascending. `None` when the column
    /// has no ordering of its own.
    pub fn compare(&self, a: &T, b: &T) -> Option<Ordering> {
        self.compare.as_ref().map(|compare| compare(a, b))
    }
}

impl<T> Clone for Column<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            header: self.header.clone(),
            cell: self.cell.clone(),
            compare: self.compare.clone(),
            sortable: self.sortable,
            align: self.align,
            width: self.width.clone(),
            row_header: self.row_header,
            wrap: self.wrap,
            class: self.class.clone(),
        }
    }
}

impl<T> PartialEq for Column<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.cell == other.cell
            && match (&self.compare, &other.compare) {
                (None, None) => true,
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                _ => false,
            }
            && self.sortable == other.sortable
            && self.align == other.align
            && self.width == other.width
            && self.row_header == other.row_header
            && self.wrap == other.wrap
            && self.class == other.class
            && self.header == other.header
    }
}

/// The order to show `rows` in: indices into `rows`, sorted by each entry of
/// `sort` in turn. The sort is stable, so rows that compare equal keep the
/// order they were given in, and entries naming a column without an ordering
/// are skipped.
///
/// [`Table`](super::Table) calls this itself; it is public for components
/// that draw rows their own way (a virtualised table).
pub fn sort_order<T>(rows: &[T], columns: &[Column<T>], sort: &[Sort]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..rows.len()).collect();
    let by: Vec<_> = sort
        .iter()
        .filter_map(|sort| {
            let column = columns.iter().find(|column| column.key == sort.column)?;
            Some((column.compare.as_ref()?, sort.direction))
        })
        .collect();
    if !by.is_empty() {
        order.sort_by(|&a, &b| {
            by.iter()
                .map(|(compare, direction)| direction.apply(compare(&rows[a], &rows[b])))
                .find(|ordering| ordering.is_ne())
                .unwrap_or(Ordering::Equal)
        });
    }
    order
}

/// Builds a table's columns once and keeps them for the life of the
/// component, so rows are not redrawn because their column definitions were
/// rebuilt. For columns that depend on state, use `use_memo` with that state
/// as its dependency instead.
#[hook]
pub fn use_columns<T: 'static>(columns: impl FnOnce() -> Vec<Column<T>>) -> Rc<[Column<T>]> {
    (*use_memo((), |()| Rc::<[Column<T>]>::from(columns()))).clone()
}
