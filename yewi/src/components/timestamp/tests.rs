use chrono::{TimeDelta, TimeZone};
use yew::prelude::*;
use yewi_test::render;

use super::{format::*, *};
use crate::{NowProvider, attrs};

/// Thursday 15 January 2026, 12:00 UTC.
fn noon() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 1, 15, 12, 0, 0).unwrap()
}

fn ago(seconds: i64) -> DateTime<Utc> {
    noon() - TimeDelta::seconds(seconds)
}

fn ms(at: DateTime<Utc>) -> i64 {
    at.timestamp_millis()
}

const UTC_STANDIN: Zone = Zone::Viewer { known: false };

/// Renders with the clock fixed at [`noon`], as a server would.
fn render_at_noon(view: fn() -> Html) -> String {
    render(move || {
        html! {
            <NowProvider now={Now::from_epoch_millis(noon().timestamp_millis() as f64)} live=false>
                { view() }
            </NowProvider>
        }
    })
}

#[test]
fn relative_text_reads_naturally_in_both_directions() {
    for (seconds, long, compact) in [
        (0, "just now", "now"),
        (59, "just now", "now"),
        (60, "1 minute ago", "1m ago"),
        (35 * 60, "35 minutes ago", "35m ago"),
        (3_600, "1 hour ago", "1h ago"),
        (23 * 3_600 + 59 * 60, "23 hours ago", "23h ago"),
        (3 * 86_400, "3 days ago", "3d ago"),
        (65 * 86_400, "2 months ago", "2mo ago"),
        (800 * 86_400, "2 years ago", "2y ago"),
        (-30, "in a moment", "now"),
        (-5 * 60, "in 5 minutes", "in 5m"),
        (-2 * 86_400, "in 2 days", "in 2d"),
    ] {
        assert_eq!(relative(ms(ago(seconds)), ms(noon()), false), long);
        assert_eq!(relative(ms(ago(seconds)), ms(noon()), true), compact);
    }
}

#[test]
fn auto_moves_from_relative_to_weekday_to_date() {
    for (seconds, long, compact) in [
        (35 * 60, "35 minutes ago", "35m ago"),
        (86_400 - 1, "23 hours ago", "23h ago"),
        (86_400, "Wednesday 12:00", "Wed 12:00"),
        (2 * 86_400 + 3_600, "Tuesday 11:00", "Tue 11:00"),
        (-3 * 86_400, "Sunday 12:00", "Sun 12:00"),
        (7 * 86_400, "8 Jan 2026", "8 Jan"),
        (40 * 86_400, "6 Dec 2025", "6 Dec 2025"),
        (-40 * 86_400, "24 Feb 2026", "24 Feb"),
    ] {
        let text = |compact| timestamp(ms(ago(seconds)), ms(noon()), TimestampDisplay::Auto, compact, UTC_STANDIN);
        assert_eq!(text(false), long);
        assert_eq!(text(true), compact);
    }
}

#[test]
fn fixed_displays_ignore_how_recent_the_moment_is() {
    let at = Utc.with_ymd_and_hms(2026, 1, 15, 9, 5, 7).unwrap();
    for (display, long, compact) in [
        (TimestampDisplay::Relative, "2 hours ago", "2h ago"),
        (TimestampDisplay::DateTime, "15 Jan 2026, 09:05", "15 Jan 09:05"),
        (TimestampDisplay::Date, "15 Jan 2026", "15 Jan"),
        (TimestampDisplay::Time, "09:05", "09:05"),
    ] {
        assert_eq!(timestamp(ms(at), ms(noon()), display, false, UTC_STANDIN), long);
        assert_eq!(timestamp(ms(at), ms(noon()), display, true, UTC_STANDIN), compact);
    }
}

#[test]
fn utc_is_labelled_on_dates_and_times_but_not_on_relative_text() {
    assert_eq!(timestamp(ms(ago(60)), ms(noon()), TimestampDisplay::Time, false, Zone::Utc), "11:59 UTC");
    assert_eq!(timestamp(ms(ago(60)), ms(noon()), TimestampDisplay::Auto, false, Zone::Utc), "1 minute ago");
}

