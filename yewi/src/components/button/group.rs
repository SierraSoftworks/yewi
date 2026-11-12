use yew::prelude::*;

use crate::Attrs;

#[derive(Properties, PartialEq)]
pub struct ButtonGroupProps {
    /// The buttons to join, as direct children.
    #[prop_or_default]
    pub children: Html,
    /// What the set of actions is for. Without it a screen reader can only
    /// announce "group".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Stacks the buttons in a column, for controls docked to an edge.
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Several actions on one thing, joined into a single control. The shared
/// edge says they belong together and takes less room than separate buttons.
///
/// Each button stays its own tab stop. For a set of options where one is
/// selected, use a segmented control instead.
#[function_component]
pub fn ButtonGroup(props: &ButtonGroupProps) -> Html {
    let class =
        classes!("yi-button-group", props.vertical.then_some("yi-button-group--vertical"), props.class.clone());

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} role="group" aria-label={props.label.clone()}>
            { props.children.clone() }
        </div>
    })
}
