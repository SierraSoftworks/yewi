use yew::prelude::*;

use super::{Button, ButtonKind, ButtonVariant};
use crate::{Attrs, IconData, Size};

#[derive(Properties, PartialEq)]
pub struct IconButtonProps {
    /// The icon on the button's face.
    pub icon: IconData,
    /// What the button does. It is the accessible name and the hover hint, so
    /// it is required: an icon alone says nothing to a screen reader.
    pub label: AttrValue,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: Size,
    /// Unavailable: not focusable and not clickable.
    #[prop_or_default]
    pub disabled: bool,
    /// Work is in progress: the icon becomes a spinner and clicks are ignored.
    #[prop_or_default]
    pub loading: bool,
    /// Makes this a toggle button and says whether it is on. Ignored with
    /// `href`. Keep `label` the same in both states.
    #[prop_or_default]
    pub pressed: Option<bool>,
    /// Renders a link that looks like a button.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// The link `target`. Only used with `href`.
    #[prop_or_default]
    pub target: Option<AttrValue>,
    /// The native button type. Ignored with `href`.
    #[prop_or_default]
    pub kind: ButtonKind,
    /// Fired on activation. Not fired while `disabled` or `loading`.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A square [`Button`] that shows only an icon, for toolbars and repeated row
/// actions where the meaning is familiar. Prefer a labelled button elsewhere.
#[function_component]
pub fn IconButton(props: &IconButtonProps) -> Html {
    // The label comes first so that `attrs` can still override either name.
    let named = [("aria-label", props.label.clone()), ("title", props.label.clone())];
    let attrs = Attrs::new(named.into_iter().chain(props.attrs.iter().map(|(key, value)| (key, value.clone()))));

    html! {
        <Button
            icon={props.icon}
            variant={props.variant}
            size={props.size}
            disabled={props.disabled}
            loading={props.loading}
            pressed={props.pressed}
            href={props.href.clone()}
            target={props.target.clone()}
            kind={props.kind}
            on_click={props.on_click.clone()}
            class={classes!("yi-button--icon-only", props.class.clone())}
            id={props.id.clone()}
            {attrs}
        />
    }
}
