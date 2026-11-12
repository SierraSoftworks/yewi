//! [`Card`]: the lifted surface that groups one subject's worth of content.

use yew::prelude::*;

use crate::Attrs;

/// How a card's header is told apart from its body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CardHeader {
    /// The header flows straight into the body, separated by space alone.
    #[default]
    Plain,
    /// A hairline rule under the header.
    Ruled,
    /// The header sits on a tonal fill.
    Tonal,
}

impl CardHeader {
    pub const ALL: [CardHeader; 3] = [CardHeader::Plain, CardHeader::Ruled, CardHeader::Tonal];

    const fn as_str(self) -> &'static str {
        match self {
            CardHeader::Plain => "plain",
            CardHeader::Ruled => "ruled",
            CardHeader::Tonal => "tonal",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    /// The body. A body made only of [`Panel`](crate::Panel)s is laid out
    /// flush automatically.
    #[prop_or_default]
    pub children: Html,
    /// The heading. Without one the card is a bare surface.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// A supporting line under the heading.
    #[prop_or_default]
    pub subtitle: Option<AttrValue>,
    /// Controls at the end of the heading row: the action the card exists
    /// for, or a link out of it.
    #[prop_or_default]
    pub actions: Html,
    /// A closing strip on a tonal fill, for form buttons or a summary line.
    #[prop_or_default]
    pub footer: Html,
    /// How the header is separated from the body.
    #[prop_or_default]
    pub header: CardHeader,
    /// Removes the body padding, for content that runs edge to edge such as
    /// a table or a list.
    #[prop_or_default]
    pub flush: bool,
    /// The heading level of the title, 1 to 6.
    #[prop_or(2)]
    pub level: u8,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A lifted surface with an optional heading row, body and footer.
#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let empty = Html::default();
    let has_header = props.title.is_some() || props.subtitle.is_some() || props.actions != empty;

    let class = classes!(
        "yi-card",
        has_header.then(|| format!("yi-card--header-{}", props.header.as_str())),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <section id={props.id.clone()} {class}>
            if has_header {
                <header class="yi-card__header">
                    <div class="yi-card__heading">
                        if let Some(title) = &props.title {
                            <@{format!("h{}", props.level.clamp(1, 6))} class="yi-card__title">
                                { title.clone() }
                            </@>
                        }
                        if let Some(subtitle) = &props.subtitle {
                            <p class="yi-card__subtitle">{ subtitle.clone() }</p>
                        }
                    </div>
                    if props.actions != empty {
                        <div class="yi-card__actions">{ props.actions.clone() }</div>
                    }
                </header>
            }
            <div class={classes!("yi-card__body", props.flush.then_some("yi-card__body--flush"))}>
                { props.children.clone() }
            </div>
            if props.footer != empty {
                <footer class="yi-card__footer">{ props.footer.clone() }</footer>
            }
        </section>
    })
}

#[cfg(test)]
mod tests;
