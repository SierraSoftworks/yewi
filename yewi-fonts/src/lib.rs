//! The self-hosted fonts of the yewi design system: Montserrat (headings),
//! Inter (text and controls) and Fira Code (code), as Latin-subset variable
//! WOFF2 files under the SIL Open Font License.
//!
//! The crate has no dependencies, so it works as a build dependency.
//!
//! - **Trunk apps**: call [`write_to`] from `build.rs` into a directory that
//!   a `post_build` hook stages, and link the `fonts.css` it writes.
//! - **Servers**: serve each of [`FILES`] under one URL prefix and the output
//!   of [`css`] for that prefix.
//!
//! yewi's stylesheet names these families with a system-font fallback, so an
//! app that ships no fonts still renders correctly.

use std::{fs, io, path::Path};

/// One font file.
#[derive(Debug, Clone, Copy)]
pub struct FontFile {
    /// File name, e.g. `inter-latin.woff2`.
    pub name: &'static str,
    /// CSS `font-family`.
    pub family: &'static str,
    /// CSS `font-style`.
    pub style: &'static str,
    /// CSS `font-weight` range covered by the variable font.
    pub weight: &'static str,
    /// The WOFF2 bytes.
    pub bytes: &'static [u8],
}

macro_rules! font {
    ($name:literal, $family:literal, $style:literal, $weight:literal) => {
        FontFile {
            name: $name,
            family: $family,
            style: $style,
            weight: $weight,
            bytes: include_bytes!(concat!("../fonts/", $name)),
        }
    };
}

/// Every bundled font file.
pub const FILES: &[FontFile] = &[
    font!("montserrat-latin.woff2", "Montserrat", "normal", "300 600"),
    font!("inter-latin.woff2", "Inter", "normal", "400 700"),
    font!("inter-italic-latin.woff2", "Inter", "italic", "400 700"),
    font!("fira-code-latin.woff2", "Fira Code", "normal", "400 600"),
];

/// The media type to serve [`FILES`] with.
pub const CONTENT_TYPE: &str = "font/woff2";

/// The `@font-face` rules with URLs relative to the stylesheet itself.
pub const RELATIVE_CSS: &str = include_str!("../fonts/fonts.css");

/// The `@font-face` rules for [`FILES`] served under `base_url`
/// (for example `"/static/fonts"`).
pub fn css(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    RELATIVE_CSS.replace("url(\"./", &format!("url(\"{base}/"))
}

/// Writes every font file and a `fonts.css` that references them relatively
/// into `dir`, creating it if needed. Files that are already up to date are
/// left untouched, so this is safe to call from a build script.
///
/// In a Trunk app, stage the directory with a `post_build` hook
/// (`cp -R fonts "$TRUNK_STAGING_DIR/fonts"`), not a `copy-dir` link: Trunk
/// resolves its assets before cargo runs the build script, so the link fails
/// on a clean checkout.
pub fn write_to(dir: impl AsRef<Path>) -> io::Result<()> {
    let dir = dir.as_ref();
    fs::create_dir_all(dir)?;
    for file in FILES {
        write_if_changed(&dir.join(file.name), file.bytes)?;
    }
    write_if_changed(&dir.join("fonts.css"), RELATIVE_CSS.as_bytes())
}

fn write_if_changed(path: &Path, contents: &[u8]) -> io::Result<()> {
    if fs::read(path).is_ok_and(|existing| existing == contents) {
        return Ok(());
    }
    fs::write(path, contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_references_every_file_under_the_base_url() {
        let css = css("/static/fonts/");
        for file in FILES {
            assert!(css.contains(&format!("url(\"/static/fonts/{}\")", file.name)));
            assert!(css.contains(&format!("font-family: \"{}\"", file.family)));
        }
        assert_eq!(css.matches("@font-face").count(), FILES.len());
    }

    #[test]
    fn files_are_woff2() {
        for file in FILES {
            assert_eq!(&file.bytes[..4], b"wOF2", "{}", file.name);
        }
    }
}
