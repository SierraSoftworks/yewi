//! Icon path data.
//!
//! Icons are constants, so only the ones an app names end up in its binary:
//!
//! ```ignore
//! html! { <Icon icon={icons::SEARCH} /> }
//! ```
//!
//! The bundled set is [Lucide](https://lucide.dev). Apps add their own with
//! [`IconData::new`]: a path on a 24-unit grid, drawn as a stroke.

mod lucide;

pub use lucide::*;

/// An icon: SVG path data on a 24x24 grid, rendered as a round-capped stroke
/// in `currentColor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconData(&'static str);

impl IconData {
    pub const fn new(path: &'static str) -> Self {
        Self(path)
    }

    /// The SVG path data (`d` attribute).
    pub const fn path(self) -> &'static str {
        self.0
    }
}
