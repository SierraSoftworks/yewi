use yew::prelude::*;

use super::plain;

pub fn render() -> Html {
    html! {
        <>
            <p>
                { "A component is three directories and no shared files. Build scripts discover each \
                   one, so parallel work never collides. " }
                <a href="https://github.com/SierraSoftworks/yewi/blob/main/CONTRIBUTING.md">{ "CONTRIBUTING.md" }</a>
                { " has the full contract; this is the map." }
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
            <h2>{ "What comes for free" }</h2>
            <ul>
                <li>{ "The page appears in the sidebar and search, with API tables read from the doc comments." }</li>
                <li>{ "Every example gets a server-render snapshot, a hydration check, an accessibility scan and screenshots in both themes." }</li>
                <li>{ "The stylesheet includes the component's SCSS whenever its feature is enabled." }</li>
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
