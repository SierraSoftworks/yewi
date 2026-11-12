use yew::prelude::*;

use super::{plain, rust};

pub fn render() -> Html {
    html! {
        <>
            <h2>{ "Install" }</h2>
            <p>
                { "Add the crate and enable the rendering mode the app uses. Heavier components \
                   (charts, map, code, markdown and others) are separate features, so an app only \
                   compiles what it names." }
            </p>
            { plain(r#"
[dependencies]
yew = { version = "0.23", features = ["csr"] }
yewi = { git = "https://github.com/SierraSoftworks/yewi", features = ["csr"] }
"#) }

            <h2>{ "Styles" }</h2>
            <p>
                { "The stylesheet for the enabled features is compiled into the crate, so there is \
                   nothing to copy or build. Pick the delivery that fits the app." }
            </p>
            <h3>{ "Client-rendered (Trunk)" }</h3>
            { rust(r#"
fn main() {
    yewi::install_styles();
    yew::Renderer::<App>::new().render();
}
"#) }
            <h3>{ "Server-rendered" }</h3>
            <p>
                { "Serve " }<code>{ "yewi::STYLESHEET" }</code>
                { " from a cached route and link it in the document head, or render " }
                <code>{ "<Styles />" }</code>{ " there. A hydrating client must not call " }
                <code>{ "install_styles" }</code>{ ": the server already delivered the styles." }
            </p>
            { rust(r#"
async fn stylesheet() -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(yewi::STYLESHEET)
}
"#) }

            <h2>{ "Fonts" }</h2>
            <p>
                { "yewi sets headings in Montserrat, text in Inter and code in Fira Code, each with a \
                   system fallback, so an app that ships no fonts still renders correctly. The files \
                   live in the dependency-free " }<code>{ "yewi-fonts" }</code>{ " crate." }
            </p>
            <h3>{ "Trunk apps" }</h3>
            <p>{ "Export the files from a build script and let Trunk copy them." }</p>
            { rust(r#"
// build.rs, with `yewi-fonts` as a build dependency
fn main() {
    yewi_fonts::write_to("fonts").unwrap();
}
"#) }
            { plain(r#"
<link data-trunk rel="copy-dir" href="fonts" />
<link rel="stylesheet" href="fonts/fonts.css" />
"#) }
            <h3>{ "Servers" }</h3>
            <p>
                { "Serve each of " }<code>{ "yewi_fonts::FILES" }</code>{ " under one prefix and link the output of " }
                <code>{ "yewi_fonts::css(prefix)" }</code>{ "." }
            </p>
            <h3>{ "System fonts only" }</h3>
            { plain(r#"
:root {
  --yw-font-heading: var(--yw-font-system);
  --yw-font-body: var(--yw-font-system);
  --yw-font-mono: var(--yw-font-system-mono);
}
"#) }

            <h2>{ "Rendering modes" }</h2>
            <p>
                { "Every component renders the same markup on the server and on the first client \
                   render, and touches browser APIs only after mounting." }
            </p>
            <ul>
                <li><code>{ "csr" }</code>{ ": client rendering with Trunk." }</li>
                <li><code>{ "ssr" }</code>{ ": server rendering, with or without a client." }</li>
                <li><code>{ "hydration" }</code>{ ": the client half of a server-rendered app." }</li>
            </ul>
            <p>
                { "Two hooks keep the two renders in agreement: " }<code>{ "use_id" }</code>
                { " for element ids and " }<code>{ "use_now" }</code>
                { " for the time. A server passes its request time through " }<code>{ "NowProvider" }</code>
                { " and sends the same value to the client." }
            </p>

            <h2>{ "Theme before first paint" }</h2>
            <p>
                { "Inline " }<code>{ "yewi::THEME_SCRIPT" }</code>
                { " in the document head so a stored light or dark choice applies before anything is drawn. \
                   See the Theming guide." }
            </p>
        </>
    }
}
