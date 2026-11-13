//! yewi: a design system and component library for [Yew](https://yew.rs).
//!
//! ```ignore
//! use yew::prelude::*;
//! use yewi::prelude::*;
//!
//! #[function_component]
//! fn App() -> Html {
//!     html! {
//!         <>
//!             <Styles />
//!             <Button variant={ButtonVariant::Primary}>{ "Deploy" }</Button>
//!         </>
//!     }
//! }
//! ```
//!
//! # Rendering modes
//!
//! Enable `csr`, `ssr` or `hydration` to match how the app renders; each
//! forwards to the Yew feature of the same name. Every component renders the
//! same markup on the server and on the first client render.
//!
//! # Styles
//!
//! The stylesheet for the enabled features is compiled into the crate. Render
//! [`Styles`], call [`install_styles`], or serve [`STYLESHEET`] from a server.
//! All rules live in `@layer yewi.*`, so any unlayered app rule overrides them.
//! [`SCSS`] holds the Sass mixins behind them, for app stylesheets.
//!
//! # Opt-in components
//!
//! Heavier components sit behind Cargo features (`charts`, `map`, `code`, ...)
//! and add nothing to the build unless enabled.

mod common;
pub mod hooks;
pub mod icons;
mod styles;

/// Every component, one module each.
pub mod components {
    include!(concat!(env!("OUT_DIR"), "/components.rs"));
}

pub use common::*;
#[doc(inline)]
pub use components::*;
pub use hooks::{Now, NowProvider, Tick, use_id, use_now};
pub use icons::IconData;
pub use styles::{SCSS, STYLESHEET, Styles, install_styles};

/// The font files, `@font-face` rules and helpers to serve or export them.
#[cfg(feature = "fonts")]
pub use yewi_fonts as fonts;

/// Everything an app usually needs: `use yewi::prelude::*;`.
pub mod prelude {
    pub use crate::attrs;
    pub use crate::common::*;
    pub use crate::components::*;
    pub use crate::hooks::{Now, NowProvider, Tick, use_id, use_now};
    pub use crate::icons::{self, IconData};
    pub use crate::styles::{SCSS, STYLESHEET, Styles, install_styles};
}
