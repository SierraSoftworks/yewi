//! Text: body copy with a size, colour, weight and overflow behaviour.

use yew::prelude::*;

use crate::{Attrs, Tone};

/// A step on the type scale, `--yi-text-xs` to `--yi-text-4xl`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextSize {
    /// 12px: captions and eyebrows.
    Xs,
    /// 13px: secondary detail.
    Sm,
    /// 14px: controls and dense UI.
    Md,
    /// 15px: longer reading text.
    Lg,
    /// 17px.
    Xl,
    /// 20px.
    Xxl,
    /// 24px.
    Xxxl,
    /// 30px.
    Xxxxl,
}

impl TextSize {
    pub const ALL: [TextSize; 8] = [
        TextSize::Xs,
        TextSize::Sm,
        TextSize::Md,
        TextSize::Lg,
        TextSize::Xl,
        TextSize::Xxl,
        TextSize::Xxxl,
        TextSize::Xxxxl,
    ];

    /// The token and BEM modifier suffix, e.g. `2xl` for `--yi-text-2xl`.
    pub const fn as_str(self) -> &'static str {
        match self {
            TextSize::Xs => "xs",
            TextSize::Sm => "sm",
            TextSize::Md => "md",
            TextSize::Lg => "lg",
            TextSize::Xl => "xl",
            TextSize::Xxl => "2xl",
            TextSize::Xxxl => "3xl",
            TextSize::Xxxxl => "4xl",
        }
    }
}

/// Where text sits in the neutral hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TextColor {
    /// The colour of the surrounding text.
    #[default]
    Inherit,
    /// Primary text, `--yi-fg`.
    Primary,
    /// Secondary text such as descriptions, `--yi-fg-muted`.
    Muted,
    /// Tertiary text such as captions and placeholders, `--yi-fg-subtle`.
    Subtle,
    /// The app's accent, `--yi-accent-text`.
    Accent,
}

impl TextColor {
    pub const ALL: [TextColor; 5] =
        [TextColor::Inherit, TextColor::Primary, TextColor::Muted, TextColor::Subtle, TextColor::Accent];

    const fn as_str(self) -> Option<&'static str> {
        match self {
            TextColor::Inherit => None,
            TextColor::Primary => Some("primary"),
            TextColor::Muted => Some("muted"),
            TextColor::Subtle => Some("subtle"),
            TextColor::Accent => Some("accent"),
        }
    }
}

/// Font weight. Emphasis stops at semibold; text is never bold for attention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextWeight {
    Regular,
    Medium,
    Semibold,
}

impl TextWeight {
    pub const ALL: [TextWeight; 3] = [TextWeight::Regular, TextWeight::Medium, TextWeight::Semibold];

    const fn as_str(self) -> &'static str {
        match self {
            TextWeight::Regular => "regular",
            TextWeight::Medium => "medium",
            TextWeight::Semibold => "semibold",
        }
    }
}

/// The element a [`Text`] renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TextTag {
    /// Inline text.
    #[default]
    Span,
    /// A paragraph.
    P,
    /// A block without paragraph semantics.
    Div,
    /// Strong importance; rendered semibold unless `weight` is set.
    Strong,
    /// Stress emphasis, in italics.
    Em,
    /// Side comments and small print; one size smaller unless `size` is set.
    Small,
}

impl TextTag {
    const fn as_str(self) -> &'static str {
        match self {
            TextTag::Span => "span",
            TextTag::P => "p",
            TextTag::Div => "div",
            TextTag::Strong => "strong",
            TextTag::Em => "em",
            TextTag::Small => "small",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TextProps {
    #[prop_or_default]
    pub children: Html,
    /// The element to render.
    #[prop_or_default]
    pub tag: TextTag,
    /// A step on the type scale. Unset inherits the surrounding size.
    #[prop_or_default]
    pub size: Option<TextSize>,
    /// Where the text sits in the neutral hierarchy.
    #[prop_or_default]
    pub color: TextColor,
    /// A status colour, for text that reports a state. Wins over `color`.
    #[prop_or_default]
    pub tone: Option<Tone>,
    /// Unset inherits the surrounding weight.
    #[prop_or_default]
    pub weight: Option<TextWeight>,
    /// Keeps the text on one line and ends it with an ellipsis. The full text
    /// stays available to assistive technology; add a `title` through `attrs`
    /// if sighted users need it too.
    #[prop_or_default]
    pub truncate: bool,
    /// Clamps the text to this many lines, ending with an ellipsis.
    #[prop_or_default]
    pub lines: Option<u8>,
    /// Sets the text in the code font, for identifiers, hashes and addresses.
    #[prop_or_default]
    pub mono: bool,
    /// Fixed-width digits, so numbers line up in columns and do not shift
    /// as they change.
    #[prop_or_default]
    pub tabular: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A run of text placed on the type scale and in the colour hierarchy.
///
/// With no props it renders a plain `<span>` that inherits everything, so it
/// is safe to wrap any text and adjust one aspect at a time.
#[function_component]
pub fn Text(props: &TextProps) -> Html {
    let lines = props.lines.filter(|lines| *lines > 0 && !props.truncate);

    let class = classes!(
        "yi-text",
        props.size.map(|size| format!("yi-text--{}", size.as_str())),
        match props.tone {
            Some(tone) => Some(format!("yi-text--{}", tone.as_str())),
            None => props.color.as_str().map(|color| format!("yi-text--{color}")),
        },
        props.weight.map(|weight| format!("yi-text--{}", weight.as_str())),
        props.truncate.then_some("yi-text--truncate"),
        lines.map(|_| "yi-text--clamp"),
        props.mono.then_some("yi-text--mono"),
        props.tabular.then_some("yi-text--tabular"),
        props.class.clone(),
    );
    let style = lines.map(|lines| format!("--yi-text-lines:{lines}"));

    props.attrs.apply(html! {
        <@{props.tag.as_str()} id={props.id.clone()} {class} {style}>
            { props.children.clone() }
        </@>
    })
}

#[cfg(test)]
mod tests;
