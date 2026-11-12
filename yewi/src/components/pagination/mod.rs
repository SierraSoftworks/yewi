//! Pagination: move through a long list one page at a time.

use yew::prelude::*;

use crate::{Attrs, Icon, NavTarget, Size, icons, nav_target::use_nav_link};

/// One slot in the row of page numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageSlot {
    /// A page number, counted from 1.
    Page(u32),
    /// Pages left out between two numbers.
    Gap,
}

/// The slots to show for `page` of `pages`.
///
/// - `siblings` pages are shown either side of the current one and
///   `boundaries` at each end, with a [`PageSlot::Gap`] where pages are left
///   out. A gap never stands in for a single page.
/// - The number of slots depends only on `pages`, `siblings` and
///   `boundaries`, so the controls do not move as the user pages through.
pub fn page_slots(page: u32, pages: u32, siblings: u32, boundaries: u32) -> Vec<PageSlot> {
    let (pages, siblings, boundaries) = (i64::from(pages), i64::from(siblings), i64::from(boundaries));
    let page = i64::from(page).clamp(1, pages.max(1));

    let start_end = boundaries.min(pages);
    let end_start = (pages - boundaries + 1).max(boundaries + 1);
    let siblings_start = (page - siblings).min(pages - boundaries - siblings * 2 - 1).max(boundaries + 2);
    let siblings_end = (page + siblings).max(boundaries + siblings * 2 + 2).min(end_start - 2);

    let numbers = |from: i64, to: i64| (from..=to).map(|n| PageSlot::Page(n as u32));
    let mut slots: Vec<PageSlot> = numbers(1, start_end).collect();
    if siblings_start > boundaries + 2 {
        slots.push(PageSlot::Gap);
    } else if boundaries + 1 < pages - boundaries {
        slots.push(PageSlot::Page((boundaries + 1) as u32));
    }
    slots.extend(numbers(siblings_start, siblings_end));
    if siblings_end < pages - boundaries - 1 {
        slots.push(PageSlot::Gap);
    } else if pages - boundaries > boundaries {
        slots.push(PageSlot::Page((pages - boundaries) as u32));
    }
    slots.extend(numbers(end_start, pages));
    slots
}

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    /// The current page, counted from 1.
    pub page: u32,
    /// The number of pages. Leave it unset when the total is unknown, as with
    /// cursor-based APIs: only previous and next are offered.
    #[prop_or_default]
    pub pages: Option<u32>,
    /// With an unknown total, whether there is a page after this one.
    #[prop_or(true)]
    pub has_next: bool,
    /// Page numbers shown either side of the current page.
    #[prop_or(1)]
    pub siblings: u32,
    /// Page numbers always shown at each end.
    #[prop_or(1)]
    pub boundaries: u32,
    /// Replaces the page numbers with "Page 3 of 12", for narrow spaces.
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub size: Size,
    /// Nothing can be activated, for example while a page is loading.
    #[prop_or_default]
    pub disabled: bool,
    /// Makes every control a link to the given page: a URL or, with the
    /// `router` feature, `NavTarget::route(..)`. Without it the controls are
    /// buttons and `on_change` does the work.
    #[prop_or_default]
    pub href: Option<Callback<u32, NavTarget>>,
    /// Fired with the page the user asks for.
    #[prop_or_default]
    pub on_change: Callback<u32>,
    /// Accessible name of the navigation landmark.
    #[prop_or(AttrValue::Static("Pagination"))]
    pub label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Previous, next and page-number controls for a paged list.
///
/// The component holds no state: render it with the current `page` and
/// update that from `on_change` (or from the URL, when using `href`).
#[function_component]
pub fn Pagination(props: &PaginationProps) -> Html {
    let page = props.page.max(1);
    let pages = props.pages.map(|pages| pages.max(1));
    let page = pages.map_or(page, |pages| page.min(pages));
    let has_next = pages.map_or(props.has_next, |pages| page < pages);

    let control = |to: u32, label: String, current: bool, enabled: bool, content: Html| {
        html! {
            <li class="yi-pagination__slot">
                <PageControl
                    page={to}
                    {label}
                    {current}
                    disabled={props.disabled || !enabled}
                    target={props.href.as_ref().map(|href| href.emit(to))}
                    on_change={props.on_change.clone()}
                >
                    { content }
                </PageControl>
            </li>
        }
    };

    let numbers = match pages {
        Some(pages) if !props.compact => page_slots(page, pages, props.siblings, props.boundaries)
            .into_iter()
            .map(|slot| match slot {
                PageSlot::Page(n) => control(n, format!("Page {n}"), n == page, true, html! { { n } }),
                PageSlot::Gap => html! {
                    <li class="yi-pagination__slot yi-pagination__gap" aria-hidden="true">{ "\u{2026}" }</li>
                },
            })
            .collect::<Html>(),
        _ => html! {
            <li class="yi-pagination__slot yi-pagination__status">
                { match pages {
                    Some(pages) => format!("Page {page} of {pages}"),
                    None => format!("Page {page}"),
                } }
            </li>
        },
    };

    let class = classes!("yi-pagination", format!("yi-pagination--{}", props.size.as_str()), props.class.clone());

    props.attrs.apply(html! {
        <nav id={props.id.clone()} {class} aria-label={props.label.clone()}>
            <ul class="yi-pagination__list" role="list">
                { control(page.saturating_sub(1), "Previous page".into(), false, page > 1, html! {
                    <Icon icon={icons::CHEVRON_LEFT} />
                }) }
                { numbers }
                { control(page.saturating_add(1), "Next page".into(), false, has_next, html! {
                    <Icon icon={icons::CHEVRON_RIGHT} />
                }) }
            </ul>
        </nav>
    })
}

#[derive(Properties, PartialEq)]
struct PageControlProps {
    page: u32,
    label: AttrValue,
    current: bool,
    disabled: bool,
    target: Option<NavTarget>,
    on_change: Callback<u32>,
    children: Html,
}

/// One control: a link when it has a target, otherwise a button.
#[function_component]
fn PageControl(props: &PageControlProps) -> Html {
    let onclick = {
        let (on_change, page, disabled) = (props.on_change.clone(), props.page, props.disabled);
        Callback::from(move |event: MouseEvent| {
            if disabled {
                event.prevent_default();
            } else {
                on_change.emit(page);
            }
        })
    };
    let target = props.target.clone().unwrap_or_default();
    let link = use_nav_link(&target, onclick);

    let aria_current = props.current.then_some("page");
    let content = props.children.clone();

    match link.href {
        Some(href) => html! {
            <a
                class="yi-pagination__control"
                href={(!props.disabled).then_some(href)}
                role={props.disabled.then_some("link")}
                aria-label={props.label.clone()}
                aria-current={aria_current}
                aria-disabled={props.disabled.then_some("true")}
                onclick={link.onclick}
            >
                { content }
            </a>
        },
        None => html! {
            <button
                type="button"
                class="yi-pagination__control"
                aria-label={props.label.clone()}
                aria-current={aria_current}
                disabled={props.disabled}
                onclick={link.onclick}
            >
                { content }
            </button>
        },
    }
}

#[cfg(test)]
mod tests;
