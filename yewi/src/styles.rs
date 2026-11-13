use yew::prelude::*;

/// The compiled stylesheet for the enabled features.
///
/// Servers can expose this at a cached URL instead of rendering [`Styles`].
pub const STYLESHEET: &str = include_str!(concat!(env!("OUT_DIR"), "/yewi.css"));

/// The Sass mixins the components are styled with (`card-surface`,
/// `focus-ring`, `heading`, `eyebrow`, `chip`, `truncate`, `motion`, ...), for
/// an app stylesheet whose own pieces should match. It is a Sass module that
/// emits no CSS by itself and needs the stylesheet's `--yi-*` tokens on the
/// page.
///
/// A build script of a crate that depends on `yewi` finds the same text in
/// the file named by the `DEP_YEWI_SCSS` environment variable, without
/// building this crate for the host:
///
/// ```no_run
/// // build.rs
/// let mixins = std::env::var("DEP_YEWI_SCSS").expect("yewi is a direct dependency");
/// std::fs::copy(mixins, "_yewi.scss").expect("the mixins should be writable");
/// ```
///
/// ```scss
/// @use "yewi" as yi;
///
/// .result-row { @include yi.card-surface; }
/// ```
pub const SCSS: &str = include_str!("../scss/_api.scss");

/// The `id` of the `<style>` element [`Styles`] and [`install_styles`] create.
pub const STYLES_ID: &str = "yewi-styles";

/// Renders the stylesheet inline as a `<style>` element.
///
/// Place it once, as early in the document as the app allows. Server-rendered
/// apps should put it (or a link to [`STYLESHEET`]) in the `<head>`.
#[function_component]
pub fn Styles() -> Html {
    html! { <style id={STYLES_ID}>{ STYLESHEET }</style> }
}

/// Appends the stylesheet to the document `<head>`, once. For client-rendered
/// apps, call it before `yew::Renderer::render()`. Does nothing off the web.
pub fn install_styles() {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
        if document.get_element_by_id(STYLES_ID).is_some() {
            return;
        }
        if let (Ok(style), Some(head)) = (document.create_element("style"), document.head()) {
            style.set_id(STYLES_ID);
            style.set_text_content(Some(STYLESHEET));
            let _ = head.append_child(&style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SCSS, STYLESHEET};

    fn compile(app: &str) -> String {
        let options = grass::Options::default().style(grass::OutputStyle::Compressed);
        grass::from_string(format!("{SCSS}\n{app}"), &options).expect("the mixins compile outside the crate")
    }

    #[test]
    fn the_mixins_are_usable_from_an_app_stylesheet() {
        assert_eq!(compile("").trim(), "", "the module emits CSS of its own");

        let css = compile(".row{@include card-surface}.name{@include truncate}a:focus-visible{@include focus-ring}");
        for expected in ["--yi-card-line", "text-overflow", "--yi-focus"] {
            assert!(css.contains(expected), "{expected} missing from {css}");
        }
    }

    #[test]
    fn stylesheet_has_no_encoding_markers() {
        // A byte order mark or `@charset` inside a layer swallows the rule after it.
        assert!(!STYLESHEET.contains('\u{feff}'), "byte order mark in the stylesheet");
        assert!(!STYLESHEET.contains("@charset"), "@charset in the stylesheet");
    }
}
