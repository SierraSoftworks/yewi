//! Content for assistive technology only.

use yew::prelude::*;

use crate::Attrs;

#[derive(Properties, PartialEq)]
pub struct VisuallyHiddenProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Hides its children visually while keeping them available to screen
/// readers, for labels that sighted users get from context.
#[function_component]
pub fn VisuallyHidden(props: &VisuallyHiddenProps) -> Html {
    props.attrs.apply(html! {
        <span id={props.id.clone()} class={classes!("yi-visually-hidden", props.class.clone())}>
            { props.children.clone() }
        </span>
    })
}
