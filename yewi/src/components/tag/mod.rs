//! A compact label for metadata: categories, attributes, `name: value` pairs.

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, Tone, icons, use_id};

#[derive(Properties, PartialEq)]
pub struct TagProps {
    /// The label, or the value when `name` is set.
    #[prop_or_default]
    pub children: Html,
    /// A key shown before the label in a quieter colour, for `name: value`
    /// metadata such as `env: production`.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Colours the tag when it carries a status, such as a "destructive"
    /// marker. Most tags stay `Neutral`.
    #[prop_or_default]
    pub tone: Tone,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Sets the label in the monospace face, for identifiers and versions.
    #[prop_or_default]
    pub mono: bool,
    /// Renders the label as a link.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// Makes the tag removable: adds a remove button that fires this.
    #[prop_or_default]
    pub on_remove: Option<Callback<MouseEvent>>,
    /// The verb in the remove button's accessible name, which is this
    /// followed by the tag's text ("Remove production").
    #[prop_or(AttrValue::Static("Remove"))]
    pub remove_label: AttrValue,
    /// Unavailable: the remove button cannot be used.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A metadata tag, optionally removable.
#[function_component]
pub fn Tag(props: &TagProps) -> Html {
    let uid = use_id();
    let text_id = format!("{uid}-text");
    let remove_id = format!("{uid}-remove");

    let class = classes!(
        "yi-tag",
        format!("yi-tag--{}", props.tone.as_str()),
        props.mono.then_some("yi-tag--mono"),
        props.class.clone(),
    );

    let text = html! {
        <>
            if let Some(name) = &props.name {
                <span class="yi-tag__name">{ name.clone() }</span>
            }
            <span class="yi-tag__value">{ props.children.clone() }</span>
        </>
    };

    props.attrs.apply(html! {
        <span id={props.id.clone()} {class}>
            if let Some(icon) = props.icon {
                <Icon {icon} />
            }
            if let Some(href) = &props.href {
                <a id={text_id.clone()} class="yi-tag__text" {href}>{ text }</a>
            } else {
                <span id={text_id.clone()} class="yi-tag__text">{ text }</span>
            }
            if let Some(on_remove) = &props.on_remove {
                <button
                    id={remove_id.clone()}
                    class="yi-tag__remove"
                    type="button"
                    disabled={props.disabled}
                    aria-label={props.remove_label.clone()}
                    aria-labelledby={format!("{remove_id} {text_id}")}
                    onclick={on_remove.clone()}
                >
                    <Icon icon={icons::X} />
                </button>
            }
        </span>
    })
}

#[cfg(test)]
mod tests;