#[test]
fn wall_clock_time_follows_the_zone_offset_across_midnight() {
    let late = Utc.with_ymd_and_hms(2026, 1, 15, 23, 30, 0).unwrap();
    assert_eq!(exact(&wall(ms(late), 0)), "Thursday 15 January 2026, 23:30:00");
    assert_eq!(exact(&wall(ms(late), 60)), "Friday 16 January 2026, 00:30:00");
    assert_eq!(exact(&wall(ms(late), -8 * 60)), "Thursday 15 January 2026, 15:30:00");
}

#[test]
fn the_calendar_agrees_with_chrono() {
    use chrono::{Datelike, Timelike};
    // Leap days, year ends, the epoch, and times before it.
    for (y, mo, d, h, mi, sec) in [
        (1970, 1, 1, 0, 0, 0),
        (1969, 12, 31, 23, 59, 59),
        (1900, 3, 1, 6, 7, 8),
        (2000, 2, 29, 12, 0, 0),
        (2024, 2, 29, 23, 59, 59),
        (2024, 3, 1, 0, 0, 0),
        (2025, 12, 31, 23, 59, 59),
        (2026, 1, 15, 12, 0, 0),
        (2100, 2, 28, 1, 2, 3),
        (2400, 2, 29, 4, 5, 6),
    ] {
        let at = Utc.with_ymd_and_hms(y, mo, d, h, mi, sec).unwrap();
        let w = wall(ms(at), 0);
        assert_eq!((w.year, w.month, w.day, w.hour, w.minute, w.second), (y.into(), mo, d, h, mi, sec));
        assert_eq!(w.weekday, at.weekday().num_days_from_monday(), "{at}");
        assert_eq!(at.nanosecond(), 0);
        assert_eq!(machine_readable(ms(at)), at.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true));
    }
}

#[test]
fn renders_a_time_element_with_the_machine_readable_instant() {
    let html = render_at_noon(|| html! { <Timestamp at={ago(35 * 60)} /> });
    assert!(html.contains("<time"));
    assert!(html.contains(r#"datetime="2026-01-15T11:25:00Z""#));
    assert!(html.contains("35 minutes ago"));
}

#[test]
fn the_server_render_writes_dates_in_utc() {
    let html = render_at_noon(|| html! { <Timestamp at={ago(30 * 86_400)} display={TimestampDisplay::DateTime} /> });
    assert!(html.contains("16 Dec 2025, 12:00"));
}

#[test]
fn the_tooltip_costs_nothing_until_it_is_shown() {
    let html = render_at_noon(|| html! { <Timestamp at={ago(60)} /> });
    assert!(!html.contains(r#"role="tooltip""#));
    assert!(!html.contains("UTC"));
    assert!(!html.contains("tabindex"));
}

#[test]
fn focusable_makes_it_a_tab_stop_only_when_there_is_a_tooltip() {
    let with = render_at_noon(|| html! { <Timestamp at={ago(60)} focusable=true /> });
    let without = render_at_noon(|| html! { <Timestamp at={ago(60)} focusable=true tooltip=false /> });
    assert!(with.contains(r#"tabindex="0""#));
    assert!(!without.contains("tabindex"));
}

#[test]
fn detail_spells_out_local_time_then_utc() {
    let html = render_at_noon(|| html! { <TimestampDetail at={ago(0)} /> });
    let local = html.find("Thursday 15 January 2026, 12:00:00").expect("the local line");
    let utc = html.rfind("12:00:00").expect("the UTC line");
    assert!(local < utc);
    assert!(html[utc..].contains("UTC"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render_at_noon(|| {
        html! { <Timestamp at={ago(60)} id="seen" class="mine" attrs={attrs! { "data-row" => "7" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="seen""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-row="7""#));
}
