#![cfg(feature = "chrono")]
//! The time left until a moment, counting down by itself.

use chrono::{DateTime, Utc};
use yew::prelude::*;

use crate::{
    Attrs, Placement, Tick, TimestampDetail, Tooltip,
    timestamp::format::{self, Units},
    use_now,
};

/// How a [`Countdown`] is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CountdownFormat {
    /// A clock: `4:05`, `1:02:30`, `2d 03:04:05`.
    #[default]
    Clock,
    /// Units, as [`Duration`](crate::Duration) writes them: `4m 05s`,
    /// `1h 02m`.
    Units,
}

impl CountdownFormat {
    pub const ALL: [CountdownFormat; 2] = [CountdownFormat::Clock, CountdownFormat::Units];
}

#[derive(Properties, PartialEq)]
pub struct CountdownProps {
    /// The moment being counted down to.
    pub until: DateTime<Utc>,
    #[prop_or_default]
    pub format: CountdownFormat,
    /// What to show once the moment has passed, in place of a zero.
    #[prop_or_default]
    pub done: Option<Html>,
    /// Called once when the countdown reaches zero while on screen. Not
    /// called for a moment that had already passed when it was first shown.
    #[prop_or_default]
    pub on_done: Callback<()>,
    /// Shows the exact target time on hover: local time, then UTC.
    #[prop_or(true)]
    pub tooltip: bool,
    /// Makes the countdown a tab stop so keyboard users can reach the target
    /// time.
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

/// The time left until a moment.
///
/// - Counts down by itself on the timer every time component on the page
///   shares, so the page around it does not re-render each second. Once the
///   moment has passed it stops updating.
/// - Never goes negative: at zero it shows `done`, or a zero.
/// - Renders a `<time>` element with the `timer` role. Screen readers do not
///   announce each second; announce the outcome yourself from `on_done`.
#[function_component]
pub fn Countdown(props: &CountdownProps) -> Html {
    let until = props.until.timestamp_millis();
    // Past the target nothing changes; this only picks how often to look.
    let pending = (crate::Now::system().epoch_millis() as i64) < until;
    let now = use_now(if pending { Tick::Second } else { Tick::Never }).epoch_millis() as i64;

    let left = until - now;
    let done = left <= 0;

    {
        let on_done = props.on_done.clone();
        let was_pending = use_mut_ref(|| false);
        use_effect_with((done, until), move |&(done, _)| {
            if !done {
                *was_pending.borrow_mut() = true;
            } else if was_pending.replace(false) {
                on_done.emit(());
            }
        });
    }

    let text = match (&props.done, done) {
        (Some(done), true) => done.clone(),
        _ => match props.format {
            CountdownFormat::Clock => format::countdown_clock(left),
            CountdownFormat::Units => {
                format::duration(format::whole_seconds_left(left) as i64 * 1_000, Units::Two)
            }
        }
        .into(),
    };
    let tip = if props.tooltip { html! { <TimestampDetail at={props.until} /> } } else { Html::default() };

    html! {
        <Tooltip
            id={props.id.clone()}
            class={classes!("yi-countdown", done.then_some("yi-countdown--done"), props.class.clone())}
            attrs={props.attrs.clone()}
            {tip}
            placement={props.placement}
            focusable={props.focusable && props.tooltip}
        >
            <time role="timer" datetime={format::machine_readable(until)}>{ text }</time>
        </Tooltip>
    }
}

#[cfg(test)]
mod tests;
