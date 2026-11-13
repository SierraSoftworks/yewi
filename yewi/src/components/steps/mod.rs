//! Steps: an ordered sequence with a status per step, as a wizard's progress
//! or the report of a run.

use yew::prelude::*;

use crate::{
    Attrs, Icon, IconData, NavTarget, Orientation, Size, VisuallyHidden, icons, is_empty, nav_target::use_nav_link,
};

/// Where a step stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum StepStatus {
    /// Not reached yet.
    #[default]
    Pending,
    /// Where the user is now. Marked with `aria-current="step"`.
    Current,
    /// Work is in flight on this step.
    Running,
    /// Finished successfully.
    Done,
    /// Finished with an error.
    Failed,
    /// Held: it cannot proceed until something else happens.
    Blocked,
    /// Deliberately not run.
    Skipped,
}

impl StepStatus {
    pub const ALL: [StepStatus; 7] = [
        StepStatus::Pending,
        StepStatus::Current,
        StepStatus::Running,
        StepStatus::Done,
        StepStatus::Failed,
        StepStatus::Blocked,
        StepStatus::Skipped,
    ];

    const fn as_str(self) -> &'static str {
        match self {
            StepStatus::Pending => "pending",
            StepStatus::Current => "current",
            StepStatus::Running => "running",
            StepStatus::Done => "done",
            StepStatus::Failed => "failed",
            StepStatus::Blocked => "blocked",
            StepStatus::Skipped => "skipped",
        }
    }

    /// What assistive technology hears after the title. `Pending` needs no
    /// words and `Current` is carried by `aria-current`.
    const fn announcement(self) -> Option<&'static str> {
        match self {
            StepStatus::Pending | StepStatus::Current => None,
            StepStatus::Running => Some("in progress"),
            StepStatus::Done => Some("completed"),
            StepStatus::Failed => Some("failed"),
            StepStatus::Blocked => Some("blocked"),
            StepStatus::Skipped => Some("skipped"),
        }
    }

    /// The marker's icon. Statuses without one show the step's number.
    const fn icon(self) -> Option<IconData> {
        match self {
            StepStatus::Pending | StepStatus::Current | StepStatus::Running => None,
            StepStatus::Done => Some(icons::CHECK),
            StepStatus::Failed => Some(icons::X),
            StepStatus::Blocked => Some(icons::PAUSE),
            StepStatus::Skipped => Some(icons::MINUS),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct StepsProps {
    /// The [`Step`]s, in order.
    #[prop_or_default]
    pub children: Html,
    #[prop_or(Orientation::Horizontal)]
    pub orientation: Orientation,
    /// Marker and text size. `Small` suits dense reports.
    #[prop_or_default]
    pub size: Size,
    /// Accessible name of the list, e.g. "Checkout progress".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// An ordered list of [`Step`]s joined by a rail. Steps number themselves.
#[function_component]
pub fn Steps(props: &StepsProps) -> Html {
    let class = classes!(
        "yi-steps",
        format!("yi-steps--{}", props.orientation.as_str()),
        format!("yi-steps--{}", props.size.as_str()),
        props.class.clone(),
    );
    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            <ol class="yi-steps__list" aria-label={props.label.clone()}>
                { props.children.clone() }
            </ol>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct StepProps {
    /// The step's name.
    pub title: AttrValue,
    #[prop_or_default]
    pub status: StepStatus,
    /// One line of supporting text under the title.
    #[prop_or_default]
    pub description: Option<AttrValue>,
    /// Short trailing detail next to the title: a duration, a count, a pill.
    #[prop_or_default]
    pub meta: Html,
    /// Detail shown under the step: output, a form, an error.
    #[prop_or_default]
    pub children: Html,
    /// Replaces the marker's number or status icon.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Replaces the status wording read by assistive technology after the
    /// title, e.g. for another language.
    #[prop_or_default]
    pub status_label: Option<AttrValue>,
    /// Makes the title a link to the step: a URL or, with the `router`
    /// feature, `NavTarget::route(..)`.
    #[prop_or_default]
    pub href: NavTarget,
    /// Makes the title a button, for moving between steps. Ignored with `href`.
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One step of a [`Steps`] list.
#[function_component]
pub fn Step(props: &StepProps) -> Html {
    let status = props.status;
    let class = classes!("yi-step", format!("yi-step--{}", status.as_str()), props.class.clone());

    let link = use_nav_link(&props.href, Callback::noop());
    let title = if let Some(href) = link.href {
        html! {
            <a class="yi-step__title yi-step__action" {href} target={link.target} rel={link.rel} onclick={link.onclick}>
                { props.title.clone() }
            </a>
        }
    } else if let Some(on_click) = &props.on_click {
        html! {
            <button type="button" class="yi-step__title yi-step__action" onclick={on_click.clone()}>
                { props.title.clone() }
            </button>
        }
    } else {
        html! { <span class="yi-step__title">{ props.title.clone() }</span> }
    };

    let announcement = props.status_label.clone().or_else(|| status.announcement().map(AttrValue::from));
    let has_meta = !is_empty(&props.meta);
    let has_detail = !is_empty(&props.children);

    props.attrs.apply(html! {
        <li id={props.id.clone()} {class} aria-current={(status == StepStatus::Current).then_some("step")}>
            <span class="yi-step__marker" aria-hidden="true">
                if let Some(icon) = props.icon.or(status.icon()) {
                    <Icon {icon} />
                }
            </span>
            <span class="yi-step__rail" aria-hidden="true" />
            <div class="yi-step__body">
                <div class="yi-step__head">
                    { title }
                    if let Some(announcement) = announcement {
                        <VisuallyHidden>{ ", " }{ announcement }</VisuallyHidden>
                    }
                    if has_meta {
                        <span class="yi-step__meta">{ props.meta.clone() }</span>
                    }
                </div>
                if let Some(description) = &props.description {
                    <p class="yi-step__description">{ description.clone() }</p>
                }
                if has_detail {
                    <div class="yi-step__detail">{ props.children.clone() }</div>
                }
            </div>
        </li>
    })
}

#[cfg(test)]
mod tests;
