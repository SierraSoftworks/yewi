//! Generates the page registry and the API reference.
//!
//! - Every directory under `src/pages/` with a `mod.rs` is a documentation
//!   page; it is declared as a module and added to `PAGES`.
//! - The API tables come from the library source: props and their doc
//!   comments from `#[derive(Properties)]` structs, public enums, and the
//!   `// Component tokens` block of each component's SCSS.

use std::{
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use quote::ToTokens;
use syn::{Attribute, Expr, Fields, FnArg, Item, Lit, Meta, Type, Visibility};

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=src/pages");
    println!("cargo:rerun-if-changed=../yewi/src/components");
    println!("cargo:rerun-if-env-changed=YEWI_SITE_BASE");

    fs::write(out.join("pages.rs"), pages(&root.join("src/pages"))).unwrap();
    fs::write(out.join("api.rs"), api(&root.join("../yewi/src/components"))).unwrap();
}

fn sorted(dir: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    entries
}

fn name_of(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_owned()
}

fn pages(dir: &Path) -> String {
    let mut mods = String::new();
    let mut list = String::from("pub static PAGES: &[Page] = &[\n");
    for page in sorted(dir) {
        let module = page.join("mod.rs");
        if !module.is_file() {
            continue;
        }
        let name = name_of(&page);
        writeln!(mods, "#[path = {:?}]\npub mod {name};", module.display().to_string()).unwrap();
        writeln!(list, "    Page {{ slug: {:?}, module: {name:?}, def: &{name}::PAGE }},", name.replace('_', "-"))
            .unwrap();
    }
    list.push_str("];\n");
    mods + &list
}

// ---- API extraction --------------------------------------------------------

fn api(dir: &Path) -> String {
    let mut out = String::from("pub static API: &[ApiModule] = &[\n");
    for component in sorted(dir).into_iter().filter(|p| p.is_dir()) {
        let mut module = Module::default();
        for file in sorted(&component) {
            match file.extension().and_then(|e| e.to_str()) {
                Some("rs") => module.read_rust(&file),
                Some("scss") => module.read_scss(&file),
                _ => {}
            }
        }
        module.write(&name_of(&component), &mut out);
    }
    out.push_str("];\n");
    out
}

/// `(name, type, default, docs)`
type Prop = (String, String, String, String);
/// `(variant, docs)`
type Variant = (String, String);

#[derive(Default)]
struct Module {
    /// `(component, props struct, docs)`
    components: Vec<(String, Option<String>, String)>,
    /// `(struct name, props)`
    props: Vec<(String, Vec<Prop>)>,
    /// `(name, docs, variants)`
    enums: Vec<(String, String, Vec<Variant>)>,
    /// `(token, default, note)`
    tokens: Vec<(String, String, String)>,
}

