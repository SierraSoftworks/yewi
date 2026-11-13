//! [`Divider`]: a thin rule between groups of content.

use yew::prelude::*;

use crate::{Attrs, Orientation, is_empty};

/// Where a divider's label sits along the rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum DividerAlign {
    Start,
    #[default]
    Center,
    End,
}

impl DividerAlign {
    const fn as_str(self) -> &'static str {
        match self {
            DividerAlign::Start => "start",
            DividerAlign::Center => "center",
            DividerAlign::End => "end",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DividerProps {
    /// An optional label set into a horizontal rule, such as "or" or a date.
    #[prop_or_default]
    pub children: Html,
    #[prop_or(Orientation::Horizontal)]
    pub orientation: Orientation,
    /// Where the label sits. Only used with a label.
    #[prop_or_default]
    pub align: DividerAlign,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A rule that separates content. It carries no outer margin: the layout
/// primitive around it owns the spacing.
#[function_component]
pub fn Divider(props: &DividerProps) -> Html {
    let labelled = !is_empty(&props.children);
    let id = props.id.clone();

    props.attrs.apply(match props.orientation {
        Orientation::Vertical => html! {
            <div
                {id}
                class={classes!("yi-divider", "yi-divider--vertical", props.class.clone())}
                role="separator"
                aria-orientation="vertical"
            />
        },
        // The label is content, so a labelled rule is not a bare separator:
        // the separator role would hide its text from assistive technology.
        Orientation::Horizontal if labelled => html! {
            <div
                {id}
                class={classes!(
                    "yi-divider",
                    "yi-divider--labelled",
                    format!("yi-divider--{}", props.align.as_str()),
                    props.class.clone(),
                )}
            >
                <span class="yi-divider__label">{ props.children.clone() }</span>
            </div>
        },
        Orientation::Horizontal => html! {
            <hr {id} class={classes!("yi-divider", props.class.clone())} />
        },
    })
}

#[cfg(test)]
mod tests;
