#![cfg(feature = "chrono")]
//! A point in time, shown relative to now or as a date, with the exact time
//! on hover.

/// The chrono version yewi's time components are built against.
pub use chrono;
use chrono::{DateTime, SecondsFormat, Utc};
use yew::prelude::*;

use crate::{Attrs, Now, Pill, Placement, Tick, Tooltip, use_now};

pub(crate) mod format;

use format::Zone;

impl From<Now> for DateTime<Utc> {
    fn from(now: Now) -> Self {
        format::to_utc(now)
    }
}

/// What a [`Timestamp`] shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TimestampDisplay {
    /// Relative within a day either side of now (`35 minutes ago`), then the
    /// weekday and time within a week (`Tuesday 14:05`), then the date
    /// (`12 Jan 2026`).
    #[default]
    Auto,
    /// Always relative to now: `35 minutes ago`, `in 3 days`.
    Relative,
    /// The date and time: `12 Jan 2026, 14:05`.
    DateTime,
    /// The date: `12 Jan 2026`.
    Date,
    /// The time of day: `14:05`.
    Time,
}

impl TimestampDisplay {
    pub const ALL: [TimestampDisplay; 5] = [
        TimestampDisplay::Auto,
        TimestampDisplay::Relative,
        TimestampDisplay::DateTime,
        TimestampDisplay::Date,
        TimestampDisplay::Time,
    ];
}

/// True once the component has mounted in a browser, which is when the
/// viewer's time zone becomes known. The server render and the render that
/// hydrates it both see `false`, so they agree.
#[hook]
pub(crate) fn use_viewer_zone(utc: bool) -> Zone {
    let mounted = use_state(|| false);
    {
        let mounted = mounted.clone();
        use_effect_with((), move |()| mounted.set(true));
    }
    if utc { Zone::Utc } else { Zone::Viewer { known: *mounted } }
}

/// How often a timestamp's text can change.
fn tick_for(at: DateTime<Utc>, display: TimestampDisplay) -> Tick {
    match display {
        TimestampDisplay::DateTime | TimestampDisplay::Date | TimestampDisplay::Time => Tick::Never,
        TimestampDisplay::Relative => Tick::Minute,
        // More than a week old, it is a date and stays one. This only picks
        // how often to look again, so the real clock is good enough.
        TimestampDisplay::Auto => {
            let age = DateTime::<Utc>::from(Now::system()).signed_duration_since(at);
            if age.num_days() > 7 { Tick::Never } else { Tick::Minute }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TimestampProps {
    /// The moment to show.
    pub at: DateTime<Utc>,
    #[prop_or_default]
    pub display: TimestampDisplay,
    /// Abbreviates for tables and other dense layouts: `35m ago`,
    /// `Tue 14:05`, and no year on dates in the current year.
    #[prop_or_default]
    pub compact: bool,
    /// Writes dates and times in UTC, marked as such, instead of the viewer's
    /// time zone.
    #[prop_or_default]
    pub utc: bool,
    /// Shows the exact time on hover: the viewer's local time, then UTC.
    #[prop_or(true)]
    pub tooltip: bool,
    /// Makes the timestamp a tab stop so keyboard users can reach the exact
    /// time. Leave it off in long lists, where it would add a stop per row.
    #[prop_or_default]
    pub focusable: bool,
    /// The preferred side for the tooltip.
    #[prop_or_default]
    pub placement: Placement,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A point in time, relative to now while it is recent and as a date once it
/// is not, with the exact time in a tooltip.
///
/// - Relative text keeps itself current. Every timestamp on the page shares
///   one timer and re-renders only on the minute; dates never re-render.
/// - Dates and times are in the viewer's time zone. A server does not know
///   it, so server-rendered dates are in UTC until the page hydrates;
///   relative text is the same on both sides. Wrap the app in a
///   [`NowProvider`](crate::NowProvider) so both sides also agree on "now".
/// - Renders a `<time>` element carrying the machine-readable instant.
#[function_component]
pub fn Timestamp(props: &TimestampProps) -> Html {
    let zone = use_viewer_zone(props.utc);
    let now = use_now(tick_for(props.at, props.display));

    let text = format::timestamp(props.at, now.into(), props.display, props.compact, zone);
    let tip = if props.tooltip { html! { <TimestampDetail at={props.at} /> } } else { Html::default() };

    html! {
        <Tooltip
            id={props.id.clone()}
            class={classes!("yi-timestamp", props.class.clone())}
            attrs={props.attrs.clone()}
            {tip}
            placement={props.placement}
            focusable={props.focusable && props.tooltip}
        >
            <time datetime={machine_readable(props.at)}>{ text }</time>
        </Tooltip>
    }
}

/// The instant as `<time datetime>` wants it: `2026-01-15T12:00:00Z`.
pub(crate) fn machine_readable(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::AutoSi, true)
}

#[derive(Properties, PartialEq)]
pub struct TimestampDetailProps {
    /// The moment to spell out.
    pub at: DateTime<Utc>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A moment in full, to the second: the viewer's local date and time, and
/// under it the same moment in UTC, marked with a `UTC` pill.
///
/// This is what a [`Timestamp`]'s tooltip shows. Use it directly where the
/// exact time should always be visible, such as a detail pane or a popover.
#[function_component]
pub fn TimestampDetail(props: &TimestampDetailProps) -> Html {
    let zone = use_viewer_zone(false);
    let (local, utc) = (zone.wall(props.at), Zone::Utc.wall(props.at));
    // The UTC line repeats the date only when it is a different day there.
    let utc_text = if local.date() == utc.date() {
        format::exact_time(&utc)
    } else {
        format!("{}, {}", format::exact_date(&utc), format::exact_time(&utc))
    };

    props.attrs.apply(html! {
        <span id={props.id.clone()} class={classes!("yi-timestamp-detail", props.class.clone())}>
            <span class="yi-timestamp-detail__local">{ format::exact(&local) }</span>
            <span class="yi-timestamp-detail__utc">
                { utc_text }
                <Pill dot=false>{ "UTC" }</Pill>
            </span>
        </span>
    })
}

#[cfg(test)]
mod tests;
