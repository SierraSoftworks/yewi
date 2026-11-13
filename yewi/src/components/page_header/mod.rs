//! [`PageHeader`]: the title row at the top of a page.

use yew::prelude::*;

use crate::Attrs;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    /// The page title, rendered as the page's `<h1>`.
    pub title: AttrValue,
    /// A supporting line, or richer content, under the title.
    #[prop_or_default]
    pub subtitle: Html,
    /// Content above the title: breadcrumbs or a link back to the parent.
    #[prop_or_default]
    pub breadcrumbs: Html,
    /// Short status beside the title: a pill, a version, a timestamp.
    #[prop_or_default]
    pub meta: Html,
    /// The page's actions, at the end of the title row. On a narrow page
    /// they move under the subtitle. Give a `NativeSelect` or other
    /// input here `fit`, or it takes a whole line for itself.
    #[prop_or_default]
    pub actions: Html,
    /// Full-width content under the header, such as tabs or a filter bar.
    #[prop_or_default]
    pub children: Html,
    /// Draws a rule under the header, for pages whose content is not
    /// already carded.
    #[prop_or_default]
    pub divided: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The title, context and actions that open a page.
#[function_component]
pub fn PageHeader(props: &PageHeaderProps) -> Html {
    let empty = Html::default();
    let class = classes!("yi-page-header", props.divided.then_some("yi-page-header--divided"), props.class.clone());

    props.attrs.apply(html! {
        <header id={props.id.clone()} {class}>
            if props.breadcrumbs != empty {
                <div class="yi-page-header__breadcrumbs">{ props.breadcrumbs.clone() }</div>
            }
            <div class="yi-page-header__row">
                <div class="yi-page-header__heading">
                    <h1 class="yi-page-header__title">{ props.title.clone() }</h1>
                    if props.meta != empty {
                        <div class="yi-page-header__meta">{ props.meta.clone() }</div>
                    }
                </div>
                if props.actions != empty {
                    <div class="yi-page-header__actions">{ props.actions.clone() }</div>
                }
            </div>
            if props.subtitle != empty {
                <div class="yi-page-header__subtitle">{ props.subtitle.clone() }</div>
            }
            if props.children != empty {
                <div class="yi-page-header__content">{ props.children.clone() }</div>
            }
        </header>
    })
}

#[cfg(test)]
mod tests;
