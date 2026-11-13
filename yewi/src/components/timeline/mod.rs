//! Timeline: events in order along a rail, each with a marker, a title and
//! whatever detail the event carries. A commit log, an incident's updates, an
//! audit trail.

use yew::prelude::*;

use crate::{
    Attrs, Icon, IconData, NavTarget, Orientation, Placement, Popover, PopoverTrigger, Size, Tone, VisuallyHidden,
    is_empty, nav_target::use_nav_link,
};

/// What a [`Timeline`] tells the events inside it.
#[derive(Clone, Copy, PartialEq)]
struct TimelineContext {
    orientation: Orientation,
}

/// Which ends of the rail fade out, saying that time ran on before the first
/// event shown or after the last.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TimelineRunOff {
    /// The rail starts at the first marker and stops at the last.
    None,
    /// The rail fades in before the first event.
    Start,
    /// The rail fades out after the last event.
    End,
    /// Both ends fade.
    #[default]
    Both,
}

impl TimelineRunOff {
    pub const ALL: [TimelineRunOff; 4] =
        [TimelineRunOff::None, TimelineRunOff::Start, TimelineRunOff::End, TimelineRunOff::Both];

    const fn as_str(self) -> &'static str {
        match self {
            TimelineRunOff::None => "none",
            TimelineRunOff::Start => "start",
            TimelineRunOff::End => "end",
            TimelineRunOff::Both => "both",
        }
    }
}

/// The shape of an event's marker. An `icon` on the item replaces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TimelineMarker {
    /// A dot: filled with the item's tone, or an outline when it has none.
    #[default]
    Dot,
    /// An outlined circle in the item's tone: an event that is not settled.
    Ring,
    /// An outlined diamond: an event of another kind than its neighbours,
    /// such as an operator's action among commits.
    Diamond,
}

impl TimelineMarker {
    pub const ALL: [TimelineMarker; 3] = [TimelineMarker::Dot, TimelineMarker::Ring, TimelineMarker::Diamond];

