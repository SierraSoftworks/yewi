//! Discovers components and compiles the stylesheet.
//!
//! - Every directory under `src/components/` with a `mod.rs` is a component.
//!   It is declared as a module and re-exported, so adding a component never
//!   means editing a shared list.
//! - A component whose `mod.rs` starts with `#![cfg(feature = "name")]` is
//!   skipped, Rust and SCSS alike, unless that feature is enabled.
//! - The stylesheet is `scss/*.scss` (foundations, one cascade layer each)
//!   followed by every enabled component's `*.scss`, compiled with `grass`.

use std::{
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

/// Foundation partials in cascade order: `(file stem, layer)`.
const FOUNDATIONS: &[(&str, &str)] = &[("reset", "yewi.reset"), ("tokens", "yewi.tokens"), ("base", "yewi.base")];
const LAYERS: &str = "@layer yewi.reset,yewi.tokens,yewi.base,yewi.components,yewi.utilities;";

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let scss = root.join("scss");
    let components_dir = root.join("src/components");

    println!("cargo:rerun-if-changed=scss");
    println!("cargo:rerun-if-changed=src/components");

    let options = grass::Options::default().style(grass::OutputStyle::Compressed).load_path(&scss);
    let compile = |path: &Path| -> String {
        grass::from_path(path, &options).unwrap_or_else(|e| panic!("failed to compile {}: {e}", path.display()))
    };

    let mut css = String::from(LAYERS);
    for (stem, layer) in FOUNDATIONS {
        let body = compile(&scss.join(format!("{stem}.scss")));
        write!(css, "@layer {layer}{{{}}}", body.trim()).unwrap();
    }

    let mut modules = String::new();
    for dir in sorted(&components_dir) {
        let module = dir.join("mod.rs");
        if !module.is_file() {
            continue;
        }
        let name = dir.file_name().unwrap().to_str().unwrap().to_owned();
        if let Some(feature) = required_feature(&module)
            && env::var_os(format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"))).is_none()
        {
            continue;
        }

        writeln!(
            modules,
            "#[path = {:?}]\npub mod {name};\n#[doc(inline)]\npub use self::{name}::*;",
            module.display().to_string()
        )
        .unwrap();

        for file in sorted(&dir) {
            let is_partial = file.extension().is_some_and(|e| e == "scss")
                && !file.file_name().unwrap().to_str().unwrap().starts_with('_');
            if is_partial {
                let body = compile(&file);
                write!(css, "@layer yewi.components{{{}}}", body.trim()).unwrap();
            }
        }
    }

    let utilities = compile(&scss.join("utilities.scss"));
    write!(css, "@layer yewi.utilities{{{}}}", utilities.trim()).unwrap();

    fs::write(out.join("components.rs"), modules).unwrap();
    fs::write(out.join("yewi.css"), css).unwrap();
}

fn sorted(dir: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    entries
}

/// The feature named by a leading `#![cfg(feature = "...")]`, if any.
fn required_feature(module: &Path) -> Option<String> {
    let source = fs::read_to_string(module).unwrap();
    source
        .lines()
        .map(str::trim)
        .take_while(|line| line.is_empty() || line.starts_with("//") || line.starts_with("#!["))
        .find_map(|line| {
            let rest = line.strip_prefix("#![cfg(feature = \"")?;
            Some(rest.split('"').next()?.to_owned())
        })
}
