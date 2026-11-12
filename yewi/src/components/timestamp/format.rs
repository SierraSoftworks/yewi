//! Time as text. Everything here is a pure function of its arguments, so the
//! same instant, clock and zone always read the same on server and client.
//!
//! Formatting is written out by hand rather than through chrono's `format`,
//! which would add its whole strftime machinery to the wasm.

use super::TimestampDisplay;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;
const MONTH: u64 = 30 * DAY;
const YEAR: u64 = 365 * DAY;

/// The zone absolute times are written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Zone {
    /// UTC, and labelled as such.
    Utc,
    /// The viewer's zone. Until the component has mounted the viewer's zone
    /// is unknown (a server never knows it), so `known` is false and UTC
    /// stands in for it.
    Viewer { known: bool },
}

impl Zone {
    /// Wall-clock time at `at` (milliseconds since the Unix epoch) in this zone.
    pub(crate) fn wall(self, at: i64) -> Wall {
        let offset = match self {
            Zone::Viewer { known: true } => viewer_offset_minutes(at),
            _ => 0,
        };
        wall(at, offset)
    }
}

/// A date and time of day as a calendar and clock show them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Wall {
    pub year: i64,
    /// 1 to 12.
    pub month: u32,
    pub day: u32,
    /// Days since Monday.
    pub weekday: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millis: u32,
}

impl Wall {
    pub(crate) fn same_day(&self, other: &Wall) -> bool {
        (self.year, self.month, self.day) == (other.year, other.month, other.day)
    }
}

/// Wall-clock time at `at` (milliseconds since the Unix epoch),
/// `offset_minutes` east of UTC.
///
/// The calendar arithmetic is done here, not by chrono, whose date types
/// bring lookup tables this never needs.
pub(crate) fn wall(at: i64, offset_minutes: i32) -> Wall {
    let local = at.saturating_add(i64::from(offset_minutes) * 60_000);
    let (days, in_day) = (local.div_euclid(86_400_000), local.rem_euclid(86_400_000) as u32);

    // Days since the epoch to a proleptic Gregorian date, counting years from
    // 1 March so the leap day falls at the end (Howard Hinnant's algorithm).
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * shifted_month + 2) / 5 + 1) as u32;
    let month = (if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 }) as u32;
    let year = year_of_era + era * 400 + i64::from(month <= 2);

    Wall {
        year,
        month,
        day,
        // 1 January 1970 was a Thursday.
        weekday: (days + 3).rem_euclid(7) as u32,
        hour: in_day / 3_600_000,
        minute: in_day / 60_000 % 60,
        second: in_day / 1_000 % 60,
        millis: in_day % 1_000,
    }
}

/// How far east of UTC the viewer's zone is at `at`, daylight saving
/// included. Only a browser knows; everywhere else it is zero.
pub(crate) fn viewer_offset_minutes(at: i64) -> i32 {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(at as f64));
        let offset = -date.get_timezone_offset();
        if offset.is_finite() { offset as i32 } else { 0 }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = at;
        0
    }
}

/// The instant as `<time datetime>` wants it: `2026-01-15T12:00:00Z`.
pub(crate) fn machine_readable(at: i64) -> String {
    let w = wall(at, 0);
    let fraction = if w.millis == 0 { String::new() } else { format!(".{:03}", w.millis) };
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{fraction}Z", w.year, w.month, w.day, w.hour, w.minute, w.second)
}

fn plural(n: u64, unit: &str) -> String {
    if n == 1 { format!("{n} {unit}") } else { format!("{n} {unit}s") }
}

/// `at` relative to `now`: `35 minutes ago`, `in 2 hours`; compact `35m ago`.
pub(crate) fn relative(at: i64, now: i64, compact: bool) -> String {
    // Whole seconds, towards zero.
    let secs = at.saturating_sub(now) / 1_000;
    let (past, abs) = (secs <= 0, secs.unsigned_abs());
    let (n, long, short) = match abs {
        0..MINUTE => {
            return match (compact, past) {
                (true, _) => "now",
                (false, true) => "just now",
                (false, false) => "in a moment",
            }
            .into();
        }
        MINUTE..HOUR => (abs / MINUTE, "minute", "m"),
        HOUR..DAY => (abs / HOUR, "hour", "h"),
        DAY..MONTH => (abs / DAY, "day", "d"),
        MONTH..YEAR => (abs / MONTH, "month", "mo"),
        _ => (abs / YEAR, "year", "y"),
    };
    let magnitude = if compact { format!("{n}{short}") } else { plural(n, long) };
    if past { format!("{magnitude} ago") } else { format!("in {magnitude}") }
}

fn clock(at: &Wall) -> String {
    format!("{:02}:{:02}", at.hour, at.minute)
}

fn month(at: &Wall) -> &'static str {
    MONTHS[(at.month as usize + 11) % 12]
}

fn weekday(at: &Wall) -> &'static str {
    WEEKDAYS[at.weekday as usize % 7]
}

/// `12 Jan 2026`. Compact drops the year when it is the current one.
fn date(at: &Wall, now: &Wall, compact: bool) -> String {
    let (day, month) = (at.day, &month(at)[..3]);
    if compact && at.year == now.year { format!("{day} {month}") } else { format!("{day} {month} {}", at.year) }
}

