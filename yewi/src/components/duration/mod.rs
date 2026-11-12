#![cfg(feature = "chrono")]
//! A length of time: how long something took, or has been running.

use chrono::{DateTime, TimeDelta, Utc};
use yew::prelude::*;

use crate::{
    Attrs, Tick,
    timestamp::format::{self, Units},
    use_now,
};

/// How a [`Duration`] is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum DurationStyle {
    /// The two largest units, with finer grain when small: `230ms`, `1.4s`,
    /// `45s`, `2m 05s`, `1h 02m`, `3d 4h`.
    #[default]
    Short,
    /// The single largest unit, for tight columns: `45s`, `2m`, `1h`, `3d`.
    Compact,
    /// Words, for sentences: `2 minutes 5 seconds`, `1 hour 2 minutes`.
    Long,
}

impl DurationStyle {
    pub const ALL: [DurationStyle; 3] = [DurationStyle::Short, DurationStyle::Compact, DurationStyle::Long];

    const fn units(self) -> Units {
        match self {
            DurationStyle::Short => Units::Two,
            DurationStyle::Compact => Units::One,
            DurationStyle::Long => Units::Words,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DurationProps {
    /// A fixed length of time. Wins over `since` and `until`.
    #[prop_or_default]
    pub value: Option<TimeDelta>,
    /// When the span began. Without `until` the duration is the time elapsed
    /// since then, and keeps counting.
    #[prop_or_default]
    pub since: Option<DateTime<Utc>>,
    /// When the span ended.
    #[prop_or_default]
    pub until: Option<DateTime<Utc>>,
    #[prop_or_default]
    pub style: DurationStyle,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

impl DurationProps {
    /// The fixed length, when the props describe one.
    fn fixed(&self) -> Option<TimeDelta> {
        self.value.or_else(|| Some(self.until?.signed_duration_since(self.since?)))
    }
}

/// How often a running duration's text changes: each second while seconds
/// show, each minute after that.
fn tick_for(elapsed: TimeDelta, style: DurationStyle) -> Tick {
    let shows_seconds = match style {
        DurationStyle::Compact => elapsed.num_minutes() < 1,
        DurationStyle::Short | DurationStyle::Long => elapsed.num_hours() < 1,
    };
    if shows_seconds { Tick::Second } else { Tick::Minute }
}

/// A length of time, written to the precision that matters at its size.
///
/// - Give it a `value`, or the `since` and `until` it spans.
/// - With only `since` it shows the time elapsed and keeps counting, on the
///   timer every time component on the page shares: each second while
///   seconds show, each minute after.
/// - Renders a `<time>` element carrying the ISO 8601 duration.
#[function_component]
pub fn Duration(props: &DurationProps) -> Html {
    let fixed = props.fixed();
    let running_from = if fixed.is_none() { props.since } else { None };
    let tick = running_from.map_or(Tick::Never, |since| {
        tick_for(DateTime::<Utc>::from(crate::Now::system()).signed_duration_since(since), props.style)
    });
    let now: DateTime<Utc> = use_now(tick).into();

    let length = fixed.or_else(|| running_from.map(|since| now.signed_duration_since(since))).unwrap_or_default();
    let millis = length.num_milliseconds();
    // A running duration moves in whole seconds.
    let millis = if running_from.is_some() { millis - millis % 1_000 } else { millis };

    props.attrs.apply(html! {
        <time
            id={props.id.clone()}
            class={classes!("yi-duration", props.class.clone())}
            datetime={format::iso_duration(millis)}
        >
            { format::duration(millis, props.style.units()) }
        </time>
    })
}

#[cfg(test)]
mod tests;
