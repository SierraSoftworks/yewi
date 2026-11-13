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
        summary: "Add yewi to an app, deliver its stylesheet and fonts, and pick a rendering mode.",
        render: getting_started::render,
    },
    Guide {
        slug: "theming",
        title: "Theming",
        summary: "Tokens, the per-app accent, light and dark themes, and how to override anything.",
        render: theming::render,
    },
    Guide {
        slug: "server-only",
        title: "Server-only apps",
        summary: "What each component does on a page that ships no wasm, and what to use where it does nothing.",
        render: server_only::render,
    },
    Guide {
        slug: "contributing",
        title: "Adding a component",
        summary: "Where a component's code, styles, documentation and tests live, and what done means.",
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
