use yewi_test::render;

use super::*;
use crate::attrs;

use PageSlot::{Gap, Page as P};

#[test]
fn slots_show_the_ends_the_neighbours_and_gaps_between() {
    let cases: &[(u32, u32, u32, u32, &[PageSlot])] = &[
        (1, 1, 1, 1, &[P(1)]),
        (2, 5, 1, 1, &[P(1), P(2), P(3), P(4), P(5)]),
        (1, 12, 1, 1, &[P(1), P(2), P(3), P(4), P(5), Gap, P(12)]),
        (6, 12, 1, 1, &[P(1), Gap, P(5), P(6), P(7), Gap, P(12)]),
        (12, 12, 1, 1, &[P(1), Gap, P(8), P(9), P(10), P(11), P(12)]),
        (4, 12, 1, 1, &[P(1), P(2), P(3), P(4), P(5), Gap, P(12)]),
        (6, 12, 0, 1, &[P(1), Gap, P(6), Gap, P(12)]),
        (6, 12, 1, 2, &[P(1), P(2), Gap, P(5), P(6), P(7), Gap, P(11), P(12)]),
        (99, 3, 1, 1, &[P(1), P(2), P(3)]),
    ];
    for &(page, pages, siblings, boundaries, expected) in cases {
        assert_eq!(page_slots(page, pages, siblings, boundaries), expected, "page {page} of {pages}");
    }
}

#[test]
fn slots_keep_their_count_and_order_on_every_page() {
    for pages in [8_u32, 20, 500] {
        let len = page_slots(1, pages, 1, 1).len();
        for page in 1..=pages {
            let slots = page_slots(page, pages, 1, 1);
            assert_eq!(slots.len(), len, "page {page} of {pages}");
            assert!(slots.contains(&P(page)) && slots.contains(&P(1)) && slots.contains(&P(pages)));
            let numbers: Vec<u32> = slots.iter().filter_map(|s| if let P(n) = s { Some(*n) } else { None }).collect();
            assert!(numbers.windows(2).all(|pair| pair[0] < pair[1]));
        }
    }
}

/// The opening tags of the controls, in order.
fn controls(html: &str) -> Vec<&str> {
    html.split('<').filter(|tag| tag.starts_with("button") || tag.starts_with("a ")).collect()
}

#[test]
fn renders_a_named_landmark_with_the_current_page_marked() {
    let html = render(|| html! { <Pagination page=3 pages=12 /> });
    assert!(html.starts_with("<nav") && html.contains(r#"aria-label="Pagination""#));
    assert_eq!(html.matches(r#"aria-current="page""#).count(), 1);
    let current = controls(&html).into_iter().find(|tag| tag.contains("aria-current")).unwrap();
    assert!(current.contains(r#"aria-label="Page 3""#));
}

#[test]
fn previous_and_next_are_disabled_at_the_ends() {
    let cases = [(1, Some(12), true, true, false), (12, Some(12), true, false, true), (1, None, false, true, true)];
    for (page, pages, has_next, previous_off, next_off) in cases {
        let html = render(move || html! { <Pagination {page} {pages} {has_next} /> });
        let controls = controls(&html);
        let is_off = |label: &str| {
            controls.iter().find(|tag| tag.contains(label)).is_some_and(|tag| tag.contains(" disabled"))
        };
        assert_eq!(is_off("Previous page"), previous_off, "page {page} of {pages:?}");
        assert_eq!(is_off("Next page"), next_off, "page {page} of {pages:?}");
    }
}

#[test]
fn compact_and_unknown_totals_show_a_status_instead_of_numbers() {
    let compact = render(|| html! { <Pagination page=3 pages=12 compact=true /> });
    assert!(compact.contains("Page 3 of 12"));
    assert_eq!(controls(&compact).len(), 2);

    let unknown = render(|| html! { <Pagination page=3 /> });
    assert!(unknown.contains("Page 3") && !unknown.contains(" of "));
    assert_eq!(controls(&unknown).len(), 2);
}

#[test]
fn href_renders_links_and_leaves_disabled_ones_unfollowable() {
    let html = render(|| {
        let href = Callback::from(|page: u32| NavTarget::href("/items").query([("page", page.to_string())]));
        html! { <Pagination page=1 pages=3 {href} /> }
    });
    assert!(!html.contains("<button"));
    assert!(html.contains(r#"href="/items?page=2""#) && html.contains(r#"href="/items?page=3""#));
    let previous = controls(&html).into_iter().find(|tag| tag.contains("Previous page")).unwrap();
    assert!(!previous.contains("href=") && previous.contains(r#"aria-disabled="true""#));
}

#[test]
fn disabled_turns_every_control_off() {
    let html = render(|| html! { <Pagination page=2 pages=3 disabled=true /> });
    assert!(controls(&html).iter().all(|tag| tag.contains(" disabled")));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Pagination page=1 pages=2 label="Results pages" id="pager" class="mine" attrs={attrs! { "data-test" => "x" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"aria-label="Results pages""#) && root.contains(r#"id="pager""#));
    assert!(root.contains("mine") && root.contains(r#"data-test="x""#));
}
