//! Placeholders in the silhouette of content that is still loading.

use yew::prelude::*;

use crate::Attrs;

/// What a [`Skeleton`] stands in for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SkeletonShape {
    /// Lines of text. Each takes exactly one line of the surrounding type, so
    /// nothing moves when the real text arrives.
    #[default]
    Text,
    /// A rectangle: an image, a chart, a control.
    Block,
    /// A circle: an avatar or a status marker.
    Circle,
}

impl SkeletonShape {
    pub const ALL: [SkeletonShape; 3] = [SkeletonShape::Text, SkeletonShape::Block, SkeletonShape::Circle];

    const fn as_str(self) -> &'static str {
        match self {
            SkeletonShape::Text => "text",
            SkeletonShape::Block => "block",
            SkeletonShape::Circle => "circle",
        }
    }
}

/// How a [`Skeleton`] signals that content is on its way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SkeletonAnimation {
    /// A highlight travels across the placeholder.
    #[default]
    Shimmer,
    /// The placeholder fades in and out.
    Pulse,
    /// Static, for placeholders that stay on screen.
    None,
}

impl SkeletonAnimation {
    pub const ALL: [SkeletonAnimation; 3] =
        [SkeletonAnimation::Shimmer, SkeletonAnimation::Pulse, SkeletonAnimation::None];

    const fn as_str(self) -> &'static str {
        match self {
            SkeletonAnimation::Shimmer => "shimmer",
            SkeletonAnimation::Pulse => "pulse",
            SkeletonAnimation::None => "static",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SkeletonProps {
    #[prop_or_default]
    pub shape: SkeletonShape,
    /// Number of lines, for `Text`. The last of several is shorter.
    #[prop_or(1)]
    pub lines: u32,
    /// Width as a CSS length. `Text` and `Block` fill their container by
    /// default; for `Circle` this is the diameter.
    #[prop_or_default]
    pub width: Option<AttrValue>,
    /// Height as a CSS length, for `Block`.
    #[prop_or_default]
    pub height: Option<AttrValue>,
    #[prop_or_default]
    pub animation: SkeletonAnimation,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A placeholder for content that is loading. It is hidden from assistive
/// technology: mark the region it sits in `aria-busy` and announce the wait
/// once, for example with a visually hidden status.
#[function_component]
pub fn Skeleton(props: &SkeletonProps) -> Html {
    let class = classes!(
        "yi-skeleton",
        format!("yi-skeleton--{}", props.shape.as_str()),
        format!("yi-skeleton--{}", props.animation.as_str()),
        props.class.clone(),
    );
    let style = [("--yi-skeleton-width", &props.width), ("--yi-skeleton-height", &props.height)]
        .into_iter()
        .filter_map(|(name, value)| value.as_ref().map(|value| format!("{name}:{value};")))
        .collect::<String>();
    let style = (!style.is_empty()).then_some(style);
    let lines = if props.shape == SkeletonShape::Text { props.lines.max(1) } else { 1 };

    props.attrs.apply(html! {
        <span id={props.id.clone()} {class} {style} aria-hidden="true">
            { for (0..lines).map(|_| html! { <span class="yi-skeleton__bone" /> }) }
        </span>
    })
}

#[cfg(test)]
mod tests;
