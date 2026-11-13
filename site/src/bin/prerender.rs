//! Prerenders every route into a Trunk `dist` directory, so each page is real
//! HTML that the wasm then hydrates.
//!
//!   prerender <dist>
//!
//! Besides the pages it writes `yewi.css`, plus `examples.json` and
//! `pages.json`: the routes the browser tests iterate over.

use std::{env, fs, path::Path};

use yew::LocalServerRenderer;
use yew_router::{
    Routable,
    history::{AnyHistory, History, MemoryHistory},
};
use yewi_site::{App, AppProps, BASE, Route, guides::GUIDES, pages::PAGES};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let dist = env::args().nth(1).expect("usage: prerender <dist>");
    let dist = Path::new(&dist);

    let template = fs::read_to_string(dist.join("index.html")).expect("run `trunk build` first: no index.html in dist");
    assert!(template.contains(MOUNT), "index.html has no {MOUNT} mount point");
    let template = without_vendor_preloads(&template);

    fs::write(dist.join("yewi.css"), yewi::STYLESHEET).unwrap();

    let mut routes = vec![Route::Home, Route::NotFound];
    routes.extend(GUIDES.iter().map(|guide| Route::guide(guide.slug)));
    routes.extend(PAGES.iter().map(|page| Route::component(page.slug)));
    let pages: Vec<String> = routes.iter().map(|route| format!("{:?}", route.to_path())).collect();

    let mut examples = Vec::new();
    for page in PAGES {
        for example in page.def.examples {
            let route = Route::Example { page: page.slug.to_owned(), example: example.slug() };
            examples.push(format!(
                r#"{{"page":{:?},"example":{:?},"title":{:?},"path":{:?}}}"#,
                page.slug,
                example.slug(),
                format!("{}: {}", page.def.title, example.title),
                route.to_path(),
            ));
            routes.push(route);
        }
    }

    let local = tokio::task::LocalSet::new();
    for route in &routes {
        let path = route.to_path();
        let body = local.run_until(render(&path)).await;
        let html = template
            .replace(MOUNT, &format!(r#"<div id="app" data-prerendered>{body}</div>"#))
            .replace("</head>", &format!(r#"<link rel="stylesheet" href="{BASE}yewi.css" id="yewi-styles"></head>"#));

        let file = match route {
            Route::Home => "index.html".to_owned(),
            Route::NotFound => "404.html".to_owned(),
            _ => format!("{}.html", path.trim_start_matches('/')),
        };
        let target = dist.join(file);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, html).unwrap();
    }

    // The live review gallery: not prerendered, the wasm starts it (`main.rs`).
    fs::write(dist.join("_gallery.html"), template.replace(MOUNT, r#"<div id="app" data-gallery="button"></div>"#))
        .unwrap();

    fs::write(dist.join("examples.json"), format!("[\n  {}\n]\n", examples.join(",\n  "))).unwrap();
    fs::write(dist.join("pages.json"), format!("[{}]\n", pages.join(", "))).unwrap();
    println!("prerendered {} routes ({} examples) into {}", routes.len(), examples.len(), dist.display());
}

const MOUNT: &str = r#"<div id="app"></div>"#;

/// Trunk preloads every wasm-bindgen snippet, which would have each page fetch
/// the scripts a component only imports on demand (the map engine, more than
/// 1 MB). Those live in a `vendor` directory beside the component's glue.
fn without_vendor_preloads(template: &str) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("<link ") {
        let end = start + rest[start..].find('>').expect("unclosed <link>") + 1;
        let link = &rest[start..end];
        out.push_str(&rest[..start]);
        if !(link.contains(r#"rel="modulepreload""#) && link.contains("/vendor/")) {
            out.push_str(link);
        }
        rest = &rest[end..];
    }
    out + rest
}

async fn render(path: &str) -> String {
    yewi::hooks::reset_ids();
    let url = format!("{}{path}", yewi_site::basename().unwrap_or_default());
    LocalServerRenderer::<App>::with_props(AppProps {
        gallery: false,
        history: {
            let history = MemoryHistory::new();
            history.push(url);
            AnyHistory::from(history)
        },
    })
    .render()
    .await
}
