//! Project automation. Run `cargo xtask` for the list of commands.
//!
//! Everything CI does is a command here, so the same checks run locally.

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio, exit},
};

const HELP: &str = "\
cargo xtask <command>

  check            format, lint, unit tests, snapshots and the feature matrix
  site             build the reference site and prerender it into site/dist
  serve            serve the site with live reload (client-rendered)
  e2e [args]       build the site, then run the browser tests and the gallery
                   (args go to Playwright, e.g. `e2e button --update-snapshots`)
  gallery [base]   rebuild e2e/gallery from the last run, diffed against a git ref
  size             measure the probe's wasm and the stylesheet per feature
                   and enforce size-budget.toml
  ci               check, e2e and size: everything, in order
    --changed[=ref]  only what the paths changed since `ref` (default origin/main)
                     call for; shared code still gets the full run
    --dry-run        with --changed, print the plan and stop
";

mod changed;

const WASM: &str = "wasm32-unknown-unknown";

fn main() {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_default();
    let rest: Vec<String> = args.collect();

    match command.as_str() {
        "check" => check(),
        "site" => site(),
        "serve" => run(Command::new("trunk").arg("serve").current_dir(root().join("site"))),
        "e2e" => e2e(&rest),
        "gallery" => gallery(&rest),
        "size" => size(),
        "ci" => ci(&rest),
        _ => {
            eprint!("{HELP}");
            exit(if command.is_empty() || command == "help" { 0 } else { 2 });
        }
    }
}

