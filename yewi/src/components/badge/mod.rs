//! A count, or a bare dot, that marks something as needing attention.

use yew::prelude::*;

use crate::{Attrs, Tone, VisuallyHidden, is_empty};

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    /// What the badge is attached to, such as an icon button or an avatar.
    /// The badge sits on its top corner. Without children the badge is
    /// inline, for a tab, a menu row or a heading.
    #[prop_or_default]
    pub children: Html,
    /// The number to show. Unset, the badge is a bare dot.
    #[prop_or_default]
    pub count: Option<u32>,
    /// Counts above this show as "99+".
    #[prop_or(99)]
    pub max: u32,
    /// Keeps the badge visible when `count` is zero.
    #[prop_or_default]
    pub show_zero: bool,
    /// What the count means. Tones are semantic, never decoration.
    #[prop_or_default]
    pub tone: Tone,
    /// A solid fill, for a count that asks for action.
    #[prop_or_default]
    pub bold: bool,
    /// What the badge means to a screen reader, replacing the bare number:
    /// "3 unread notifications". A dot needs one to be announced at all.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

impl BadgeProps {
    /// The text on the badge: `None` for a dot.
    fn text(&self) -> Option<String> {
        self.count.map(|count| if count > self.max { format!("{}+", self.max) } else { count.to_string() })
    }

    fn hidden(&self) -> bool {
        self.count == Some(0) && !self.show_zero
    }
}

/// A count badge or attention dot, inline or attached to its children.
#[function_component]
pub fn Badge(props: &BadgeProps) -> Html {
    let anchored = !is_empty(&props.children);
    let text = props.text();

    let badge = |root: bool| {
        let class = classes!(
            "yi-badge",
            format!("yi-badge--{}", props.tone.as_str()),
            props.bold.then_some("yi-badge--bold"),
            text.is_none().then_some("yi-badge--dot"),
            anchored.then_some("yi-badge--corner"),
            root.then(|| props.class.clone()),
        );
        let id = root.then(|| props.id.clone()).flatten();
        match (&text, &props.label) {
            (Some(text), Some(label)) => html! {
                <span {id} {class}>
                    <span aria-hidden="true">{ text }</span>
                    <VisuallyHidden>{ label.clone() }</VisuallyHidden>
                </span>
            },
            (Some(text), None) => html! { <span {id} {class}>{ text }</span> },
            (None, Some(label)) => html! { <span {id} {class}><VisuallyHidden>{ label.clone() }</VisuallyHidden></span> },
            (None, None) => html! { <span {id} {class} aria-hidden="true" /> },
        }
    };

    if !anchored {
        return if props.hidden() { Html::default() } else { props.attrs.apply(badge(true)) };
    }

    props.attrs.apply(html! {
        <span id={props.id.clone()} class={classes!("yi-badge-anchor", props.class.clone())}>
            { props.children.clone() }
            if !props.hidden() {
                { badge(false) }
            }
        </span>
    })
}

#[cfg(test)]
mod tests;
