//! The probe never installs the stylesheet: `cargo xtask size` measures it apart.
//!
//! Includes `src/features/<feature>.rs` for every enabled feature. Each file
//! defines `pub fn view() -> yew::Html` that renders the feature's components,
//! so their code is linked and counted.

use std::{env, fmt::Write as _, fs, path::PathBuf};

fn main() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/features");
    println!("cargo:rerun-if-changed=src/features");

    let mut mods = String::new();
    let mut views = String::from("pub fn views() -> Vec<yew::Html> {\n    vec![\n");
    let mut files: Vec<_> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
    files.sort();
    for file in files.iter().filter(|f| f.extension().is_some_and(|e| e == "rs")) {
        let name = file.file_stem().unwrap().to_str().unwrap();
        if env::var_os(format!("CARGO_FEATURE_{}", name.to_uppercase())).is_none() {
            continue;
        }
        writeln!(mods, "#[path = {:?}]\nmod {name};", file.display().to_string()).unwrap();
        writeln!(views, "        {name}::view(),").unwrap();
    }
    views.push_str("    ]\n}\n");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out.join("features.rs"), mods + &views).unwrap();
}
