//! A coloured state label, so a condition looks the same wherever it appears.

use yew::prelude::*;

use crate::{Attrs, NavTarget, StatusDot, Tone, nav_target::use_nav_link};

#[derive(Properties, PartialEq)]
pub struct PillProps {
    /// The state, in a word or two.
    #[prop_or_default]
    pub children: Html,
    /// What the state means. Tones are semantic, never decoration.
    #[prop_or_default]
    pub tone: Tone,
    /// A solid fill, for the one state on a view that must not be missed.
    #[prop_or_default]
    pub bold: bool,
    /// The state is live or in flight: the dot pulses.
    #[prop_or_default]
    pub active: bool,
    /// Shows the leading dot. Turn it off for a plain label, or to place
    /// your own `StatusDot` or icon as content.
    #[prop_or(true)]
    pub dot: bool,
    /// Renders a link, for a pill that opens what it names: a URL or, with
    /// the `router` feature, `NavTarget::route(..)`.
    #[prop_or_default]
    pub href: NavTarget,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A status pill: a soft tinted label with a leading dot.
#[function_component]
pub fn Pill(props: &PillProps) -> Html {
    let class = classes!(
        "yi-pill",
        format!("yi-pill--{}", props.tone.as_str()),
        props.bold.then_some("yi-pill--bold"),
        props.class.clone(),
    );

    let content = html! {
        <>
            if props.dot {
                <StatusDot active={props.active} />
            }
            { props.children.clone() }
        </>
    };

    let link = use_nav_link(&props.href, Callback::noop());
    props.attrs.apply(match link.href {
        Some(href) => html! {
            <a id={props.id.clone()} {class} {href} target={link.target} rel={link.rel} onclick={link.onclick}>
                { content }
            </a>
        },
        None => html! {
            <span id={props.id.clone()} {class}>{ content }</span>
        },
    })
}

#[cfg(test)]
mod tests;
