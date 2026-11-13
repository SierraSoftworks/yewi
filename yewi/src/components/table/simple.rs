//! The table frame and sortable header shared by [`Table`](super::Table) and
//! hand-written tables.

use yew::prelude::*;

use super::{CellAlign, SortClick, SortDirection};
use crate::is_empty;
use crate::{Attrs, Icon, icons};

/// How rows are told apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TableVariant {
    /// One rule under the header, rows separated by whitespace alone.
    #[default]
    Open,
    /// A hairline between rows and a stronger rule under the header, like a
    /// printed table. For dense data that is read across.
    Ruled,
    /// A tonal header and alternating row fills, for wide tables where the
    /// eye has far to travel.
    Zebra,
}

impl TableVariant {
    pub const ALL: [TableVariant; 3] = [TableVariant::Open, TableVariant::Ruled, TableVariant::Zebra];

    const fn as_str(self) -> &'static str {
        match self {
            TableVariant::Open => "open",
            TableVariant::Ruled => "ruled",
            TableVariant::Zebra => "zebra",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SimpleTableProps {
    /// The table's own markup: `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>`.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub variant: TableVariant,
    /// Tighter rows, and compact density for controls inside them.
    #[prop_or_default]
    pub compact: bool,
    /// Keeps the header in view while the rows scroll: inside the table when
    /// `max_height` is set, otherwise against the page. Against the page a
    /// table too wide for its container overflows instead of scrolling.
    #[prop_or_default]
    pub sticky_header: bool,
    /// Caps the table's height as a CSS length; the rows scroll within it.
    #[prop_or_default]
    pub max_height: Option<AttrValue>,
    /// Columns take the widths they are given (or equal shares) instead of
    /// being sized from their content. Long cells can then truncate, and the
    /// browser lays out a large table without measuring every row.
    #[prop_or_default]
    pub fixed: bool,
    /// Lets every cell wrap onto several lines, for tables of prose. By
    /// default cells stay on one line and the table scrolls sideways; a
    /// single cell opts in with `yi-table__cell--wrap`.
    #[prop_or_default]
    pub wrap: bool,
    /// Names the table for assistive technology.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// The rows are loading or being refreshed.
    #[prop_or_default]
    pub busy: bool,
    /// Content under the table, outside its scrolling area: a row count,
    /// pagination.
    #[prop_or_default]
    pub footer: Html,
    /// How many rows the table has in all, header included, when only some
    /// of them are in the document (`aria-rowcount`). Give each row that is
    /// present its `aria-rowindex`.
    #[prop_or_default]
    pub row_count: Option<usize>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The table frame around markup you write yourself: the look, density,
/// scrolling and sticky header of [`Table`](super::Table) without its data
/// model. Use it for tables whose rows are not uniform (group headings, cells
/// that span) and for rendered documents.
///
/// Rows take `yi-table__row`, a group heading row `yi-table__group`,
/// right-aligned cells `yi-table__cell--end` and cells that may wrap
/// `yi-table__cell--wrap`.
#[function_component]
pub fn SimpleTable(props: &SimpleTableProps) -> Html {
    let class = classes!(
        "yi-table",
        format!("yi-table--{}", props.variant.as_str()),
        props.compact.then_some("yi-table--compact"),
        props.sticky_header.then_some("yi-table--sticky"),
        props.max_height.is_some().then_some("yi-table--scroll"),
        props.fixed.then_some("yi-table--fixed"),
        props.wrap.then_some("yi-table--wrap"),
        props.class.clone(),
    );
    let style = props.max_height.as_ref().map(|height| format!("--yi-table-max-height:{height}"));
    // Against the page nothing scrolls inside the table, so there is nothing
    // for the keyboard to reach.
    let scrolls = !props.sticky_header || props.max_height.is_some();

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} {style} data-yi-density={props.compact.then_some("compact")}>
            <div class="yi-table__scroll" tabindex={scrolls.then_some("0")}>
                <table
                    class="yi-table__grid"
                    aria-label={props.label.clone()}
                    aria-busy={props.busy.then_some("true")}
                    aria-rowcount={props.row_count.map(|count| count.to_string())}
                >
                    { props.children.clone() }
                </table>
            </div>
            if !is_empty(&props.footer) {
                <div class="yi-table__footer">{ props.footer.clone() }</div>
            }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct SortHeaderProps {
    /// The column's title.
    #[prop_or_default]
    pub children: Html,
    /// How the table is sorted by this column, if it is.
    #[prop_or_default]
    pub direction: Option<SortDirection>,
    /// The column's place in a sort over several columns, from 1. Shown
    /// beside the arrow.
    #[prop_or_default]
    pub priority: Option<usize>,
    /// Fired when the header is activated, with the direction to sort in
    /// next and whether Shift was held.
    #[prop_or_default]
    pub on_sort: Callback<SortClick>,
    #[prop_or_default]
    pub align: CellAlign,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A column header that sorts when activated. [`Table`](super::Table) renders
/// these itself; use it directly in a [`SimpleTable`]. It is a real button
/// inside the `<th>`, and the `<th>` reports the direction through
/// `aria-sort`.
#[function_component]
pub fn SortHeader(props: &SortHeaderProps) -> Html {
    let direction = props.direction;
    let onclick = props.on_sort.reform(move |event: MouseEvent| SortClick {
        direction: direction.map_or(SortDirection::Ascending, SortDirection::reversed),
        extend: event.shift_key(),
    });
    let icon = match direction {
        None => icons::CHEVRONS_UP_DOWN,
        Some(SortDirection::Ascending) => icons::ARROW_UP,
        Some(SortDirection::Descending) => icons::ARROW_DOWN,
    };
    let class = classes!(
        "yi-table__header",
        "yi-table__header--sortable",
        direction.map(|_| "yi-table__header--sorted"),
        props.align.class(),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <th id={props.id.clone()} {class} scope="col" aria-sort={direction.map(SortDirection::as_str)}>
            <button class="yi-table__sort" type="button" {onclick}>
                <span class="yi-table__sort-label">{ props.children.clone() }</span>
                <Icon {icon} class="yi-table__sort-icon" />
                if let Some(priority) = props.priority {
                    <span class="yi-table__sort-priority" aria-hidden="true">{ priority }</span>
                }
            </button>
        </th>
    })
}
