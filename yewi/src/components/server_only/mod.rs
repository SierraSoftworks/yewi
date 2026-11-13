//! Server-only pages: components that work in a page that loads no wasm.
//!
//! Wrap the page in [`ServerOnly`]. Inside it components render markup the
//! browser can operate alone, and declare the rest in `data-yi-*` attributes
//! that one small script, [`BEHAVIOURS`], acts on: copying, the number
//! stepper, the theme toggle.
//!
//! ```ignore
//! html! {
//!     <ServerOnly script="/static/yewi.js">
//!         <Page />
//!     </ServerOnly>
//! }
//! ```
//!
//! On such a page a prop like `value` or `checked` is the starting value and
//! no `on_*` callback runs: state lives in the browser, and data leaves
//! through forms and links. An app that hydrates, or renders in the browser,
//! does not use this: its components already do everything themselves.

use yew::prelude::*;

/// The script that drives components inside [`ServerOnly`]. Plain JavaScript
/// with no dependencies; it does nothing on a page without such components,
/// and loading it twice is harmless.
///
/// Serve it from a cached route and name that route in `ServerOnly::script`,
/// or leave `script` unset to have it inlined.
pub const BEHAVIOURS: &str = include_str!("behaviours.js");

#[derive(Clone, PartialEq)]
struct Mode;

/// Whether the component is inside a [`ServerOnly`], and so has to work
/// without its own event handlers.
#[hook]
pub(crate) fn use_server_only() -> bool {
    use_context::<Mode>().is_some()
}

#[derive(Properties, PartialEq)]
pub struct ServerOnlyProps {
    #[prop_or_default]
    pub children: Html,
    /// Where the app serves [`BEHAVIOURS`]. Without it the script is inlined
    /// after the content, which a strict content security policy refuses.
    #[prop_or_default]
    pub script: Option<AttrValue>,
}

/// Marks its content as a page that loads no wasm, and loads [`BEHAVIOURS`].
/// It renders no element of its own.
#[function_component]
pub fn ServerOnly(props: &ServerOnlyProps) -> Html {
    html! {
        <ContextProvider<Mode> context={Mode}>
            { props.children.clone() }
            if let Some(src) = &props.script {
                <script {src} defer=true />
            } else {
                <script>{ BEHAVIOURS }</script>
            }
        </ContextProvider<Mode>>
    }
}

#[cfg(test)]
mod tests;
