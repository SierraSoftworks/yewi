use yew::prelude::*;

use super::plain;

pub fn render() -> Html {
    html! {
        <>
            <p>
                { "Each component lives in three directories and doesn't touch any shared files. The build \
                   scripts find each one automatically, so people working on different components in \
                   parallel never edit the same file. " }
                <a href="https://github.com/SierraSoftworks/yewi/blob/main/CONTRIBUTING.md">{ "CONTRIBUTING.md" }</a>
                { " describes everything a component needs. This page is a quick map of where things go." }
            </p>
            { plain(r#"
yewi/src/components/<name>/
    mod.rs            the component, its props and enums
    <name>.scss       its styles, tokens only
    tests.rs          unit and server-render tests

site/src/pages/<name>/
    mod.rs            page! { title, category, summary, examples }
    examples/*.rs     one scenario each: mounted live and shown as source

e2e/tests/<name>.spec.ts
                      behaviour specs for the interactions
"#) }
            <h2>{ "What you get for free" }</h2>
            <ul>
                <li>{ "The page shows up in the sidebar and in search, and its API tables are built from your doc comments." }</li>
                <li>{ "Every example is checked with a server-render snapshot, a hydration test, an accessibility scan and screenshots in both themes." }</li>
                <li>{ "The component's SCSS is added to the stylesheet whenever its feature is enabled." }</li>
            </ul>
            <h2>{ "Checks" }</h2>
            { plain(r#"
cargo xtask check       # format, lint, unit tests, snapshots, feature matrix
cargo xtask e2e         # build the site, prerender it, run the browser tests
cargo xtask serve       # the site with live reload
"#) }
        </>
    }
}
