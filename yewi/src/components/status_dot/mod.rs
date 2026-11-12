//! A small coloured dot that carries a status.

use yew::prelude::*;

use crate::{Attrs, Tone};

#[derive(Properties, PartialEq)]
pub struct StatusDotProps {
    /// The status the dot carries. Unset, the dot takes the colour of the
    /// text around it, which is how it follows a `Pill`.
    #[prop_or_default]
    pub tone: Option<Tone>,
    /// Pulses to show the status is live or in flight. Purely visual, and
    /// still for users who prefer reduced motion.
    #[prop_or_default]
    pub active: bool,
    /// Accessible name. Set it when the dot is the only thing that conveys
    /// the status; leave it unset next to a text label, where the dot is
    /// hidden from assistive technology.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Diameter as a CSS length. Defaults to `0.5rem`.
    #[prop_or_default]
    pub size: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A status dot, on its own beside a label or as the content of a `Pill`.
#[function_component]
pub fn StatusDot(props: &StatusDotProps) -> Html {
    let class = classes!(
        "yi-status-dot",
        props.tone.map(|tone| format!("yi-status-dot--{}", tone.as_str())),
        props.active.then_some("yi-status-dot--pulse"),
        props.class.clone(),
    );
    let style = props.size.as_ref().map(|size| format!("--yi-status-dot-size:{size}"));
    let decorative = props.label.is_none();

    props.attrs.apply(html! {
        <span
            id={props.id.clone()}
            {class}
            {style}
            role={(!decorative).then_some("img")}
            aria-label={props.label.clone()}
            aria-hidden={decorative.then_some("true")}
        />
    })
}

#[cfg(test)]
mod tests;
