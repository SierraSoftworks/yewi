//! Headings: a semantic level with a visual size that can differ from it.

use yew::prelude::*;

use crate::{Attrs, TextSize};

/// The heading's rank in the document outline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum HeadingLevel {
    /// The page title. One per page.
    H1,
    /// A section of the page.
    #[default]
    H2,
    /// A subsection, or the title of a card.
    H3,
    /// A panel or group title.
    H4,
    /// A label at body-text size.
    H5,
    /// A label at body-text size.
    H6,
}

impl HeadingLevel {
    pub const ALL: [HeadingLevel; 6] = [
        HeadingLevel::H1,
        HeadingLevel::H2,
        HeadingLevel::H3,
        HeadingLevel::H4,
        HeadingLevel::H5,
        HeadingLevel::H6,
    ];

    const fn tag(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
            HeadingLevel::H6 => "h6",
        }
    }

    /// The size a heading of this level has unless `size` overrides it.
    pub const fn default_size(self) -> TextSize {
        match self {
            HeadingLevel::H1 => TextSize::Xxxxl,
            HeadingLevel::H2 => TextSize::Xxxl,
            HeadingLevel::H3 => TextSize::Xxl,
            HeadingLevel::H4 => TextSize::Xl,
            HeadingLevel::H5 | HeadingLevel::H6 => TextSize::Md,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct HeadingProps {
    #[prop_or_default]
    pub children: Html,
    /// The rank in the document outline. Choose it for structure, not looks.
    #[prop_or_default]
    pub level: HeadingLevel,
    /// The visual size, when it should differ from the level's default. The
    /// weight follows the size: lighter as it gets larger.
    #[prop_or_default]
    pub size: Option<TextSize>,
    /// Keeps the heading on one line and ends it with an ellipsis.
    #[prop_or_default]
    pub truncate: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A heading in Montserrat whose weight falls as its size rises.
///
/// `level` sets the element (`<h1>` to `<h6>`) and with it the document
/// outline; `size` changes only how large it looks.
#[function_component]
pub fn Heading(props: &HeadingProps) -> Html {
    let size = props.size.unwrap_or(props.level.default_size());
    let class = classes!(
        "yi-heading",
        format!("yi-heading--{}", size.as_str()),
        props.truncate.then_some("yi-heading--truncate"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <@{props.level.tag()} id={props.id.clone()} {class}>
            { props.children.clone() }
        </@>
    })
}

#[cfg(test)]
mod tests;