/// What a [`Timestamp`](super::Timestamp) shows.
pub(crate) fn timestamp(
    at: i64,
    now: i64,
    display: TimestampDisplay,
    compact: bool,
    zone: Zone,
) -> String {
    let age = (at.saturating_sub(now) / 1_000).unsigned_abs();
    let display = match display {
        TimestampDisplay::Auto if age < DAY => TimestampDisplay::Relative,
        TimestampDisplay::Auto if age < 7 * DAY => TimestampDisplay::Auto,
        TimestampDisplay::Auto => TimestampDisplay::Date,
        other => other,
    };
    let (local, today) = (zone.wall(at), zone.wall(now));
    let text = match display {
        TimestampDisplay::Relative => return relative(at, now, compact),
        // Only reached inside the last or next week.
        TimestampDisplay::Auto => {
            let weekday = weekday(&local);
            format!("{} {}", if compact { &weekday[..3] } else { weekday }, clock(&local))
        }
        TimestampDisplay::Date => date(&local, &today, compact),
        TimestampDisplay::Time => clock(&local),
        TimestampDisplay::DateTime => {
            format!("{}{} {}", date(&local, &today, compact), if compact { "" } else { "," }, clock(&local))
        }
    };
    if zone == Zone::Utc { format!("{text} UTC") } else { text }
}

/// The full date and time to the second: `Thursday 15 January 2026, 13:00:00`.
pub(crate) fn exact(at: &Wall) -> String {
    format!("{} {}, {}", weekday(at), exact_date(at), exact_time(at))
}

/// `15 January 2026`
pub(crate) fn exact_date(at: &Wall) -> String {
    format!("{} {} {}", at.day, month(at), at.year)
}

/// `13:00:00`
pub(crate) fn exact_time(at: &Wall) -> String {
    format!("{:02}:{:02}:{:02}", at.hour, at.minute, at.second)
}

/// How a length of time is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Units {
    /// The two largest units: `2m 05s`.
    Two,
    /// The single largest unit: `2m`.
    One,
    /// The two largest units in words: `2 minutes 5 seconds`.
    Words,
}

/// A length of time, to the precision that matters at its size.
pub(crate) fn duration(millis: i64, units: Units) -> String {
    let ms = millis.unsigned_abs();
    let secs = ms / 1_000;
    let sign = if millis < 0 { "-" } else { "" };

    // (major, its unit, minor, its unit)
    let (major, minor) = match secs {
        0..MINUTE => (None, (secs, "s", "second")),
        MINUTE..HOUR => (Some((secs / MINUTE, "m", "minute")), (secs % MINUTE, "s", "second")),
        HOUR..DAY => (Some((secs / HOUR, "h", "hour")), (secs % HOUR / MINUTE, "m", "minute")),
        _ => (Some((secs / DAY, "d", "day")), (secs % DAY / HOUR, "h", "hour")),
    };

    let body = match (major, units) {
        // Under a minute there is one unit, with finer grain when small.
        (None, _) => {
            let words = units == Units::Words;
            match ms {
                0 => if words { "0 seconds".into() } else { "0s".into() },
                1..1_000 => if words { plural(ms, "millisecond") } else { format!("{ms}ms") },
                // Whole seconds stay whole, so a running value reads `5s`, not `5.0s`.
                1_000..10_000 if units != Units::One && !ms.is_multiple_of(1_000) => {
                    let value = format!("{}.{}", ms / 1_000, ms % 1_000 / 100);
                    if words { format!("{value} seconds") } else { format!("{value}s") }
                }
                _ => if words { plural(secs, "second") } else { format!("{secs}s") },
            }
        }
        (Some((n, short, _)), Units::One) => format!("{n}{short}"),
        // Seconds and minutes keep two digits so a running value holds its width.
        (Some((n, short, _)), Units::Two) => match minor {
            (m, unit @ ("s" | "m"), _) => format!("{n}{short} {m:02}{unit}"),
            (m, unit, _) => format!("{n}{short} {m}{unit}"),
        },
        (Some((n, _, long)), Units::Words) => match minor {
            (0, ..) => plural(n, long),
            (m, _, minor_long) => format!("{} {}", plural(n, long), plural(m, minor_long)),
        },
    };
    format!("{sign}{body}")
}

/// A length of time as an ISO 8601 duration, for `<time datetime>`: `PT125S`.
pub(crate) fn iso_duration(millis: i64) -> String {
    let ms = millis.unsigned_abs();
    let sign = if millis < 0 { "-" } else { "" };
    match ms % 1_000 {
        0 => format!("{sign}PT{}S", ms / 1_000),
        fraction => format!("{sign}PT{}.{fraction:03}S", ms / 1_000),
    }
}

/// Time left as a clock: `4:05`, `1:02:30`, and `2d 03:04:05` past a day.
/// Never negative, and a part second counts as a whole one, so the clock
/// reads `0:00` only once the time is up.
pub(crate) fn countdown_clock(millis_left: i64) -> String {
    let secs = whole_seconds_left(millis_left);
    let (d, h, m, s) = (secs / DAY, secs % DAY / HOUR, secs % HOUR / MINUTE, secs % MINUTE);
    match (d, h) {
        (0, 0) => format!("{m}:{s:02}"),
        (0, _) => format!("{h}:{m:02}:{s:02}"),
        _ => format!("{d}d {h:02}:{m:02}:{s:02}"),
    }
}

pub(crate) fn whole_seconds_left(millis_left: i64) -> u64 {
    (millis_left.max(0) as u64).div_ceil(1_000)
}
