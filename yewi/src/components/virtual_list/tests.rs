use std::rc::Rc;

use yewi_test::render;

use super::*;
use crate::{Column, Sort};

fn numbers(count: u32) -> Rc<[u32]> {
    (0..count).collect()
}

fn list(count: u32) -> String {
    render(move || {
        html! {
            <VirtualList<u32>
                label="Numbers"
                items={numbers(count)}
                render={RowFn::new(|n: &u32| html! { <span>{ format!("item {n}") }</span> })}
            />
        }
    })
}

fn table(count: u32, sort: Vec<Sort>) -> String {
    render(move || {
        let columns: Rc<[Column<u32>]> =
            Rc::from([Column::new("n", "Number", |n: &u32| format!("row {n}")).sort_by_key(|n| *n)]);
        html! { <VirtualTable<u32> label="Numbers" {columns} rows={numbers(count)} default_sort={sort.clone()} /> }
    })
}

#[test]
fn the_window_covers_the_view_and_stays_small() {
    // (scroll offset, view height, row height, rows, overscan)
    let cases = [
        (0.0, 400.0, 40.0, 100_000, 8),
        (1_234_567.0, 400.0, 40.0, 100_000, 8),
        (3_999_600.0, 400.0, 40.0, 100_000, 8),
        (500.0, 900.0, 31.0, 1_000, 2),
        (0.0, 400.0, 40.0, 5, 8),
        (9_999_999.0, 400.0, 40.0, 50, 8),
        (0.0, 400.0, 40.0, 0, 8),
    ];
    for (top, view, row, count, overscan) in cases {
        let range = virtual_range(top, view, row, count, overscan);
        let first = ((top / row) as usize).min(count);
        let last = (((top + view) / row).ceil() as usize).min(count);
        assert!(range.start <= first && range.end >= last, "{range:?} must cover rows {first}..{last}");
        assert!(range.end <= count);
        assert!(range.len() <= (view / row) as usize + 4 * overscan + 8, "{range:?} draws too much");
    }
}

#[test]
fn the_window_moves_in_steps_and_starts_on_an_even_row() {
    let ranges: Vec<_> = (0..400).map(|pixel| virtual_range(f64::from(pixel) * 10.0, 400.0, 40.0, 100_000, 8)).collect();
    let moves = ranges.windows(2).filter(|pair| pair[0] != pair[1]).count();
    assert!(moves <= 100 / 4, "scrolling 100 rows redrew {moves} times");
    assert!(ranges.iter().all(|range| range.start % 2 == 0), "striped rows would change colour");
}

#[test]
fn a_list_draws_a_window_and_reserves_the_rest() {
    let html = list(100_000);
    let drawn = html.matches(r#"role="listitem""#).count();
    assert!((1..100).contains(&drawn), "drew {drawn} rows");
    assert!(html.contains("item 0") && !html.contains("item 5000"));
    assert!(html.contains(r#"aria-setsize="100000""#) && html.contains(r#"aria-posinset="1""#));
    assert!(html.contains("block-size:4000000px"), "the scroll height stands for every row");
    assert!(html.contains(r#"aria-label="Numbers""#) && html.contains(r#"tabindex="0""#));
}

#[test]
fn a_short_list_draws_everything_and_an_empty_one_says_so() {
    assert_eq!(list(3).matches(r#"role="listitem""#).count(), 3);
    let empty = list(0);
    assert!(empty.contains("Nothing to show"));
    assert!(!empty.contains(r#"role="list""#), "a list role needs items");
}

#[test]
fn a_table_draws_a_window_of_the_sorted_rows() {
    let cases = [(vec![], "row 0", "row 99999"), (vec![Sort::desc("n")], "row 99999", ">row 0<")];
    for (sort, first, absent) in cases {
        let html = table(100_000, sort);
        let drawn = html.matches("yi-table__row").count();
        assert!((1..100).contains(&drawn), "drew {drawn} rows");
        assert!(html.contains(first) && !html.contains(absent));
        assert!(html.contains(r#"aria-rowcount="100001""#));
        assert!(html.contains(r#"aria-rowindex="2""#), "the first data row follows the header");
    }
}

#[test]
fn a_table_keeps_the_states_of_table() {
    assert!(table(0, vec![]).contains("Nothing to show"));
    assert_eq!(table(4, vec![]).matches("yi-table__row").count(), 4);
}
