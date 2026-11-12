use yew::prelude::*;

/// The compiled stylesheet for the enabled features.
///
/// Servers can expose this at a cached URL instead of rendering [`Styles`].
pub const STYLESHEET: &str = include_str!(concat!(env!("OUT_DIR"), "/yewi.css"));

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
    use super::STYLESHEET;

    #[test]
    fn stylesheet_has_no_encoding_markers() {
        // A byte order mark or `@charset` inside a layer swallows the rule after it.
        assert!(!STYLESHEET.contains('\u{feff}'), "byte order mark in the stylesheet");
        assert!(!STYLESHEET.contains("@charset"), "@charset in the stylesheet");
    }
}
