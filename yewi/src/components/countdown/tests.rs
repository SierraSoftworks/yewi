use chrono::{TimeDelta, TimeZone};
use yewi_test::render;

use super::*;
use crate::{Now, NowProvider, attrs};

fn noon() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 1, 15, 12, 0, 0).unwrap()
}

/// Renders a countdown with `millis_left` on the clock.
fn render_with_left(millis_left: i64, format: CountdownFormat, done: Option<&'static str>) -> String {
    render(move || {
        let now = Now::from_epoch_millis(noon().timestamp_millis() as f64);
        let until = noon() + TimeDelta::milliseconds(millis_left);
        html! {
            <NowProvider {now} live=false>
                <Countdown {until} {format} done={done.map(Html::from)} />
            </NowProvider>
        }
    })
}

#[test]
fn counts_down_in_either_format() {
    for (millis, clock, units) in [
        (245_000, "4:05", "4m 05s"),
        (3_750_000, "1:02:30", "1h 02m"),
        (183_845_000, "2d 03:04:05", "2d 3h"),
        (59_000, "0:59", "59s"),
        // A part second still counts: zero means the time is up.
        (400, "0:01", "1s"),
    ] {
        assert!(render_with_left(millis, CountdownFormat::Clock, None).contains(&format!(">{clock}<")));
        assert!(render_with_left(millis, CountdownFormat::Units, None).contains(&format!(">{units}<")));
    }
}

#[test]
fn never_goes_negative() {
    for millis in [0, -90_000] {
        assert!(render_with_left(millis, CountdownFormat::Clock, None).contains(">0:00<"));
    }
}

#[test]
fn done_content_replaces_the_zero_only_once_the_time_is_up() {
    assert!(render_with_left(-1_000, CountdownFormat::Clock, Some("expired")).contains("expired"));
    assert!(!render_with_left(1_000, CountdownFormat::Clock, Some("expired")).contains("expired"));
}

#[test]
fn is_a_timer_carrying_its_target() {
    let html = render_with_left(245_000, CountdownFormat::Clock, None);
    assert!(html.contains(r#"role="timer""#));
    assert!(html.contains(r#"datetime="2026-01-15T12:04:05Z""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Countdown until={noon()} id="bake" class="mine" attrs={attrs! { "data-step" => "2" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="bake""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-step="2""#));
}