    const fn as_str(self) -> &'static str {
        match self {
            TimelineMarker::Dot => "dot",
            TimelineMarker::Ring => "ring",
            TimelineMarker::Diamond => "diamond",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    /// The [`TimelineItem`]s, in the order they should read.
    #[prop_or_default]
    pub children: Html,
    #[prop_or(Orientation::Vertical)]
    pub orientation: Orientation,
    /// Which ends of the rail fade out.
    #[prop_or_default]
    pub run_off: TimelineRunOff,
    /// Spacing and text size. `Small` suits a dense log.
    #[prop_or_default]
    pub size: Size,
    /// Draws a rule between events. Vertical only.
    #[prop_or_default]
    pub divided: bool,
    /// Accessible name of the list, e.g. "Rollout history".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// An ordered list of [`TimelineItem`]s joined by a rail.
///
/// - The rail below an event takes that event's tone, down to the next one.
/// - The ends of the rail fade out by default; see `run_off`.
/// - Items are plain markup with no state of their own, so a long log costs
///   only its elements.
#[function_component]
pub fn Timeline(props: &TimelineProps) -> Html {
    let class = classes!(
        "yi-timeline",
        format!("yi-timeline--{}", props.orientation.as_str()),
        format!("yi-timeline--{}", props.size.as_str()),
        format!("yi-timeline--run-off-{}", props.run_off.as_str()),
        (props.divided && props.orientation == Orientation::Vertical).then_some("yi-timeline--divided"),
        props.class.clone(),
    );
    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            <ol class="yi-timeline__list" aria-label={props.label.clone()}>
                <ContextProvider<TimelineContext> context={TimelineContext { orientation: props.orientation }}>
                    { props.children.clone() }
                </ContextProvider<TimelineContext>>
            </ol>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct TimelineItemProps {
    /// What happened: a commit's subject, an update's headline.
    pub title: Html,
    /// The event's status. It colours the marker and the rail below it.
    #[prop_or_default]
    pub tone: Option<Tone>,
    /// The tone in words for assistive technology ("resolved", "failed"),
    /// read after the title. Leave it out when the text already says it.
    #[prop_or_default]
    pub tone_label: Option<AttrValue>,
    #[prop_or_default]
    pub marker: TimelineMarker,
    /// Shows an icon in a tinted circle instead of the marker shape.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Pulses the marker: the event is still going on.
    #[prop_or_default]
    pub active: bool,
    /// A muted line under the title: who, when, where.
    #[prop_or_default]
    pub meta: Html,
    /// Trailing content beside the title: a hash, a pill, a button.
    #[prop_or_default]
    pub aside: Html,
    /// Detail under the event: a message, progress, a card.
    #[prop_or_default]
    pub children: Html,
    /// Makes the title a link, and the whole event its target: a URL or,
    /// with the `router` feature, `NavTarget::route(..)`. Controls in the
    /// other slots stay usable.
    #[prop_or_default]
    pub href: NavTarget,
    /// Makes the title a button, and the whole event its target. Ignored
    /// with `href`.
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
    /// The preferred side for the popover of a horizontal timeline.
    #[prop_or_default]
    pub placement: Placement,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One event of a [`Timeline`].
#[function_component]
pub fn TimelineItem(props: &TimelineItemProps) -> Html {
    let orientation = use_context::<TimelineContext>().map_or(Orientation::Vertical, |context| context.orientation);
    let interactive = !props.href.is_empty() || props.on_click.is_some();
    let class = classes!(
        "yi-timeline__item",
        props.tone.map(|tone| format!("yi-timeline__item--toned yi-timeline__item--{}", tone.as_str())),
        props.icon.is_some().then_some("yi-timeline__item--icon"),
        interactive.then_some("yi-timeline__item--interactive"),
        props.class.clone(),
    );
    let marker_class = classes!(
        "yi-timeline__marker",
        props.icon.is_none().then(|| format!("yi-timeline__marker--{}", props.marker.as_str())),
        props.active.then_some("yi-timeline__marker--active"),
    );
    let marker = html! {
        <span class={marker_class} aria-hidden="true">
            if let Some(icon) = props.icon {
                <Icon {icon} />
            }
        </span>
    };

    let link = use_nav_link(&props.href, Callback::noop());
    let title = if let Some(href) = link.href {
        html! {
            <a
                class="yi-timeline__title yi-timeline__action"
                {href}
                target={link.target}
                rel={link.rel}
                onclick={link.onclick}
            >
                { props.title.clone() }
            </a>
        }
    } else if let Some(on_click) = &props.on_click {
        html! {
            <button type="button" class="yi-timeline__title yi-timeline__action" onclick={on_click.clone()}>
                { props.title.clone() }
            </button>
        }
    } else {
        html! { <span class="yi-timeline__title">{ props.title.clone() }</span> }
    };
    let tone_label = props.tone_label.as_ref().map(|label| html! { <VisuallyHidden>{ ", " }{ label.clone() }</VisuallyHidden> });
    let has_meta = !is_empty(&props.meta);
    let has_aside = !is_empty(&props.aside);
    let has_detail = !is_empty(&props.children);

    if orientation == Orientation::Horizontal {
        // The marker is all that shows; the event's text is its popover.
        let header = html! { <>{ title }{ tone_label.clone() }{ props.aside.clone() }</> };
        let (content, footer) =
            if has_detail { (props.children.clone(), props.meta.clone()) } else { (props.meta.clone(), Html::default()) };
        return props.attrs.apply(html! {
            <li id={props.id.clone()} {class}>
                <Popover
                    class="yi-timeline__trigger"
                    trigger={PopoverTrigger::Hover}
                    placement={props.placement}
                    focusable=true
                    arrow=true
                    {header}
                    {content}
                    {footer}
                >
                    { marker }
                    <VisuallyHidden>{ props.title.clone() }{ tone_label }</VisuallyHidden>
                </Popover>
            </li>
        });
    }

    props.attrs.apply(html! {
        <li id={props.id.clone()} {class}>
            { marker }
            <div class="yi-timeline__body">
                <div class="yi-timeline__head">
                    <div class="yi-timeline__main">
                        { title }
                        { tone_label }
                    </div>
                    if has_aside {
                        <div class="yi-timeline__aside">{ props.aside.clone() }</div>
                    }
                </div>
                if has_meta {
                    <div class="yi-timeline__meta">{ props.meta.clone() }</div>
                }
                if has_detail {
                    <div class="yi-timeline__detail">{ props.children.clone() }</div>
                }
            </div>
        </li>
    })
}

#[cfg(test)]
mod tests;
