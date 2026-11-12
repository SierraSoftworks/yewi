use std::rc::Rc;

use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

fn cells() -> Rc<[HistoryCell]> {
    Rc::from([
        HistoryCell::new(Tone::Ok, "11:00, passing"),
        HistoryCell::new(Tone::Danger, "11:10, failing"),
        HistoryCell::new(Tone::Neutral, ""),
    ])
}

fn detail() -> Callback<usize, HistoryDetail> {
    Callback::from(|index: usize| HistoryDetail::new(format!("detail {index}")))
}

#[test]
fn is_a_named_group_of_named_cells() {
    let html = render(|| html! { <HistoryStrip label="Last 30 minutes" cells={cells()} /> });
    assert!(html.contains(r#"role="group""#));
    assert!(html.contains(r#"aria-label="Last 30 minutes""#));
    assert!(html.contains(r#"aria-label="11:00, passing""#));
    assert!(html.contains(r#"aria-label="11:10, failing""#));
    assert_eq!(html.matches(r#"role="img""#).count(), 2);
    // A cell without a label is hidden rather than announced as a nameless image.
    assert_eq!(html.matches(r#"aria-hidden="true""#).count(), 1);
}

#[test]
fn is_a_tab_stop_only_when_it_has_detail_to_show() {
    let plain = render(|| html! { <HistoryStrip cells={cells()} /> });
    let detailed = render(|| html! { <HistoryStrip cells={cells()} detail={detail()} /> });
    assert!(!plain.contains("tabindex"));
    assert!(detailed.contains(r#"tabindex="0""#));
}

#[test]
fn no_detail_is_built_until_a_cell_is_looked_at() {
    let html = render(|| html! { <HistoryStrip cells={cells()} detail={detail()} /> });
    assert!(!html.contains("detail "));
}

#[test]
fn every_tone_variant_and_size_is_distinguishable() {
    let one = |tone| -> Rc<[HistoryCell]> { Rc::from([HistoryCell::new(tone, "x")]) };
    let mut rendered: Vec<String> =
        Tone::ALL.into_iter().map(|tone| render(move || html! { <HistoryStrip cells={one(tone)} /> })).collect();
    rendered.push(render(move || {
        html! { <HistoryStrip cells={one(Tone::Neutral)} variant={HistoryStripVariant::Discrete} /> }
    }));
    rendered.extend(
        [Size::Small, Size::Large]
            .into_iter()
            .map(|size| render(move || html! { <HistoryStrip cells={one(Tone::Neutral)} {size} /> })),
    );
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn keys_move_between_cells_and_stop_at_the_ends() {
    let cases = [
        ("ArrowLeft", Some(2), Some(1)),
        ("ArrowLeft", Some(0), Some(0)),
        ("ArrowRight", Some(1), Some(2)),
        ("ArrowRight", Some(2), Some(2)),
        ("ArrowLeft", None, Some(2)),
        ("Home", Some(2), Some(0)),
        ("End", Some(0), Some(2)),
        ("a", Some(1), None),
    ];
    for (key, from, to) in cases {
        assert_eq!(step(key, from, 3), to, "{key} from {from:?}");
    }
    assert_eq!(step("End", None, 0), None);
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <HistoryStrip id="probe" class="mine" cells={cells()} attrs={attrs! { "data-probe" => "api" }} /> }
    });
    assert!(html.contains(r#"id="probe""#));
    assert!(html.contains("mine"));
    assert!(html.contains(r#"data-probe="api""#));
}
