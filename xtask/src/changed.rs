//! `cargo xtask ci --changed`: picks the checks a change needs from the paths it touches.
//!
//! A component is three directories (see CONTRIBUTING.md), so a change that stays inside them
//! only needs that component's tests, plus those of whatever uses it. Anything shared by every
//! component falls back to the full run.

use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    process::{Command, exit},
};

/// What one changed path means for the checks to run.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Change {
    /// Prose, licences and repository metadata: nothing is built from them.
    Docs,
    /// `yewi/src/components/<name>/`.
    Component(String),
    /// A documentation page or its recorded output, by module name.
    Page(String),
    /// A behaviour spec, by file stem.
    Spec(String),
    /// The reference site around the pages: shell, guides, styles.
    Site,
    /// A size probe for an opt-in feature.
    Probe,
    /// Shared by everything: tokens, mixins, hooks, the harness, manifests.
    Foundations,
}

pub fn classify(path: &str) -> Change {
    let parts: Vec<&str> = path.split('/').collect();
    let module = |slug: &str| slug.replace('-', "_");
    match parts.as_slice() {
        ["yewi", "src", "components", name, _, ..] => Change::Component(module(name)),
        ["site", "src", "pages", name, _, ..] => Change::Page(module(name)),
        ["e2e", "baselines", page, _, ..] => Change::Page(module(page)),
        ["site", "tests", "snapshots", file] if file.contains("__") => {
            Change::Page(module(file.split("__").next().unwrap()))
        }
        ["e2e", "tests", file] if file.ends_with(".spec.ts") => Change::Spec(file.trim_end_matches(".spec.ts").into()),
        ["tests", "size-probe", "src", "features", _] => Change::Probe,
        ["site", ..] => Change::Site,
        [".github", ..] | [".gitignore"] => Change::Docs,
        [first, ..] if ["yewi", "yewi-fonts", "yewi-test", "e2e", "tests", "xtask", "tools"].contains(first) => {
            Change::Foundations
        }
        [.., file] if file.starts_with("LICENSE") => Change::Docs,
        [.., file] if [".md", ".svg", ".png", ".jpg", ".txt"].iter().any(|ext| file.ends_with(ext)) => Change::Docs,
        _ => Change::Foundations,
    }
}

/// The checks a set of changes needs.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Plan {
    /// The first path that forces the full run, if any. Nothing else matters then.
    pub full: Option<String>,
    /// `cargo xtask check` rather than only the format check.
    pub check: bool,
    /// `cargo xtask size`.
    pub size: bool,
    /// The site spec and the sweeps over every page and example.
    pub site: bool,
    /// Pages (module names) whose examples and behaviour specs run.
    pub pages: BTreeSet<String>,
    /// Further behaviour specs, by file stem.
    pub specs: BTreeSet<String>,
}

impl Plan {
    /// `users` names the pages that show a component: its own and those of everything built on it.
    pub fn new(paths: &[String], users: impl Fn(&str) -> BTreeSet<String>) -> Self {
        let mut plan = Self::default();
        for path in paths {
            match classify(path) {
                Change::Docs => {}
                Change::Foundations => {
                    plan.full.get_or_insert_with(|| path.clone());
                }
                Change::Component(name) => {
                    plan.check = true;
                    plan.size = true;
                    plan.pages.extend(users(&name));
                }
                Change::Page(name) => {
                    plan.check = true;
                    plan.pages.insert(name);
                }
                Change::Spec(stem) => {
                    plan.specs.insert(stem);
                }
                Change::Site => {
                    plan.check = true;
                    plan.site = true;
                }
                Change::Probe => {
                    plan.check = true;
                    plan.size = true;
                }
            }
        }
        plan
    }

    /// The Playwright `--grep` pattern for the browser tests to run, if any. It is matched
    /// against "<project> <file> <describe> <test>".
    pub fn grep(&self) -> Option<String> {
        let mut any = Vec::new();
        if self.site {
            any.push(r"sweeps/".to_owned());
            any.push(spec("site"));
        }
        for page in &self.pages {
            let slug = page.replace('_', "-");
            if !self.site {
                // Example sweeps are described as "<slug>/<example>", page sweeps start with the route.
                any.push(format!(r" {slug}/"));
                any.push(format!(r"/components/{slug} hydrates"));
            }
            any.push(spec(page));
        }
        any.extend(self.specs.iter().map(|stem| spec(stem)));
        (!any.is_empty()).then(|| any.join("|"))
    }

