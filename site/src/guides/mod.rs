//! Long-form guides. Unlike component pages these are an explicit list: add
//! the module and an entry to [`GUIDES`].

use yew::prelude::*;
use yewi::Highlight;

mod contributing;
mod getting_started;
mod server_only;
mod theming;

pub struct Guide {
    pub slug: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub render: fn() -> Html,
}

pub static GUIDES: &[Guide] = &[
    Guide {
        slug: "getting-started",
        title: "Getting started",
        summary: "Adding yewi to an app, serving its stylesheet and fonts, and choosing how your app renders.",
        render: getting_started::render,
    },
    Guide {
        slug: "theming",
        title: "Theming",
        summary: "How design tokens work, how to give your app its own accent colour, how light and dark themes are chosen, and how to override any style.",
        render: theming::render,
    },
    Guide {
        slug: "server-only",
        title: "Server-only apps",
        summary: "Using components on pages which don't ship wasm, the limitations this brings, and how to work around them.",
        render: server_only::render,
    },
    Guide {
        slug: "contributing",
        title: "Adding a component",
        summary: "Where a component's code, styles, documentation and tests live, and what needs to be in place before it's finished.",
        render: contributing::render,
    },
];

impl Guide {
    pub fn find(slug: &str) -> Option<&'static Guide> {
        GUIDES.iter().find(|guide| guide.slug == slug)
    }
}

/// A block of Rust source.
fn rust(source: &'static str) -> Html {
    html! { <pre class="docs-snippet"><Highlight code={source.trim()} language="rust" /></pre> }
}

/// A block of anything else (TOML, HTML, CSS, shell), shown plain.
fn plain(source: &str) -> Html {
    html! { <pre class="docs-snippet"><code>{ source.trim().to_owned() }</code></pre> }
}
