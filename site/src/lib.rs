//! The yewi reference site.
//!
//! It is three things at once:
//!
//! - **Documentation**: a page per component with live examples and their
//!   source, in the manner of the ElementPlus docs.
//! - **Test fixture**: every example has an isolated route that the browser
//!   tests screenshot, check for accessibility and hydrate.
//! - **Dogfood**: it is built with yewi, prerendered on the server and
//!   hydrated in the browser.

pub mod app;
pub mod docs;
mod gallery;
pub mod guides;
mod shell;

/// Documentation pages, one module per directory under `src/pages/`.
pub mod pages {
    use crate::docs::Page;

    include!(concat!(env!("OUT_DIR"), "/pages.rs"));
}

/// The API reference extracted from the library source at build time.
pub mod api {
    use crate::docs::{ApiComponent, ApiEnum, ApiModule, ApiProp};

    include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

pub use app::{App, AppProps, Route};

/// The path the site is served under, with a leading and trailing slash.
/// Set `YEWI_SITE_BASE` at build time for a sub-path such as `/yewi/`.
pub const BASE: &str = match option_env!("YEWI_SITE_BASE") {
    Some(base) => base,
    None => "/",
};

/// [`BASE`] as a router basename: no trailing slash, `None` at the root.
pub fn basename() -> Option<&'static str> {
    let trimmed = BASE.trim_end_matches('/');
    (!trimmed.is_empty()).then_some(trimmed)
}

/// The time examples see, so renders and screenshots are reproducible:
/// 2026-01-15 12:00:00 UTC.
pub const EXAMPLE_NOW_MILLIS: f64 = 1_768_478_400_000.0;