    pub fn describe(&self) -> String {
        if let Some(path) = &self.full {
            return format!("full run: {path} is shared by every component");
        }
        let mut steps = vec![if self.check { "check".to_owned() } else { "format check".to_owned() }];
        if self.site {
            steps.push("site spec and every sweep".into());
        }
        if !self.pages.is_empty() {
            let pages: Vec<_> = self.pages.iter().map(String::as_str).collect();
            steps.push(format!("browser tests for {} page(s): {}", pages.len(), pages.join(", ")));
        }
        if !self.specs.is_empty() {
            let specs: Vec<_> = self.specs.iter().map(String::as_str).collect();
            steps.push(format!("specs: {}", specs.join(", ")));
        }
        if self.size {
            steps.push("size".into());
        }
        steps.join("; ")
    }
}

/// Matches a spec file whether it is named with underscores or hyphens.
fn spec(name: &str) -> String {
    let name: String = name.chars().map(|c| if c == '_' || c == '-' { "[_-]".into() } else { c.to_string() }).collect();
    format!(r"(^|[ /]){name}\.spec\.ts")
}

/// Paths that differ from where this branch left `base`, including uncommitted and untracked files.
pub fn changed_paths(root: &Path, base: &str) -> Vec<String> {
    let git = |args: &[&str]| -> String {
        let output = Command::new("git").args(args).current_dir(root).output().expect("failed to run git");
        if !output.status.success() {
            eprintln!("git {} failed: {}", args.join(" "), String::from_utf8_lossy(&output.stderr).trim());
            eprintln!("--changed compares with {base}: fetch it first, or name another ref with --changed=<ref>");
            exit(1);
        }
        String::from_utf8_lossy(&output.stdout).into_owned()
    };
    let fork = git(&["merge-base", base, "HEAD"]);
    let mut paths: BTreeSet<String> = BTreeSet::new();
    paths.extend(git(&["diff", "--name-only", "--no-renames", fork.trim()]).lines().map(str::to_owned));
    paths.extend(git(&["ls-files", "--others", "--exclude-standard"]).lines().map(str::to_owned));
    paths.into_iter().collect()
}

/// The pages that show `component`: its own, those of the components built on it, and every
/// page whose examples name one of those. Matched on source text, so it errs towards running
/// too much; what it cannot see (a component restyling another through a custom property
/// alone) is left to the full run.
pub fn pages_using(root: &Path, component: &str) -> BTreeSet<String> {
    let components = sources(&root.join("yewi/src/components"));
    let pages = sources(&root.join("site/src/pages"));

    let mut affected = BTreeSet::from([component.to_owned()]);
    let mut queue = vec![component.to_owned()];
    let mut names = Vec::new();
    while let Some(name) = queue.pop() {
        let Some((_, source)) = components.iter().find(|(other, _)| *other == name) else { continue };
        let exported = exports(&name, source);
        let class = [class(&name)];
        for (other, text) in &components {
            // Components import each other explicitly, so only imports count: a private item or
            // an enum variant that happens to share a name does not.
            if !affected.contains(other) && (mentions(&imports(text), &exported) || mentions(text, &class)) {
                affected.insert(other.clone());
                queue.push(other.clone());
            }
        }
        // Pages import the whole prelude, so there any distinctive name counts.
        names.extend(exported.into_iter().filter(|name| name.starts_with(char::is_uppercase) || name.contains('_')));
        names.extend(class);
    }

    pages
        .iter()
        .filter(|(page, text)| affected.contains(page) || mentions(text, &names))
        .map(|(page, _)| page.clone())
        .collect()
}

/// `(directory name, all of its Rust and SCSS)` for each directory in `dir`.
fn sources(dir: &Path) -> Vec<(String, String)> {
    fn read(dir: &Path, into: &mut String) {
        for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                read(&path, into);
            } else if matches!(path.extension().and_then(|e| e.to_str()), Some("rs" | "scss")) {
                // Comments name other components without using them.
                for line in fs::read_to_string(&path).unwrap_or_default().lines() {
                    if !line.trim_start().starts_with("//") {
                        into.push_str(line);
                        into.push('\n');
                    }
                }
            }
        }
    }
    let mut all = Vec::new();
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        if entry.path().is_dir() {
            let mut text = String::new();
            read(&entry.path(), &mut text);
            all.push((entry.file_name().to_string_lossy().into_owned(), text));
        }
    }
    all
}

