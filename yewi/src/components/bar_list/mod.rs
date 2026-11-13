//! [`BarList`]: a ranked breakdown, each row a label, a figure and a bar
//! showing its size against the largest.

use std::rc::Rc;

use yew::prelude::*;

use crate::is_empty;
use crate::{Attrs, Icon, IconData, Pagination, Size, Tone};

/// One row of a [`BarList`].
#[derive(Debug, Clone, PartialEq)]
pub struct BarItem {
    /// Identifies the row in `selected`, `on_select` and the row action.
    pub value: AttrValue,
    /// What the row is, as shown.
    pub label: AttrValue,
    /// The figure the bar is drawn from.
    pub amount: f64,
    /// The figure as shown. Defaults to `amount` in compact form (`1.2k`).
    pub display: Option<AttrValue>,
    /// An icon before the label.
    pub icon: Option<IconData>,
    /// A short piece of text before the label, such as a flag or a rank.
    pub glyph: Option<AttrValue>,
    /// Makes the row a link.
    pub href: Option<AttrValue>,
    /// Colours this row's bar to carry a status.
    pub tone: Option<Tone>,
    /// Marks a row that stands for missing values ("Unknown", "Direct").
    /// It is drawn quieter and gets no row action.
    pub muted: bool,
}

impl BarItem {
    pub fn new(value: impl Into<AttrValue>, label: impl Into<AttrValue>, amount: f64) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            amount,
            display: None,
            icon: None,
            glyph: None,
            href: None,
            tone: None,
            muted: false,
        }
    }

    /// A row whose label is also its `value`.
    pub fn labelled(label: impl Into<AttrValue>, amount: f64) -> Self {
        let label = label.into();
        Self::new(label.clone(), label, amount)
    }

    pub fn display(mut self, display: impl Into<AttrValue>) -> Self {
        self.display = Some(display.into());
        self
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn glyph(mut self, glyph: impl Into<AttrValue>) -> Self {
        self.glyph = Some(glyph.into());
        self
    }

    pub fn href(mut self, href: impl Into<AttrValue>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = Some(tone);
        self
    }

    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}

/// Where a row's bar is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BarListVariant {
    /// A soft bar behind the label. The densest form, for dashboards.
    #[default]
    Behind,
    /// A thin bar on a track under the label, for long labels or where the
    /// figures need more air.
    Under,
}

impl BarListVariant {
    pub const ALL: [BarListVariant; 2] = [BarListVariant::Behind, BarListVariant::Under];

    const fn as_str(self) -> &'static str {
        match self {
            BarListVariant::Behind => "behind",
            BarListVariant::Under => "under",
        }
    }
}

/// The order rows are shown in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BarListOrder {
    /// Largest first.
    #[default]
    Descending,
    /// As given, for data with an order of its own such as weekdays.
    Given,
}

/// What happens to rows beyond the first few.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BarListOverflow {
    /// Every row is shown.
    #[default]
    All,
    /// The first rows are shown, with a "Show all" control for the rest.
    Expand(usize),
    /// Rows are shown this many at a time, with [`Pagination`] underneath.
    Pages(usize),
}

/// A control at the end of every row, such as "Open" or "Manage".
#[derive(Debug, Clone, PartialEq)]
pub struct BarListAction {
    pub icon: IconData,
    /// What it does. Each row's control is named "{label}: {row label}".
    pub label: AttrValue,
    /// Fired with the row's `value`.
    pub on_click: Callback<AttrValue>,
}

impl BarListAction {
    pub fn new(icon: IconData, label: impl Into<AttrValue>, on_click: Callback<AttrValue>) -> Self {
        Self { icon, label: label.into(), on_click }
    }
}

/// A number in compact form: `950`, `1.2k`, `3.4M`, `1B`.
pub fn compact_number(value: f64) -> String {
    const UNITS: [(f64, &str); 4] = [(1e9, "B"), (1e6, "M"), (1e3, "k"), (1.0, "")];
    let size = value.abs();
    // 999,950 rounds to 1000.0k, so step up a unit just before it.
    let (scale, suffix) = UNITS.into_iter().find(|(scale, _)| size >= scale * 0.99995).unwrap_or((1.0, ""));
    let text = format!("{:.1}", value / scale);
    format!("{}{suffix}", text.strip_suffix(".0").unwrap_or(&text))
}

/// A share of the total, as a whole percentage.
fn share(amount: f64, total: f64) -> String {
    let percent = if total > 0.0 && amount.is_finite() { amount.max(0.0) / total * 100.0 } else { 0.0 };
    if percent > 0.0 && percent < 0.5 { "<1%".into() } else { format!("{percent:.0}%") }
}