fn ci(args: &[String]) {
    let mut base = None;
    let mut dry_run = false;
    for arg in args {
        match arg.as_str() {
            "--changed" => base = Some("origin/main"),
            "--dry-run" => dry_run = true,
            _ => match arg.strip_prefix("--changed=") {
                Some(reference) => base = Some(reference),
                None => {
                    eprint!("unknown argument `{arg}`\n\n{HELP}");
                    exit(2);
                }
            },
        }
    }

    let plan = match base {
        Some(base) => {
            let paths = changed::changed_paths(&root(), base);
            let plan = changed::Plan::new(&paths, |component| changed::pages_using(&root(), component));
            eprintln!("\x1b[1m{} path(s) changed since {base}: {}\x1b[0m", paths.len(), plan.describe());
            plan
        }
        None => changed::Plan { full: Some("everything".into()), ..Default::default() },
    };
    if dry_run {
        return;
    }

    if plan.full.is_some() {
        check();
        e2e(&[]);
        size();
        return;
    }
    if plan.check {
        check();
    } else {
        fmt();
    }
    if let Some(grep) = plan.grep() {
        e2e(&["--grep".into(), grep]);
    }
    if plan.size {
        size();
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_owned()
}

fn cargo() -> Command {
    let mut command = Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
    command.current_dir(root());
    command
}

fn run(command: &mut Command) {
    eprintln!("\x1b[1m$ {} {}\x1b[0m", command.get_program().to_string_lossy(), {
        let args: Vec<_> = command.get_args().map(|a| a.to_string_lossy()).collect();
        args.join(" ")
    });
    match command.status() {
        Ok(status) if status.success() => {}
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(error) => {
            eprintln!("failed to run {}: {error}", command.get_program().to_string_lossy());
            exit(1);
        }
    }
}

/// The opt-in features of the `yewi` crate, read from its manifest.
fn optional_features() -> Vec<String> {
    let manifest = fs::read_to_string(root().join("yewi/Cargo.toml")).expect("yewi/Cargo.toml");
    let section = manifest.split("[features]").nth(1).unwrap_or_default();
    let section = section.split("\n[").next().unwrap_or_default();
    section
        .lines()
        .filter_map(|line| line.split_once('=').map(|(name, _)| name.trim().to_owned()))
        .filter(|name| !name.starts_with('#') && !name.contains(' '))
        .filter(|name| !["default", "csr", "ssr", "hydration", "full", "fonts"].contains(&name.as_str()))
        .collect()
}

fn fmt() {
    run(cargo().args(["fmt", "--all", "--check"]));
}

fn check() {
    fmt();
    run(cargo().args([
        "clippy",
        "--workspace",
        "--all-targets",
        "--features",
        "yewi-site/ssr",
        "--",
        "-D",
        "warnings",
    ]));
    run(cargo().args(["clippy", "-p", "yewi", "-p", "yewi-site", "--target", WASM]).args([
        "--features",
        "yewi/full,yewi-site/hydration",
        "--",
        "-D",
        "warnings",
    ]));
    run(cargo().args(["test", "--workspace", "--features", "yewi-site/ssr,yewi/full"]));

    // Each rendering mode and each opt-in feature must build on its own.
    for mode in ["csr", "hydration"] {
        run(cargo().args(["check", "-p", "yewi", "--target", WASM, "--features", mode]));
    }
    run(cargo().args(["check", "-p", "yewi", "--features", "ssr"]));
    for feature in optional_features() {
        run(cargo().args(["check", "-p", "yewi", "--target", WASM, "--features"]).arg(format!("csr,{feature}")));
    }
}

fn site() {
    let site = root().join("site");
    let base = env::var("YEWI_SITE_BASE").unwrap_or_else(|_| "/".into());
    run(Command::new("trunk").args(["build", "--release", "--public-url", &base]).current_dir(&site));
    run(cargo()
        .args(["run", "-p", "yewi-site", "--bin", "prerender", "--features", "ssr", "--"])
        .arg(site.join("dist")));
}

fn e2e(args: &[String]) {
    site();
    let e2e = root().join("e2e");
    if !e2e.join("node_modules").exists() {
        run(Command::new("npm").arg("ci").current_dir(&e2e));
    }
    // Playwright only rewrites a baseline that fails its comparison, so with the usual
    // tolerance a small change is never recorded. An update run compares exactly instead.
    let updating = args.iter().any(|arg| arg.starts_with("--update-snapshots") || arg == "-u");
    let mut test = Command::new("npx");
    test.args(["playwright", "test"]).args(args).current_dir(&e2e);
    if updating {
        test.env("YEWI_E2E_UPDATE", "1");
    }
    run(&mut test);
    gallery(&[]);
}

fn gallery(args: &[String]) {
    run(Command::new("node").arg("gallery.mjs").args(args).current_dir(root().join("e2e")));
}

fn size() {
    let budgets: BTreeMap<String, u64> = fs::read_to_string(root().join("size-budget.toml"))
        .expect("size-budget.toml")
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .filter_map(|line| line.split_once('='))
        .filter_map(|(name, bytes)| Some((name.trim().to_owned(), bytes.trim().parse().ok()?)))
        .collect();

    // (wasm code and data, stylesheet) in bytes. The probe does not embed the
    // stylesheet, so the two are measured apart.
    let measure = |features: &str| -> (u64, u64) {
        let mut build = cargo();
        build.args(["build", "-p", "size-probe", "--release", "--target", WASM, "--message-format=json"]);
        if !features.is_empty() {
            build.args(["--features", features]);
        }
        eprintln!("\x1b[1m$ cargo build -p size-probe --release --features '{features}'\x1b[0m");
        let output = build.stderr(Stdio::inherit()).output().expect("failed to run cargo");
        if !output.status.success() {
            exit(output.status.code().unwrap_or(1));
        }
        let wasm = root().join("target").join(WASM).join("release/size_probe.wasm");
        let wasm = code_and_data(&fs::read(&wasm).unwrap_or_else(|e| panic!("{}: {e}", wasm.display())));
        let css = stylesheet_dir(&String::from_utf8_lossy(&output.stdout)).join("yewi.css");
        (wasm, fs::metadata(&css).unwrap_or_else(|e| panic!("{}: {e}", css.display())).len())
    };

    let (core, stylesheet) = measure("");
    let mut rows = vec![
        ("core".to_owned(), core, budgets.get("core").copied()),
        ("stylesheet".to_owned(), stylesheet, budgets.get("stylesheet").copied()),
    ];
    for feature in optional_features() {
        let (wasm, css) = measure(&feature);
        let added = wasm.saturating_sub(core) + css.saturating_sub(stylesheet);
        rows.push((feature.clone(), added, budgets.get(&feature).copied()));
    }

    println!("\n{:<16} {:>10} {:>10}", "", "bytes", "budget");
    let mut failed = false;
    for (name, bytes, budget) in rows {
        let verdict = match budget {
            Some(budget) if bytes > budget => {
                failed = true;
                "OVER"
            }
            Some(_) => "",
            None => {
                failed = true;
                "NO BUDGET"
            }
        };
        let budget = budget.map_or_else(|| "-".into(), |b| b.to_string());
        println!("{name:<16} {bytes:>10} {budget:>10} {verdict}");
    }
    if failed {
        eprintln!("\nsize budget exceeded or missing: see size-budget.toml");
        exit(1);
    }
}

/// The `OUT_DIR` of the `yewi` build script, from cargo's JSON messages.
fn stylesheet_dir(messages: &str) -> PathBuf {
    messages
        .lines()
        .filter(|line| line.contains("\"build-script-executed\"") && line.contains("/yewi#"))
        .find_map(|line| {
            let path = line.split("\"out_dir\":\"").nth(1)?.split('"').next()?;
            Some(PathBuf::from(path.replace("\\\\", "\\")))
        })
        .expect("cargo did not report the yewi build script")
}

/// The bytes of a wasm module's code and data sections: what an app ships,
/// without the names, bindgen descriptors and exports that tooling strips.
fn code_and_data(wasm: &[u8]) -> u64 {
    const CODE: u8 = 10;
    const DATA: u8 = 11;

    let mut total = 0;
    let mut at = 8; // magic and version
    while at < wasm.len() {
        let id = wasm[at];
        at += 1;
        let (mut size, mut shift) = (0_u64, 0);
        loop {
            let byte = wasm[at];
            at += 1;
            size |= u64::from(byte & 0x7f) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        if id == CODE || id == DATA {
            total += size;
        }
        at += size as usize;
    }
    total
}