/// The names other Rust code would import to use a component: its top-level items and its module.
fn exports(name: &str, source: &str) -> Vec<String> {
    const ITEMS: [&str; 7] = ["fn", "struct", "enum", "trait", "type", "const", "static"];
    let mut found = BTreeSet::from([format!("{name}::")]);
    // rustfmt leaves only top-level items unindented, which excludes methods and associated consts.
    for line in source.lines().filter(|line| line.starts_with("pub") && !line.starts_with("pub(super)")) {
        let mut words = line.split(|c: char| !(c.is_alphanumeric() || c == '_')).filter(|word| !word.is_empty());
        if let Some(item) = words.by_ref().skip_while(|word| !ITEMS.contains(word)).nth(1) {
            found.insert(item.to_owned());
        }
    }
    found.into_iter().collect()
}

/// The prefix of a component's CSS classes.
fn class(name: &str) -> String {
    format!("yi-{}", name.replace('_', "-"))
}

/// The parts of a Rust source that reach into the rest of the crate: `use crate::{..};`
/// statements and inline `crate::` paths, each up to the end of its statement.
fn imports(text: &str) -> String {
    let mut found = String::new();
    for root in ["crate::", "super::"] {
        for (at, _) in text.match_indices(root) {
            found.push_str(text[at..].split(';').next().unwrap_or_default());
            found.push('\n');
        }
    }
    found
}

