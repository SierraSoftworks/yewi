//! Links: navigation inside running text and on its own.

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, icons};

#[cfg(feature = "router")]
mod router;
#[cfg(feature = "router")]
pub use router::*;

/// How much a link stands out from the text around it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum LinkVariant {
    /// The accent colour: the usual link.
    #[default]
    Default,
    /// The colour of the surrounding text, turning to the accent on hover.
    /// For names in tables and lists, where a column of accent is too loud.
    Subtle,
}

impl LinkVariant {
    pub const ALL: [LinkVariant; 2] = [LinkVariant::Default, LinkVariant::Subtle];

    const fn as_str(self) -> &'static str {
        match self {
            LinkVariant::Default => "default",
            LinkVariant::Subtle => "subtle",
        }
    }
}

/// When a link is underlined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum LinkUnderline {
    /// Underlined inside running text (a paragraph, list item or table
    /// cell), where colour alone must not mark a link; elsewhere on hover.
    #[default]
    Auto,
    /// Always underlined.
    Always,
    /// Underlined on hover and keyboard focus only.
    Hover,
}

impl LinkUnderline {
    pub const ALL: [LinkUnderline; 3] = [LinkUnderline::Auto, LinkUnderline::Always, LinkUnderline::Hover];

    const fn as_str(self) -> &'static str {
        match self {
            LinkUnderline::Auto => "auto",
            LinkUnderline::Always => "always",
            LinkUnderline::Hover => "hover",
        }
    }
}

/// The classes shared by [`Link`] and the router-aware link.
pub(crate) fn link_classes(variant: LinkVariant, underline: LinkUnderline, class: &Classes) -> Classes {
    classes!(
        "yi-link",
        format!("yi-link--{}", variant.as_str()),
        format!("yi-link--underline-{}", underline.as_str()),
        class.clone(),
    )
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    /// The link text.
    #[prop_or_default]
    pub children: Html,
    /// Where the link goes. Without it the link renders a `<button>` that
    /// looks the same, for an action that does not navigate.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    #[prop_or_default]
    pub variant: LinkVariant,
    #[prop_or_default]
    pub underline: LinkUnderline,
    /// Opens in a new tab, marks the link with an arrow and tells assistive
    /// technology so. For links that leave the app.
    #[prop_or_default]
    pub external: bool,
    /// An icon shown before the text.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// The link `target`. `external` sets `_blank`.
    #[prop_or_default]
    pub target: Option<AttrValue>,
    /// The link `rel`. Opening a new tab adds `noopener noreferrer`.
    #[prop_or_default]
    pub rel: Option<AttrValue>,
    /// Unavailable: cannot be followed or clicked.
    #[prop_or_default]
    pub disabled: bool,
    /// Fired on activation, before the browser follows `href`. Not fired
    /// while `disabled`.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A text link, or without `href` a button that reads as one.
#[function_component]
pub fn Link(props: &LinkProps) -> Html {
    let disabled = props.disabled;
    let class = link_classes(props.variant, props.underline, &props.class);

    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |event: MouseEvent| {
            if disabled {
                event.prevent_default();
            } else {
                on_click.emit(event);
            }
        })
    };

    let target = props.target.clone().or_else(|| props.external.then(|| "_blank".into()));
    let new_tab = target.as_ref().is_some_and(|target| target == "_blank");
    let rel = match (&props.rel, new_tab) {
        (Some(rel), true) => Some(AttrValue::from(format!("{rel} noopener noreferrer"))),
        (None, true) => Some("noopener noreferrer".into()),
        (rel, false) => rel.clone(),
    };

    let content = html! {
        <>
            if let Some(icon) = props.icon {
                <Icon {icon} class="yi-link__icon" />
            }
            { props.children.clone() }
            if props.external {
                <Icon icon={icons::ARROW_UP_RIGHT} class="yi-link__external" label="(opens in a new tab)" />
            }
        </>
    };

    props.attrs.apply(match &props.href {
        Some(href) => html! {
            <a
                id={props.id.clone()}
                {class}
                href={(!disabled).then(|| href.clone())}
                {target}
                {rel}
                role={disabled.then_some("link")}
                aria-disabled={disabled.then_some("true")}
                {onclick}
            >
                { content }
            </a>
        },
        None => html! {
            <button id={props.id.clone()} {class} type="button" {disabled} {onclick}>
                { content }
            </button>
        },
    })
}

#[cfg(test)]
mod tests;
