//! BarSegment bars: how a whole (a rollout's hosts, a quota, the fleet) splits
//! into states, readable at a glance.

use std::rc::Rc;

use yew::prelude::*;

use crate::{Attrs, Size, Tone, Tooltip};

/// One coloured part of a [`SegmentBar`].
#[derive(Debug, Clone, PartialEq)]
pub struct BarSegment {
    /// The segment's share of the whole, in the same unit as `total`.
    pub value: f64,
    /// What the segment means. `Tone::Active` is drawn with a moving sheen.
    pub tone: Tone,
    /// What the value counts: "done", "failed".
    pub label: AttrValue,
}

impl BarSegment {
    pub fn new(value: f64, tone: Tone, label: impl Into<AttrValue>) -> Self {
        Self { value, tone, label: label.into() }
    }
}

#[derive(Properties, PartialEq)]
pub struct SegmentBarProps {
    /// The segments, in display order. Segments without a positive value are
    /// left out.
    pub segments: Rc<[BarSegment]>,
    /// The whole. What the segments leave of it is drawn as empty track.
    /// Without one, the segments fill the bar.
    #[prop_or_default]
    pub total: Option<f64>,
    /// What the remainder is called: "to go", "free".
    #[prop_or(AttrValue::Static("remaining"))]
    pub rest_label: AttrValue,
    /// What the bar measures, for assistive technology: "Hosts".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Lists each segment's value and label under the bar.
    #[prop_or_default]
    pub legend: bool,
    /// The bar's thickness.
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A value as people write it: no trailing `.0`, one decimal otherwise.
fn number(value: f64) -> String {
    if value.fract() == 0.0 { format!("{value:.0}") } else { format!("{value:.1}") }
}

/// A thin bar divided in proportion to its segments.
///
/// - The bar is one image to assistive technology, named by `label` and a
///   summary of every part: "Hosts: 12 done, 3 failed, 5 to go".
/// - Hovering a segment shows its value and label; `legend` lists them all
///   under the bar.
/// - Every segment stays wide enough to see, however small its share.
#[function_component]
pub fn SegmentBar(props: &SegmentBarProps) -> Html {
    let shown: Vec<&BarSegment> = props.segments.iter().filter(|s| s.value > 0.0).collect();
    let used: f64 = shown.iter().map(|s| s.value).sum();
    let rest = props.total.map_or(0.0, |total| (total - used).max(0.0));

    let mut parts: Vec<String> = shown.iter().map(|s| format!("{} {}", number(s.value), s.label)).collect();
    if rest > 0.0 {
        parts.push(format!("{} {}", number(rest), props.rest_label));
    }
    let summary = match (&props.label, parts.is_empty()) {
        (Some(label), true) => label.to_string(),
        (Some(label), false) => format!("{label}: {}", parts.join(", ")),
        (None, _) => parts.join(", "),
    };

    let class = classes!("yi-segment-bar", format!("yi-segment-bar--{}", props.size.as_str()), props.class.clone());

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            <span class="yi-segment-bar__track" role="img" aria-label={summary}>
                { for shown.iter().zip(&parts).map(|(segment, part)| html! {
                    <Tooltip
                        tip={part.clone()}
                        class={classes!("yi-segment-bar__segment", format!("yi-segment-bar__segment--{}", segment.tone.as_str()))}
                        attrs={crate::attrs! { "style" => format!("flex-grow: {}", segment.value) }}
                    />
                }) }
                if rest > 0.0 {
                    <span class="yi-segment-bar__rest" style={format!("flex-grow: {rest}")} />
                }
            </span>
            if props.legend {
                <ul class="yi-segment-bar__legend" aria-hidden="true">
                    { for shown.iter().map(|segment| html! {
                        <li class="yi-segment-bar__entry">
                            <span class={classes!("yi-segment-bar__swatch", format!("yi-segment-bar__swatch--{}", segment.tone.as_str()))} />
                            <span class="yi-segment-bar__value">{ number(segment.value) }</span>
                            { segment.label.clone() }
                        </li>
                    }) }
                    if rest > 0.0 {
                        <li class="yi-segment-bar__entry">
                            <span class="yi-segment-bar__swatch yi-segment-bar__swatch--rest" />
                            <span class="yi-segment-bar__value">{ number(rest) }</span>
                            { props.rest_label.clone() }
                        </li>
                    }
                </ul>
            }
        </div>
    })
}

#[cfg(test)]
mod tests;