/// Whether `text` uses any needle as a whole name: a longer name does not count, nor does
/// another type's variant or constant of the same name (`Language::Markdown`).
fn mentions(text: &str, needles: &[String]) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    needles.iter().any(|needle| {
        text.match_indices(needle.as_str()).any(|(at, _)| {
            let (before, after) = (&text[..at], &text[at + needle.len()..]);
            if needle.starts_with("yi-") {
                // `yi-tag__label` and `yi-tag--small` belong to Tag, `yi-tag-picker` does not. Custom
                // properties are left out: `--yi-duration-fast` is a token, not the Duration component.
                let continues = after.starts_with(|c: char| c.is_alphanumeric())
                    || (after.starts_with('-') && !after.starts_with("--"));
                let prefixed = before.ends_with(word) || before.ends_with('-');
                return !continues && !prefixed;
            }
            let owner = before.strip_suffix("::").map(|path| path.rsplit(|c| !word(c)).next().unwrap_or_default());
            let ends = needle.ends_with("::") || !after.starts_with(word);
            ends && !before.ends_with(word) && !owner.is_some_and(|owner| owner.starts_with(char::is_uppercase))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(paths: &[&str]) -> Plan {
        let paths: Vec<String> = paths.iter().map(|path| (*path).to_owned()).collect();
        // Stand-in for the source scan: every component is shown on its own page and on "gallery".
        Plan::new(&paths, |name| BTreeSet::from([name.to_owned(), "gallery".to_owned()]))
    }

    #[test]
    fn paths_map_to_what_they_affect() {
        let cases = [
            ("README.md", Change::Docs),
            ("docs/logo.svg", Change::Docs),
            ("LICENSE-THIRD-PARTY", Change::Docs),
            (".github/workflows/ci.yml", Change::Docs),
            ("yewi/src/components/split_button/mod.rs", Change::Component("split_button".into())),
            ("yewi/src/components/kbd/kbd.scss", Change::Component("kbd".into())),
            ("site/src/pages/kbd/examples/basic.rs", Change::Page("kbd".into())),
            ("e2e/baselines/split-button/basic-light-phone.png", Change::Page("split_button".into())),
            ("site/tests/snapshots/split-button__basic.snap", Change::Page("split_button".into())),
            ("e2e/tests/menu-button.spec.ts", Change::Spec("menu-button".into())),
            ("tests/size-probe/src/features/qr.rs", Change::Probe),
            ("site/src/shell.rs", Change::Site),
            ("site/styles/site.scss", Change::Site),
            ("site/index.html", Change::Site),
            ("yewi/scss/tokens.scss", Change::Foundations),
            ("yewi/src/common.rs", Change::Foundations),
            ("yewi/src/hooks/id.rs", Change::Foundations),
            ("yewi/README.md", Change::Foundations),
            ("xtask/src/main.rs", Change::Foundations),
            ("e2e/fixtures.ts", Change::Foundations),
            ("e2e/tests/sweeps/examples.spec.ts", Change::Foundations),
            ("Cargo.lock", Change::Foundations),
            ("size-budget.toml", Change::Foundations),
            ("something/new.rs", Change::Foundations),
        ];
        for (path, expected) in cases {
            assert_eq!(classify(path), expected, "{path}");
        }
    }

    #[test]
    fn docs_only_needs_no_tests() {
        let plan = plan(&["README.md", "CONTRIBUTING.md"]);
        assert_eq!(plan, Plan::default());
        assert_eq!(plan.grep(), None);
    }

    #[test]
    fn foundations_force_the_full_run() {
        let plan = plan(&["site/src/pages/kbd/mod.rs", "yewi/scss/tokens.scss"]);
        assert_eq!(plan.full.as_deref(), Some("yewi/scss/tokens.scss"));
    }

    #[test]
    fn a_component_runs_its_users_and_the_size_budget() {
        let plan = plan(&["yewi/src/components/kbd/kbd.scss"]);
        assert!(plan.full.is_none() && plan.check && plan.size && !plan.site);
        assert_eq!(plan.pages, BTreeSet::from(["kbd".to_owned(), "gallery".to_owned()]));
    }

    #[test]
    fn a_page_runs_only_itself() {
        let plan = plan(&["site/src/pages/split_button/examples/basic.rs"]);
        assert!(plan.check && !plan.size && !plan.site);
        assert_eq!(plan.pages, BTreeSet::from(["split_button".to_owned()]));
    }

    #[test]
    fn the_site_shell_runs_every_sweep() {
        let plan = plan(&["site/src/shell.rs", "README.md"]);
        assert!(plan.full.is_none() && plan.check && plan.site && !plan.size);
    }

    /// The pattern is a regular expression for Playwright; this checks its shape against the
    /// titles it has to tell apart without pulling in a regex engine.
    #[test]
    fn grep_selects_by_page_and_spec() {
        let pattern = plan(&["site/src/pages/split_button/mod.rs", "e2e/tests/use_id.spec.ts"]).grep().unwrap();
        for part in [
            " split-button/",
            "/components/split-button hydrates",
            r"split[_-]button\.spec\.ts",
            r"use[_-]id\.spec\.ts",
        ] {
            assert!(pattern.contains(part), "{pattern} should contain {part}");
        }
        assert!(!pattern.contains("sweeps/"));

        let pattern = plan(&["site/styles/site.scss"]).grep().unwrap();
        assert!(pattern.contains("sweeps/") && pattern.contains(r"site\.spec\.ts"), "{pattern}");
    }

    #[test]
    fn exports_are_top_level_items() {
        let source = "pub struct TagProps;\n#[function_component]\npub fn Tag() {}\npub(crate) fn tag_text() {}\n\
                      pub(super) fn inner() {}\nimpl Tag {\n    pub const ALL: u8 = 0;\n}\nfn private_helper() {}";
        let names = exports("tag", source);
        for expected in ["TagProps", "Tag", "tag_text", "tag::"] {
            assert!(names.iter().any(|name| name == expected), "{expected} in {names:?}");
        }
        for unexpected in ["inner", "ALL", "private_helper"] {
            assert!(!names.iter().any(|name| name == unexpected), "{unexpected} in {names:?}");
        }
    }

    #[test]
    fn imports_are_what_reaches_into_the_crate() {
        let found =
            imports("use crate::{\n    Attrs, Tag,\n};\nenum Kind { Kbd }\nlet x = crate::Icon::new(); let y = Local;");
        for expected in ["Attrs", "Tag", "Icon"] {
            assert!(found.contains(expected), "{expected} in {found}");
        }
        for unexpected in ["Kbd", "Local"] {
            assert!(!found.contains(unexpected), "{unexpected} in {found}");
        }
    }

    #[test]
    fn mentions_match_whole_names() {
        let needles = [exports("tag", "pub fn Tag() {}"), vec![class("tag")]].concat();
        let cases = [
            ("html! { <Tag>{\"a\"}</Tag> }", true),
            ("use crate::tag::helper;", true),
            (".yi-tag__label { }", true),
            (".yi-tag--small { }", true),
            ("class=\"yi-tag\"", true),
            ("html! { <TagPicker /> }", false),
            (".yi-tag-picker { }", false),
            ("--yi-tag-picker-gap: 0;", false),
            ("--yi-tag-bg: red;", false),
            ("let stage::x = 1;", false),
            ("Kind::Tag => {}", false),
            ("crate::Tag", true),
        ];
        for (text, expected) in cases {
            assert_eq!(mentions(text, &needles), expected, "{text}");
        }
    }
}
