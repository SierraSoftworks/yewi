//! Waiting indicators: [`Spinner`], [`Loading`] and [`LoadingOverlay`].
//!
//! Use these when the shape of what is coming is unknown. When it is known,
//! a [`Skeleton`](crate::Skeleton) reserves the layout instead.

use yew::prelude::*;

use crate::{Attrs, Size};

#[derive(Properties, PartialEq)]
pub struct SpinnerProps {
    /// Diameter as a CSS length. Defaults to `1em`, so the spinner follows
    /// the text it sits in.
    #[prop_or_default]
    pub size: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The bare rotating ring. It is decorative and hidden from assistive
/// technology: use [`Loading`] unless something else announces the wait.
#[function_component]
pub fn Spinner(props: &SpinnerProps) -> Html {
    let style = props.size.as_ref().map(|size| format!("--yi-spinner-size:{size}"));

    props.attrs.apply(html! {
        <svg
            id={props.id.clone()}
            class={classes!("yi-spinner", props.class.clone())}
            {style}
            viewBox="0 0 24 24"
            aria-hidden="true"
            focusable="false"
        >
            <circle class="yi-spinner__track" cx="12" cy="12" r="9" />
            <path class="yi-spinner__arc" d="M12 3a9 9 0 0 1 9 9" />
        </svg>
    })
}

/// How a [`Loading`] indicator occupies its container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum LoadingLayout {
    /// Spinner and label side by side, in the flow of the content.
    #[default]
    Inline,
    /// Centred in a padded block, standing in for a whole section or page.
    Block,
}

impl LoadingLayout {
    const fn as_str(self) -> &'static str {
        match self {
            LoadingLayout::Inline => "inline",
            LoadingLayout::Block => "block",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    /// What is being waited for. Always announced to assistive technology.
    #[prop_or(AttrValue::Static("Loading"))]
    pub label: AttrValue,
    /// Shows only the spinner. The label is still announced.
    #[prop_or_default]
    pub label_hidden: bool,
    #[prop_or_default]
    pub layout: LoadingLayout,
    #[prop_or_default]
    pub size: Size,
    /// Stays invisible for `--yi-loading-delay`, so a fast response never
    /// flashes a spinner.
    #[prop_or_default]
    pub delay: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A spinner with a label, announced politely as a status.
#[function_component]
pub fn Loading(props: &LoadingProps) -> Html {
    let class = classes!(
        "yi-loading",
        format!("yi-loading--{}", props.layout.as_str()),
        format!("yi-loading--{}", props.size.as_str()),
        props.delay.then_some("yi-loading--delay"),
        props.class.clone(),
    );
    let label_class = if props.label_hidden { "yi-loading__label yi-visually-hidden" } else { "yi-loading__label" };

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} role="status">
            <Spinner />
            <span class={label_class}>{ props.label.clone() }</span>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct LoadingOverlayProps {
    /// The content being refreshed. It stays mounted and keeps its size.
    #[prop_or_default]
    pub children: Html,
    /// Whether the content is being refreshed.
    #[prop_or_default]
    pub active: bool,
    /// What is being waited for.
    #[prop_or(AttrValue::Static("Loading"))]
    pub label: AttrValue,
    /// Shows only the spinner. The label is still announced.
    #[prop_or_default]
    pub label_hidden: bool,
    /// Size of the indicator.
    #[prop_or_default]
    pub size: Size,
    /// Waits `--yi-loading-delay` before dimming, so a fast refresh never
    /// flashes.
    #[prop_or_default]
    pub delay: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Dims content that is being refreshed and centres a [`Loading`] indicator
/// over it. While `active` the content is `inert`: it cannot be clicked,
/// focused or read, and the region reports `aria-busy`.
#[function_component]
pub fn LoadingOverlay(props: &LoadingOverlayProps) -> Html {
    let class = classes!("yi-loading-overlay", props.delay.then_some("yi-loading-overlay--delay"), props.class.clone());

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} aria-busy={props.active.then_some("true")}>
            <div inert={props.active}>
                { props.children.clone() }
            </div>
            if props.active {
                <div class="yi-loading-overlay__scrim">
                    <Loading
                        label={props.label.clone()}
                        label_hidden={props.label_hidden}
                        size={props.size}
                    />
                </div>
            }
        </div>
    })
}

#[cfg(test)]
mod tests;