impl Module {
    fn read_rust(&mut self, path: &Path) {
        let source = fs::read_to_string(path).unwrap();
        let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()));
        for item in file.items {
            match item {
                Item::Fn(f) if is_pub(&f.vis) && has_attr(&f.attrs, "function_component") => {
                    let props = f.sig.inputs.first().and_then(|arg| match arg {
                        FnArg::Typed(arg) => match &*arg.ty {
                            Type::Reference(r) => Some(struct_name(&r.elem)),
                            other => Some(struct_name(other)),
                        },
                        FnArg::Receiver(_) => None,
                    });
                    self.components.push((f.sig.ident.to_string(), props, docs(&f.attrs)));
                }
                Item::Struct(s) if is_pub(&s.vis) && derives(&s.attrs, "Properties") => {
                    let Fields::Named(fields) = s.fields else { continue };
                    let props = fields
                        .named
                        .iter()
                        .filter(|f| is_pub(&f.vis))
                        .map(|f| {
                            (
                                f.ident.as_ref().unwrap().to_string(),
                                type_string(&f.ty),
                                default_of(&f.attrs),
                                docs(&f.attrs),
                            )
                        })
                        .collect();
                    self.props.push((s.ident.to_string(), props));
                }
                Item::Enum(e) if is_pub(&e.vis) => {
                    let variants = e.variants.iter().map(|v| (v.ident.to_string(), docs(&v.attrs))).collect();
                    self.enums.push((e.ident.to_string(), docs(&e.attrs), variants));
                }
                _ => {}
            }
        }
    }

    /// Reads the declarations that follow a `// Component tokens` comment, up
    /// to the first blank line.
    fn read_scss(&mut self, path: &Path) {
        let source = fs::read_to_string(path).unwrap();
        let mut lines = source.lines();
        while let Some(line) = lines.next() {
            if line.trim() != "// Component tokens" {
                continue;
            }
            for line in lines.by_ref() {
                let line = line.trim();
                let Some((name, rest)) = line.split_once(':') else { break };
                if !name.starts_with("--") {
                    break;
                }
                let (value, note) = rest.split_once("//").unwrap_or((rest, ""));
                self.tokens.push((
                    name.to_owned(),
                    value.trim().trim_end_matches(';').to_owned(),
                    note.trim().to_owned(),
                ));
            }
        }
    }

    fn write(&self, name: &str, out: &mut String) {
        writeln!(out, "    ApiModule {{ module: {name:?}, components: &[").unwrap();
        for (component, props_name, docs) in &self.components {
            writeln!(out, "        ApiComponent {{ name: {component:?}, docs: {docs:?}, props: &[").unwrap();
            let props = props_name.as_ref().and_then(|n| self.props.iter().find(|(s, _)| s == n));
            for (name, ty, default, docs) in props.map(|(_, p)| p.as_slice()).unwrap_or_default() {
                writeln!(
                    out,
                    "            ApiProp {{ name: {name:?}, ty: {ty:?}, default: {default:?}, docs: {docs:?} }},"
                )
                .unwrap();
            }
            writeln!(out, "        ] }},").unwrap();
        }
        writeln!(out, "    ], enums: &[").unwrap();
        for (name, docs, variants) in &self.enums {
            writeln!(out, "        ApiEnum {{ name: {name:?}, docs: {docs:?}, variants: &{variants:?} }},").unwrap();
        }
        writeln!(out, "    ], tokens: &{:?} }},", self.tokens).unwrap();
    }
}

fn is_pub(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

fn has_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|a| a.path().segments.last().is_some_and(|s| s.ident == name))
}

fn derives(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().filter(|a| a.path().is_ident("derive")).any(|a| a.meta.to_token_stream().to_string().contains(name))
}

fn docs(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .filter_map(|a| match &a.meta {
            Meta::NameValue(nv) => match &nv.value {
                Expr::Lit(lit) => match &lit.lit {
                    Lit::Str(s) => Some(s.value().trim().to_owned()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_owned()
}

/// `""` for a required prop, otherwise the default as written.
fn default_of(attrs: &[Attribute]) -> String {
    for attr in attrs {
        let Some(name) = attr.path().get_ident().map(ToString::to_string) else { continue };
        match name.as_str() {
            "prop_or_default" => return "default".into(),
            "prop_or" | "prop_or_else" => {
                return attr
                    .parse_args::<proc_macro2::TokenStream>()
                    .map(|tokens| compact(&tokens.to_string()))
                    .unwrap_or_default();
            }
            _ => {}
        }
    }
    String::new()
}

/// The struct a props type names, without its path or generic arguments:
/// `TableProps` for `&TableProps<T>`, so generic components find their props.
fn struct_name(ty: &Type) -> String {
    match ty {
        Type::Path(path) => path.path.segments.last().map_or_else(|| type_string(ty), |s| s.ident.to_string()),
        other => type_string(other),
    }
}

fn type_string(ty: &Type) -> String {
    compact(&ty.to_token_stream().to_string())
}

/// Removes the spaces `TokenStream` puts around punctuation.
fn compact(tokens: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = tokens.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c == ' ' {
            let prev = chars[..i].iter().rev().find(|c| **c != ' ').copied().unwrap_or(' ');
            let next = chars[i + 1..].iter().find(|c| **c != ' ').copied().unwrap_or(' ');
            let tight = |c: char| "<>:&()[],;.".contains(c);
            if tight(prev) && prev != ',' || tight(next) {
                continue;
            }
        }
        out.push(c);
    }
    out.replace(",", ", ").replace(",  ", ", ")
}
