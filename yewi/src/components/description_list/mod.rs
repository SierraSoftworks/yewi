//! Description lists: labelled facts about one thing, as a native `<dl>`.

use yew::prelude::*;

use crate::Attrs;

/// Where a term sits relative to its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum DescriptionLayout {
    /// Term beside its value, with every value starting on the same line.
    /// Falls back to term-above-value when the list is too narrow.
    #[default]
    Inline,
    /// Term above its value, with items flowing into as many columns as fit:
    /// summary strips and page headers.
    Stacked,
}

impl DescriptionLayout {
    pub const ALL: [DescriptionLayout; 2] = [DescriptionLayout::Inline, DescriptionLayout::Stacked];

    const fn as_str(self) -> &'static str {
        match self {
            DescriptionLayout::Inline => "inline",
            DescriptionLayout::Stacked => "stacked",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DescriptionListProps {
    /// The [`DescriptionItem`]s.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub layout: DescriptionLayout,
    /// A hairline between rows, for long lists that are scanned by term.
    /// Only used with `DescriptionLayout::Inline`.
    #[prop_or_default]
    pub ruled: bool,
    /// Smallest column width as a CSS length, for `DescriptionLayout::Stacked`.
    /// Defaults to `9rem`. Ignored when `columns` is set.
    #[prop_or_default]
    pub min: Option<AttrValue>,
    /// A fixed number of equal columns, for `DescriptionLayout::Stacked`.
    #[prop_or_default]
    pub columns: Option<u8>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A list of terms and their values: the facts about a host, a certificate,
/// a release.
#[function_component]
pub fn DescriptionList(props: &DescriptionListProps) -> Html {
    let stacked = props.layout == DescriptionLayout::Stacked;

    let style: String = [
        props.columns.filter(|_| stacked).map(|n| format!("--yi-description-list-columns:{n}")),
        props.min.as_ref().filter(|_| stacked).map(|min| format!("--yi-description-list-min:{min}")),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(";");

    props.attrs.apply(html! {
        <dl
            id={props.id.clone()}
            class={classes!(
                "yi-description-list",
                format!("yi-description-list--{}", props.layout.as_str()),
                (props.ruled && !stacked).then_some("yi-description-list--ruled"),
                (props.columns.is_some() && stacked).then_some("yi-description-list--fixed"),
                props.class.clone(),
            )}
            style={(!style.is_empty()).then_some(style)}
        >
            { props.children.clone() }
        </dl>
    })
}

#[derive(Properties, PartialEq)]
pub struct DescriptionItemProps {
    /// The label: what the value is.
    pub term: Html,
    /// The value. Any content: text, pills, links, a copyable id. When empty
    /// a muted dash is shown, so a missing value never collapses the row.
    #[prop_or_default]
    pub children: Html,
    /// Sets the value in the monospace face: paths, ids, hashes, addresses.
    #[prop_or_default]
    pub mono: bool,
    /// Spans every column of a `DescriptionLayout::Stacked` list.
    #[prop_or_default]
    pub wide: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One term and its value inside a [`DescriptionList`].
#[function_component]
pub fn DescriptionItem(props: &DescriptionItemProps) -> Html {
    let empty = props.children == Html::default();

    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!(
                "yi-description-list__item",
                props.wide.then_some("yi-description-list__item--wide"),
                props.class.clone(),
            )}
        >
            <dt class="yi-description-list__term">{ props.term.clone() }</dt>
            <dd class={classes!(
                "yi-description-list__value",
                props.mono.then_some("yi-description-list__value--mono"),
                empty.then_some("yi-description-list__value--empty"),
            )}>
                if empty { { "\u{2014}" } } else { { props.children.clone() } }
            </dd>
        </div>
    })
}

#[cfg(test)]
mod tests;
