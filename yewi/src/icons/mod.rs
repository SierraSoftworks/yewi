//! Icon path data.
//!
//! Icons are constants, so only the ones an app names end up in its binary:
//!
//! ```ignore
//! html! { <Icon icon={icons::SEARCH} /> }
//! ```
//!
//! The bundled set is [Lucide](https://lucide.dev). Apps add their own with
//! [`IconData::new`], a path on a 24-unit grid drawn as a stroke, or
//! [`IconData::filled`] for a solid glyph such as a brand mark.

mod lucide;

pub use lucide::*;

/// An icon: SVG path data on a 24x24 grid, in `currentColor`. It is drawn as
/// a round-capped stroke, or filled when made with [`IconData::filled`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconData {
    path: &'static str,
    filled: bool,
}

impl IconData {
    /// An outline icon: the path is stroked.
    pub const fn new(path: &'static str) -> Self {
        Self { path, filled: false }
    }

    /// A solid glyph: the path is filled and not stroked. For brand and
    /// operating system marks, which are drawn as shapes rather than lines.
    /// Overlapping subpaths cut holes (the even-odd rule).
    pub const fn filled(path: &'static str) -> Self {
        Self { path, filled: true }
    }

    /// The SVG path data (`d` attribute).
    pub const fn path(self) -> &'static str {
        self.path
    }

    /// Whether the path is filled rather than stroked.
    pub const fn is_filled(self) -> bool {
        self.filled
    }
}
