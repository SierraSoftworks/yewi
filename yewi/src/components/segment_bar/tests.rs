use std::rc::Rc;

use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

fn segments(values: &[(f64, Tone, &'static str)]) -> Rc<[BarSegment]> {
    values.iter().map(|&(value, tone, label)| BarSegment::new(value, tone, label)).collect()
}

fn summary(total: Option<f64>, values: &'static [(f64, Tone, &'static str)]) -> String {
    render(move || html! { <SegmentBar label="Hosts" segments={segments(values)} {total} rest_label="to go" /> })
}

#[test]
fn is_one_image_that_names_every_part() {
    let html = summary(Some(20.0), &[(12.0, Tone::Ok, "done"), (3.0, Tone::Danger, "failed")]);
    assert!(html.contains(r#"role="img""#));
    assert!(html.contains(r#"aria-label="Hosts: 12 done, 3 failed, 5 to go""#), "{html}");
}

#[test]
fn leaves_out_empty_segments_and_a_spent_remainder() {
    let html = summary(Some(10.0), &[(10.0, Tone::Ok, "done"), (0.0, Tone::Danger, "failed")]);
    assert!(html.contains("10 done"));
    assert!(!html.contains("failed"));
    assert!(!html.contains("to go"));
}

#[test]
fn segments_beyond_the_total_leave_no_remainder() {
    let html = summary(Some(5.0), &[(4.0, Tone::Ok, "done"), (3.0, Tone::Warn, "held")]);
    assert!(html.contains("4 done, 3 held"));
    assert!(!html.contains("to go"));
}

#[test]
fn fractional_values_keep_one_decimal() {
    let html = summary(None, &[(2.5, Tone::Info, "GB used")]);
    assert!(html.contains("2.5 GB used"));
}

#[test]
fn the_legend_lists_each_part_and_stays_out_of_the_accessible_name() {
    let values = &[(12.0, Tone::Ok, "done"), (3.0, Tone::Danger, "failed")];
    let plain = render(|| html! { <SegmentBar segments={segments(values)} total={20.0} /> });
    let legend = render(|| html! { <SegmentBar segments={segments(values)} total={20.0} legend=true /> });
    assert!(!plain.contains("<ul"));
    assert!(legend.contains(r#"aria-hidden="true""#));
    assert_eq!(legend.matches("<li").count(), 3);
    assert!(legend.contains("remaining"));
}

#[test]
fn every_tone_and_size_is_distinguishable() {
    let mut rendered: Vec<String> = Tone::ALL
        .into_iter()
        .map(|tone| render(move || html! { <SegmentBar segments={segments(&[(1.0, tone, "x")])} /> }))
        .collect();
    rendered.extend(
        [Size::Small, Size::Large]
            .into_iter()
            .map(|size| render(move || html! { <SegmentBar segments={segments(&[(1.0, Tone::Neutral, "x")])} {size} /> })),
    );
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <SegmentBar id="hosts" class="mine" segments={segments(&[])} attrs={attrs! { "data-row" => "7" }} /> }
    });
    assert!(html.contains(r#"id="hosts""#));
    assert!(html.contains("mine"));
    assert!(html.contains(r#"data-row="7""#));
}