/// The rows in display order, and the figures bars and shares are drawn from.
#[derive(PartialEq)]
struct Ranking {
    /// Indices into `items`.
    order: Vec<usize>,
    max: f64,
    total: f64,
}

fn rank(items: &[BarItem], order: BarListOrder) -> Ranking {
    let mut indices: Vec<usize> = (0..items.len()).collect();
    if order == BarListOrder::Descending {
        indices.sort_by(|a, b| items[*b].amount.total_cmp(&items[*a].amount));
    }
    let amounts = || items.iter().map(|item| item.amount.max(0.0)).filter(|amount| amount.is_finite());
    Ranking { order: indices, max: amounts().fold(0.0, f64::max), total: amounts().sum() }
}

#[derive(Properties, PartialEq)]
pub struct BarListProps {
    /// The rows. Keep the same `Rc` between renders while the data is
    /// unchanged and the list is not ranked again.
    pub items: Rc<[BarItem]>,
    /// Accessible name of the list, such as "Top pages".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// A heading over the label column, such as "Page".
    #[prop_or_default]
    pub heading: Option<AttrValue>,
    /// A heading over the figures, such as "Visitors".
    #[prop_or_default]
    pub value_heading: Option<AttrValue>,
    #[prop_or_default]
    pub variant: BarListVariant,
    /// Row height.
    #[prop_or_default]
    pub size: Size,
    /// Colours every bar to carry a status. Bars use the accent otherwise.
    #[prop_or_default]
    pub tone: Tone,
    #[prop_or_default]
    pub order: BarListOrder,
    /// What to do with long lists.
    #[prop_or_default]
    pub overflow: BarListOverflow,
    /// Shows each row's share of the total as a percentage.
    #[prop_or_default]
    pub show_share: bool,
    /// The amount a full-width bar stands for. Defaults to the largest row.
    #[prop_or_default]
    pub max: Option<f64>,
    /// The amount shares are taken of. Defaults to the sum of the rows; set
    /// it when the rows are only the top of a longer list.
    #[prop_or_default]
    pub total: Option<f64>,
    /// The `value` of the chosen row. Only used with `on_select`.
    #[prop_or_default]
    pub selected: Option<AttrValue>,
    /// Makes each row a toggle button, for rows that filter the view. Fired
    /// with the row's `value`; clear the filter when it is the selected one.
    #[prop_or_default]
    pub on_select: Option<Callback<AttrValue>>,
    /// A control at the end of every row.
    #[prop_or_default]
    pub action: Option<BarListAction>,
    /// Shown instead of the list when there are no rows.
    #[prop_or_default]
    pub empty: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Rows above which off-screen rows skip layout and paint.
const LONG: usize = 40;

/// A ranked list of labelled figures with proportional bars.
///
/// Rows are plain text by default, links when an item has `href`, and toggle
/// buttons with `on_select`. One click handler serves the whole list, so a
/// long list costs no more to keep interactive than a short one.
#[function_component]
pub fn BarList(props: &BarListProps) -> Html {
    let expanded = use_state(|| false);
    let page = use_state(|| 1_u32);
    let ranking = use_memo((props.items.clone(), props.order), |(items, order)| rank(items, *order));

    let count = ranking.order.len();
    let (skip, take, pages) = match props.overflow {
        BarListOverflow::Expand(limit) if !*expanded => (0, limit.max(1), 1),
        BarListOverflow::Pages(per_page) => {
            let per_page = per_page.max(1);
            let pages = count.div_ceil(per_page).max(1);
            let current = (*page as usize).clamp(1, pages);
            ((current - 1) * per_page, per_page, pages)
        }
        _ => (0, count, 1),
    };
    let shown = &ranking.order[skip.min(count)..(skip + take).min(count)];

    let max = props.max.unwrap_or(ranking.max);
    let total = props.total.unwrap_or(ranking.total);
    let selected = props.on_select.as_ref().and(props.selected.as_ref());

    let rows = shown.iter().map(|&index| {
        let item = &props.items[index];
        let fill = if max > 0.0 && item.amount.is_finite() { (item.amount / max * 100.0).clamp(0.0, 100.0) } else { 0.0 };
        let content = html! {
            <>
                <span class="yi-bar-list__label">
                    if let Some(icon) = item.icon {
                        <Icon {icon} />
                    }
                    if let Some(glyph) = &item.glyph {
                        <span class="yi-bar-list__glyph" aria-hidden="true">{ glyph.clone() }</span>
                    }
                    <span class="yi-bar-list__text">{ item.label.clone() }</span>
                </span>
                if props.show_share {
                    <span class="yi-bar-list__share">{ share(item.amount, total) }</span>
                }
                <span class="yi-bar-list__value">
                    { item.display.clone().unwrap_or_else(|| compact_number(item.amount).into()) }
                </span>
            </>
        };
        let row = if let Some(href) = &item.href {
            html! { <a class="yi-bar-list__row" href={href.clone()}>{ content }</a> }
        } else if props.on_select.is_some() {
            let pressed = selected == Some(&item.value);
            html! {
                <button
                    type="button"
                    class="yi-bar-list__row"
                    aria-pressed={if pressed { "true" } else { "false" }}
                    data-yi-row={index.to_string()}
                >
                    { content }
                </button>
            }
        } else {
            html! { <div class="yi-bar-list__row">{ content }</div> }
        };
        let action = props.action.as_ref().filter(|_| !item.muted).map(|action| {
            html! {
                <button
                    type="button"
                    class="yi-bar-list__action"
                    title={action.label.clone()}
                    aria-label={format!("{}: {}", action.label, item.label)}
                    data-yi-action={index.to_string()}
                >
                    <Icon icon={action.icon} />
                </button>
            }
        });

        html! {
            <li
                key={index}
                class={classes!(
                    "yi-bar-list__item",
                    item.muted.then_some("yi-bar-list__item--muted"),
                    item.tone.map(|tone| format!("yi-bar-list__item--{}", tone.as_str())),
                )}
                style={format!("--yi-bar-list-fill:{fill:.1}%")}
            >
                { row }
                { action }
            </li>
        }
    });

    // One handler for every row and row action: the control that was hit
    // names its item by index.
    let onclick = (props.on_select.is_some() || props.action.is_some()).then(|| {
        let (items, on_select, action) = (props.items.clone(), props.on_select.clone(), props.action.clone());
        Callback::from(move |event: MouseEvent| {
            let Some((is_action, index)) = hit(&event) else { return };
            let Some(item) = items.get(index) else { return };
            if is_action {
                if let Some(action) = &action {
                    action.on_click.emit(item.value.clone());
                }
            } else if let Some(on_select) = &on_select {
                on_select.emit(item.value.clone());
            }
        })
    });

    let footer = match props.overflow {
        BarListOverflow::Expand(limit) if count > limit.max(1) => {
            let onclick = {
                let expanded = expanded.clone();
                Callback::from(move |_: MouseEvent| expanded.set(!*expanded))
            };
            html! {
                <button
                    type="button"
                    class="yi-bar-list__more"
                    aria-expanded={if *expanded { "true" } else { "false" }}
                    {onclick}
                >
                    { if *expanded { "Show fewer".to_string() } else { format!("Show all {count}") } }
                </button>
            }
        }
        BarListOverflow::Pages(_) if pages > 1 => {
            let on_change = {
                let page = page.clone();
                Callback::from(move |to: u32| page.set(to))
            };
            html! {
                <Pagination
                    class="yi-bar-list__pages"
                    page={(*page).clamp(1, pages as u32)}
                    pages={pages as u32}
                    compact=true
                    size={Size::Small}
                    label={match &props.label {
                        Some(label) => format!("{label} pages"),
                        None => "Pages".to_string(),
                    }}
                    {on_change}
                />
            }
        }
        _ => Html::default(),
    };

    let class = classes!(
        "yi-bar-list",
        format!("yi-bar-list--{}", props.variant.as_str()),
        format!("yi-bar-list--{}", props.size.as_str()),
        (props.tone != Tone::Neutral).then(|| format!("yi-bar-list--{}", props.tone.as_str())),
        (shown.len() > LONG).then_some("yi-bar-list--long"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            if props.heading.is_some() || props.value_heading.is_some() {
                <div class="yi-bar-list__head">
                    <span>{ props.heading.clone() }</span>
                    <span>{ props.value_heading.clone() }</span>
                </div>
            }
            if count == 0 {
                <div class="yi-bar-list__empty">
                    if is_empty(&props.empty) {
                        { "No data" }
                    } else {
                        { props.empty.clone() }
                    }
                </div>
            } else {
                <ol class="yi-bar-list__rows" role="list" aria-label={props.label.clone()} {onclick}>
                    { for rows }
                </ol>
                { footer }
            }
        </div>
    })
}

/// The row (`false`) or row action (`true`) a click landed on, and the index
/// of its item. Always `None` off the web.
fn hit(event: &MouseEvent) -> Option<(bool, usize)> {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;

        let target = event.target()?.dyn_into::<web_sys::Element>().ok()?;
        let control = target.closest("[data-yi-row],[data-yi-action]").ok()??;
        let action = control.get_attribute("data-yi-action");
        let is_action = action.is_some();
        let index = action.or_else(|| control.get_attribute("data-yi-row"))?.parse().ok()?;
        Some((is_action, index))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = event;
        None
    }
}

#[cfg(test)]
mod tests;
