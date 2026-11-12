use std::rc::Rc;

use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{attrs, icons};

fn items() -> Rc<[BarItem]> {
    Rc::from(vec![
        BarItem::new("/docs", "Docs", 250.0),
        BarItem::new("/", "Home", 1500.0),
        BarItem::new("/blog", "Blog", 250.0),
    ])
}

fn many(count: usize) -> Rc<[BarItem]> {
    (0..count).map(|i| BarItem::labelled(format!("Row {i:03}"), (count - i) as f64)).collect()
}

/// The fill of each row, in display order.
fn fills(html: &str) -> Vec<String> {
    html.split("--yi-bar-list-fill:").skip(1).map(|rest| rest[..rest.find('%').unwrap()].to_string()).collect()
}

fn position(html: &str, text: &str) -> usize {
    html.find(text).unwrap_or_else(|| panic!("missing {text}"))
}

#[test]
fn ranks_rows_largest_first_with_bars_relative_to_the_largest() {
    let html = render(|| html! { <BarList items={items()} label="Top pages" /> });
    assert!(position(&html, "Home") < position(&html, "Docs"));
    assert!(position(&html, "Docs") < position(&html, "Blog"), "equal rows keep their given order");
    assert_eq!(fills(&html), ["100.0", "16.7", "16.7"]);
    assert!(html.contains(r#"aria-label="Top pages""#));
    assert!(html.contains("1.5k"));
}

#[test]
fn given_order_and_an_explicit_max_are_respected() {
    let html = render(|| html! { <BarList items={items()} order={BarListOrder::Given} max={3000.0} /> });
    assert!(position(&html, "Docs") < position(&html, "Home"));
    assert_eq!(fills(&html)[1], "50.0");
}

#[test]
fn bars_stay_within_the_row_for_odd_amounts() {
    let odd: Rc<[BarItem]> =
        Rc::from(vec![BarItem::labelled("Negative", -5.0), BarItem::labelled("Over", 20.0), BarItem::labelled("Nan", f64::NAN)]);
    let html = render(move || html! { <BarList items={odd.clone()} max={10.0} order={BarListOrder::Given} /> });
    assert_eq!(fills(&html), ["0.0", "100.0", "0.0"]);

    let zero: Rc<[BarItem]> = Rc::from(vec![BarItem::labelled("Nothing", 0.0)]);
    let html = render(move || html! { <BarList items={zero.clone()} show_share=true /> });
    assert_eq!(fills(&html), ["0.0"]);
    assert!(html.contains("0%"));
}

#[test]
fn shares_are_of_the_sum_unless_a_total_is_given() {
    let html = render(|| html! { <BarList items={items()} show_share=true /> });
    assert!(html.contains("75%"));

    let html = render(|| html! { <BarList items={items()} show_share=true total={1_000_000.0} /> });
    assert!(html.contains("&lt;1%") || html.contains("<1%"), "a small share is not rounded to nothing");
}

#[test]
fn compact_numbers() {
    for (value, expected) in [
        (0.0, "0"),
        (950.0, "950"),
        (12.5, "12.5"),
        (1000.0, "1k"),
        (1234.0, "1.2k"),
        (999_960.0, "1M"),
        (3_400_000.0, "3.4M"),
        (2e9, "2B"),
        (-1500.0, "-1.5k"),
    ] {
        assert_eq!(compact_number(value), expected, "{value}");
    }
}

#[test]
fn display_overrides_the_formatted_amount() {
    let items: Rc<[BarItem]> = Rc::from(vec![BarItem::labelled("p95", 1840.0).display("1.84 s")]);
    let html = render(move || html! { <BarList items={items.clone()} /> });
    assert!(html.contains("1.84 s"));
    assert!(!html.contains("1.8k"));
}

#[test]
fn rows_are_text_links_or_toggle_buttons() {
    let plain = render(|| html! { <BarList items={items()} /> });
    assert!(!plain.contains("<button") && !plain.contains("<a "));

    let linked: Rc<[BarItem]> = Rc::from(vec![BarItem::labelled("Docs", 1.0).href("/docs")]);
    let html = render(move || html! { <BarList items={linked.clone()} /> });
    assert!(html.contains(r#"href="/docs""#));

    let html = render(|| html! { <BarList items={items()} selected="/docs" on_select={Callback::noop()} /> });
    assert_eq!(html.matches(r#"aria-pressed="true""#).count(), 1);
    assert_eq!(html.matches(r#"aria-pressed="false""#).count(), 2);

    let html = render(|| html! { <BarList items={items()} selected="/docs" /> });
    assert!(!html.contains("aria-pressed"), "nothing is selected without on_select");
}

#[test]
fn row_actions_are_named_after_their_row_and_skip_muted_rows() {
    let items: Rc<[BarItem]> = Rc::from(vec![BarItem::labelled("Apps", 2.0), BarItem::new("", "Unknown", 1.0).muted()]);
    let html = render(move || {
        let action = BarListAction::new(icons::SETTINGS, "Manage", Callback::noop());
        html! { <BarList items={items.clone()} {action} /> }
    });
    assert!(html.contains(r#"aria-label="Manage: Apps""#));
    assert!(!html.contains("Manage: Unknown"));
}

#[test]
fn overflow_limits_the_rows_shown() {
    let count = |html: &str| fills(html).len();

    let all = render(|| html! { <BarList items={many(30)} /> });
    assert_eq!(count(&all), 30);

    let expand = render(|| html! { <BarList items={many(30)} overflow={BarListOverflow::Expand(8)} /> });
    assert_eq!(count(&expand), 8);
    assert!(expand.contains("Show all 30"));
    assert!(expand.contains(r#"aria-expanded="false""#));

    let pages = render(|| html! { <BarList items={many(30)} label="Paths" overflow={BarListOverflow::Pages(8)} /> });
    assert_eq!(count(&pages), 8);
    assert!(pages.contains("Page 1 of 4"));
    assert!(pages.contains(r#"aria-label="Paths pages""#));

    for overflow in [BarListOverflow::Expand(8), BarListOverflow::Pages(8)] {
        let html = render(move || html! { <BarList items={many(8)} {overflow} /> });
        assert_eq!(count(&html), 8);
        assert!(!html.contains("Show all") && !html.contains("<nav"), "{overflow:?} adds controls it does not need");
    }
}

#[test]
fn empty_lists_say_so() {
    let none = || -> Rc<[BarItem]> { Rc::from(Vec::new()) };
    assert!(render(move || html! { <BarList items={none()} /> }).contains("No data"));
    let html = render(move || html! { <BarList items={none()} empty={html! { "Nothing this week" }} /> });
    assert!(html.contains("Nothing this week"));
    assert!(!html.contains("<ol"));
}

#[test]
fn headings_icons_and_glyphs_are_rendered() {
    let items: Rc<[BarItem]> =
        Rc::from(vec![BarItem::labelled("Germany", 3.0).glyph("DE"), BarItem::labelled("Chrome", 2.0).icon(icons::GLOBE)]);
    let html = render(move || html! { <BarList items={items.clone()} heading="Source" value_heading="Visitors" /> });
    for text in ["Source", "Visitors", "DE", "<svg"] {
        assert!(html.contains(text), "missing {text}");
    }
}

#[test]
fn variants_sizes_and_tones_are_distinguishable() {
    let mut rendered = vec![render(|| html! { <BarList items={items()} /> })];
    rendered.push(render(|| html! { <BarList items={items()} variant={BarListVariant::Under} /> }));
    rendered
        .extend([Size::Small, Size::Large].map(|size| render(move || html! { <BarList items={items()} {size} /> })));
    rendered
        .extend(Tone::ALL.into_iter().skip(1).map(|tone| render(move || html! { <BarList items={items()} {tone} /> })));
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <BarList id="pages" class="mine" attrs={attrs! { "data-panel" => "pages" }} items={items()} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="pages""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-panel="pages""#));
}
