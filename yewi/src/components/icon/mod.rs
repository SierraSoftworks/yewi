//! A stroked SVG icon.

use yew::prelude::*;

use crate::{Attrs, IconData};

#[derive(Properties, PartialEq)]
pub struct IconProps {
    /// The icon to draw, from [`yewi::icons`](crate::icons) or the app's own.
    pub icon: IconData,
    /// Accessible name. Leave unset for a decorative icon, which is hidden
    /// from assistive technology.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Width and height as a CSS length. Defaults to `1.15em`, so the icon
    /// follows the text it sits in.
    #[prop_or_default]
    pub size: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Draws an [`IconData`] as an inline SVG in the current text colour.
#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    let style = props.size.as_ref().map(|size| format!("--yi-icon-size:{size}"));
    let decorative = props.label.is_none();

    props.attrs.apply(html! {
        <svg
            id={props.id.clone()}
            class={classes!("yi-icon", props.class.clone())}
            {style}
            viewBox="0 0 24 24"
            role={(!decorative).then_some("img")}
            aria-label={props.label.clone()}
            aria-hidden={decorative.then_some("true")}
            focusable="false"
        >
            <path d={props.icon.path()} />
        </svg>
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icons;

    #[test]
    fn decorative_icons_are_hidden_from_assistive_technology() {
        let html = yewi_test::render(|| html! { <Icon icon={icons::CHECK} /> });
        assert!(html.contains(r#"aria-hidden="true""#));
        assert!(!html.contains("role="));
    }

    #[test]
    fn labelled_icons_are_exposed_as_images() {
        let html = yewi_test::render(|| html! { <Icon icon={icons::CHECK} label="Passed" /> });
        assert!(html.contains(r#"role="img""#));
        assert!(html.contains(r#"aria-label="Passed""#));
        assert!(!html.contains("aria-hidden"));
    }
}
