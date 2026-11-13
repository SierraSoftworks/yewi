//! What a view says when it has nothing to show: [`EmptyState`] when that is
//! expected, [`ErrorState`] when loading failed.
//!
//! Both are plain content with no surface of their own, so they sit equally
//! well on a page, in a card or in a table body.

use yew::prelude::*;

use crate::{Attrs, Button, Icon, IconData, Size, Tone, icons, is_empty};

/// Where the content of an [`EmptyState`] or [`ErrorState`] sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum EmptyStateAlign {
    /// Centred, for a view or card with nothing else in it.
    #[default]
    Center,
    /// Aligned with the start of the surrounding content, for a section that
    /// sits among others.
    Start,
}

impl EmptyStateAlign {
    const fn as_str(self) -> &'static str {
        match self {
            EmptyStateAlign::Center => "center",
            EmptyStateAlign::Start => "start",
        }
    }
}

struct Parts<'a> {
    role: Option<&'static str>,
    title: &'a AttrValue,
    body: &'a Html,
    icon: Option<IconData>,
    tone: Tone,
    actions: Html,
    align: EmptyStateAlign,
    size: Size,
    class: &'a Classes,
    id: &'a Option<AttrValue>,
    attrs: &'a Attrs,
}

fn view(parts: Parts) -> Html {
    let class = classes!(
        "yi-empty-state",
        format!("yi-empty-state--{}", parts.align.as_str()),
        format!("yi-empty-state--{}", parts.size.as_str()),
        format!("yi-empty-state--{}", parts.tone.as_str()),
        parts.class.clone(),
    );

    parts.attrs.apply(html! {
        <div id={parts.id.clone()} {class} role={parts.role}>
            if let Some(icon) = parts.icon {
                <span class="yi-empty-state__icon"><Icon {icon} /></span>
            }
            <p class="yi-empty-state__title">{ parts.title.clone() }</p>
            if !is_empty(parts.body) {
                <div class="yi-empty-state__body">{ parts.body.clone() }</div>
            }
            if !is_empty(&parts.actions) {
                <div class="yi-empty-state__actions">{ parts.actions }</div>
            }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct EmptyStateProps {
    /// The headline: what is not here, e.g. "No incidents reported".
    pub title: AttrValue,
    /// Supporting copy: whether this is a problem, and what to do about it.
    #[prop_or_default]
    pub children: Html,
    /// An icon shown above the title.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Colours the icon. Use `Ok` when empty is good news.
    #[prop_or_default]
    pub tone: Tone,
    /// Buttons or links for the next step.
    #[prop_or_default]
    pub actions: Html,
    #[prop_or_default]
    pub align: EmptyStateAlign,
    /// `Small` fits a table body or a narrow card, `Large` a whole page.
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A calm end state for a view that has loaded and has nothing to show.
#[function_component]
pub fn EmptyState(props: &EmptyStateProps) -> Html {
    view(Parts {
        role: None,
        title: &props.title,
        body: &props.children,
        icon: props.icon,
        tone: props.tone,
        actions: props.actions.clone(),
        align: props.align,
        size: props.size,
        class: &props.class,
        id: &props.id,
        attrs: &props.attrs,
    })
}

#[derive(Properties, PartialEq)]
pub struct ErrorStateProps {
    /// The headline: what failed, in plain words.
    #[prop_or(AttrValue::Static("Something went wrong"))]
    pub title: AttrValue,
    /// The detail: the error message and what the reader can do.
    #[prop_or_default]
    pub children: Html,
    /// The icon shown above the title.
    #[prop_or(Some(icons::CIRCLE_ALERT))]
    pub icon: Option<IconData>,
    /// Shows a retry button and is fired when it is activated. Leave unset
    /// when trying again cannot change the outcome, such as a refused
    /// permission.
    #[prop_or_default]
    pub on_retry: Option<Callback<()>>,
    /// The retry button's label.
    #[prop_or(AttrValue::Static("Try again"))]
    pub retry_label: AttrValue,
    /// A retry is in flight: the retry button shows as loading.
    #[prop_or_default]
    pub retrying: bool,
    /// Further buttons or links, after the retry button.
    #[prop_or_default]
    pub actions: Html,
    #[prop_or_default]
    pub align: EmptyStateAlign,
    /// `Small` fits a table body or a narrow card, `Large` a whole page.
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A failed load, announced as an alert, with an optional way to retry.
#[function_component]
pub fn ErrorState(props: &ErrorStateProps) -> Html {
    let button_size = if props.size == Size::Small { Size::Small } else { Size::Medium };
    let actions = html! {
        <>
            if let Some(on_retry) = &props.on_retry {
                <Button
                    size={button_size}
                    icon={icons::REFRESH_CW}
                    loading={props.retrying}
                    on_click={on_retry.reform(|_| ())}
                >
                    { props.retry_label.clone() }
                </Button>
            }
            { props.actions.clone() }
        </>
    };
    let actions = if props.on_retry.is_none() && is_empty(&props.actions) { Html::default() } else { actions };

    view(Parts {
        role: Some("alert"),
        title: &props.title,
        body: &props.children,
        icon: props.icon,
        tone: Tone::Danger,
        actions,
        align: props.align,
        size: props.size,
        class: &props.class,
        id: &props.id,
        attrs: &props.attrs,
    })
}

#[cfg(test)]
mod tests;
