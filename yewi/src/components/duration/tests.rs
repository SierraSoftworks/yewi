use chrono::TimeZone;
use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{Now, NowProvider, attrs};

fn text(millis: i64, style: DurationStyle) -> String {
    format::duration(millis, style.units())
}

#[test]
fn each_style_writes_to_the_precision_that_matters() {
    for (millis, short, compact, long) in [
        (0, "0s", "0s", "0 seconds"),
        (230, "230ms", "230ms", "230 milliseconds"),
        (1_400, "1.4s", "1s", "1.4 seconds"),
        (2_000, "2s", "2s", "2 seconds"),
        (45_000, "45s", "45s", "45 seconds"),
        (60_000, "1m 00s", "1m", "1 minute"),
        (125_000, "2m 05s", "2m", "2 minutes 5 seconds"),
        (3_720_000, "1h 02m", "1h", "1 hour 2 minutes"),
        (273_600_000, "3d 4h", "3d", "3 days 4 hours"),
        (-125_000, "-2m 05s", "-2m", "-2 minutes 5 seconds"),
    ] {
        assert_eq!(text(millis, DurationStyle::Short), short);
        assert_eq!(text(millis, DurationStyle::Compact), compact);
        assert_eq!(text(millis, DurationStyle::Long), long);
    }
}

#[test]
fn renders_a_time_element_with_the_iso_duration() {
    let html = render(|| html! { <Duration value={TimeDelta::milliseconds(125_500)} /> });
    assert!(html.starts_with("<time"));
    assert!(html.contains(r#"datetime="PT125.500S""#));
    assert!(html.contains("2m 05s"));
}

#[test]
fn spans_the_time_between_since_and_until() {
    let html = render(|| {
        let since = Utc.with_ymd_and_hms(2026, 1, 15, 11, 0, 0).unwrap();
        html! { <Duration {since} until={since + TimeDelta::minutes(62)} /> }
    });
    assert!(html.contains("1h 02m"));
}

#[test]
fn with_only_since_it_shows_the_time_elapsed() {
    let html = render(|| {
        let since = Utc.with_ymd_and_hms(2026, 1, 15, 11, 55, 30).unwrap();
        let now = Now::from_epoch_millis((since + TimeDelta::milliseconds(270_400)).timestamp_millis() as f64);
        html! { <NowProvider {now} live=false><Duration {since} /></NowProvider> }
    });
    assert!(html.contains("4m 30s"));
    assert!(html.contains(r#"datetime="PT270S""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Duration value={TimeDelta::seconds(5)} id="took" class="mine" attrs={attrs! { "title" => "Build" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="took""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"title="Build""#));
}
