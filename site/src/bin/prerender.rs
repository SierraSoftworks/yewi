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

    fs::write(dist.join("examples.json"), format!("[\n  {}\n]\n", examples.join(",\n  "))).unwrap();
    fs::write(dist.join("pages.json"), format!("[{}]\n", pages.join(", "))).unwrap();
    println!("prerendered {} routes ({} examples) into {}", routes.len(), examples.len(), dist.display());
}

const MOUNT: &str = r#"<div id="app"></div>"#;

async fn render(path: &str) -> String {
    yewi::hooks::reset_ids();
    let url = format!("{}{path}", yewi_site::basename().unwrap_or_default());
    LocalServerRenderer::<App>::with_props(AppProps {
        history: {
            let history = MemoryHistory::new();
            history.push(url);
            AnyHistory::from(history)
        },
    })
    .render()
    .await
}
