//! Inline code: a short piece of code inside a sentence.

use yew::prelude::*;

use crate::Attrs;

#[derive(Properties, PartialEq)]
pub struct InlineCodeProps {
    /// The code, as text.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A command, identifier, path or value quoted inside running text.
///
/// It scales with the text around it and wraps at any character, so a long
/// identifier never forces its container to scroll.
#[function_component]
pub fn InlineCode(props: &InlineCodeProps) -> Html {
    props.attrs.apply(html! {
        <code id={props.id.clone()} class={classes!("yi-inline-code", props.class.clone())}>
            { props.children.clone() }
        </code>
    })
}

#[cfg(test)]
mod tests;
