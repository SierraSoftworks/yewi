//! Layout primitives: [`Stack`], [`Cluster`] and [`Grid`].
//!
//! They own the space *between* children, so components never need outer
//! margins.

use yew::prelude::*;

use crate::{Attrs, Space};

/// Cross-axis alignment of children.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl Align {
    const fn as_str(self) -> &'static str {
        match self {
            Align::Start => "start",
            Align::Center => "center",
            Align::End => "end",
            Align::Stretch => "stretch",
            Align::Baseline => "baseline",
        }
    }
}

/// Main-axis distribution of children.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
    Between,
}

impl Justify {
    const fn as_str(self) -> &'static str {
        match self {
            Justify::Start => "start",
            Justify::Center => "center",
            Justify::End => "end",
            Justify::Between => "between",
        }
    }
}

fn flex_classes(base: &'static str, align: Option<Align>, justify: Option<Justify>, class: &Classes) -> Classes {
    classes!(
        base,
        align.map(|a| format!("{base}--align-{}", a.as_str())),
        justify.map(|j| format!("{base}--justify-{}", j.as_str())),
        class.clone(),
    )
}

#[derive(Properties, PartialEq)]
pub struct StackProps {
    #[prop_or_default]
    pub children: Html,
    /// Space between children. Defaults to `Space::Md`.
    #[prop_or_default]
    pub gap: Option<Space>,
    #[prop_or_default]
    pub align: Option<Align>,
    #[prop_or_default]
    pub justify: Option<Justify>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Children in a column with even gaps.
#[function_component]
pub fn Stack(props: &StackProps) -> Html {
    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={flex_classes("yi-stack", props.align, props.justify, &props.class)}
            style={props.gap.map(|gap| format!("--yi-gap:{}", gap.css()))}
        >
            { props.children.clone() }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct ClusterProps {
    #[prop_or_default]
    pub children: Html,
    /// Space between children. Defaults to `Space::Sm`.
    #[prop_or_default]
    pub gap: Option<Space>,
    #[prop_or_default]
    pub align: Option<Align>,
    #[prop_or_default]
    pub justify: Option<Justify>,
    /// Keep children on one line instead of wrapping.
    #[prop_or_default]
    pub nowrap: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Children in a row that wraps: toolbars, button rows, tag lists.
#[function_component]
pub fn Cluster(props: &ClusterProps) -> Html {
    let mut class = flex_classes("yi-cluster", props.align, props.justify, &props.class);
    if props.nowrap {
        class.push("yi-cluster--nowrap");
    }
    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} style={props.gap.map(|gap| format!("--yi-gap:{}", gap.css()))}>
            { props.children.clone() }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct GridProps {
    #[prop_or_default]
    pub children: Html,
    /// Space between cells. Defaults to `Space::Lg`.
    #[prop_or_default]
    pub gap: Option<Space>,
    /// Smallest column width as a CSS length; as many equal columns as fit
    /// are created. Defaults to `16rem`. Ignored when `columns` is set.
    #[prop_or_default]
    pub min: Option<AttrValue>,
    /// A fixed number of equal columns.
    #[prop_or_default]
    pub columns: Option<u8>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Equal columns that reflow to fit the container, with no breakpoints.
#[function_component]
pub fn Grid(props: &GridProps) -> Html {
    let style: String = [
        props.gap.map(|gap| format!("--yi-gap:{}", gap.css())),
        props.columns.map(|n| format!("--yi-grid-columns:{n}")),
        props.min.as_ref().map(|min| format!("--yi-grid-min:{min}")),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(";");

    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!("yi-grid", props.columns.map(|_| "yi-grid--fixed"), props.class.clone())}
            style={(!style.is_empty()).then_some(style)}
        >
            { props.children.clone() }
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gap_is_taken_from_the_spacing_scale() {
        let html = yewi_test::render(|| html! { <Stack gap={Space::Xl}>{ "a" }</Stack> });
        assert!(html.contains("var(--yi-space-xl)"));
    }

    #[test]
    fn attrs_reach_the_root_element() {
        let html = yewi_test::render(|| {
            html! { <Cluster attrs={crate::attrs! { "role" => "toolbar", "aria-label" => "Actions" }}>{ "a" }</Cluster> }
        });
        assert!(html.contains(r#"role="toolbar""#));
        assert!(html.contains(r#"aria-label="Actions""#));
    }
}
