use yew::prelude::*;

use super::{plain, rust};

pub fn render() -> Html {
    html! {
        <>
            <h2>{ "Install" }</h2>
            <p>
                { "Add the crate and enable the rendering mode your app uses. Heavier components, such \
                   as charts, maps, code and Markdown, sit behind their own features, so your app only \
                   compiles the ones it asks for." }
            </p>
            { plain(r#"
[dependencies]
yew = { version = "0.23", features = ["csr"] }
yewi = { git = "https://github.com/SierraSoftworks/yewi", features = ["csr"] }
"#) }

            <h2>{ "Styles" }</h2>
            <p>
                { "The stylesheet for the features you enable is compiled into the crate, so there's \
                   nothing to copy or build. Choose whichever way of delivering it suits your app." }
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
                { " from a cached route and link to it in the document head, or render " }
                <code>{ "<Styles />" }</code>{ " in the head instead. A client that hydrates a server-rendered \
                   page shouldn't call " }
                <code>{ "install_styles" }</code>{ ", because the server has already delivered the styles." }
            </p>
            { rust(r#"
async fn stylesheet() -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(yewi::STYLESHEET)
}
"#) }

            <h2>{ "Fonts" }</h2>
            <p>
                { "yewi sets headings in Montserrat, body text in Inter and code in Fira Code. Each one \
                   falls back to a system font, so an app that doesn't ship any fonts still looks right. \
                   The font files live in the " }<code>{ "yewi-fonts" }</code>{ " crate, which has no dependencies." }
            </p>
            <h3>{ "Trunk apps" }</h3>
            <p>{ "Write the font files out from a build script and let Trunk copy them into your build." }</p>
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
                { "Serve every file in " }<code>{ "yewi_fonts::FILES" }</code>{ " under a single path prefix, then link \
                   the stylesheet returned by " }
                <code>{ "yewi_fonts::css(prefix)" }</code>{ "." }
            </p>
            <h3>{ "System fonts only" }</h3>
            { plain(r#"
:root {
  --yi-font-heading: var(--yi-font-system);
  --yi-font-body: var(--yi-font-system);
  --yi-font-mono: var(--yi-font-system-mono);
}
"#) }

            <h2>{ "Rendering modes" }</h2>
            <p>
                { "Every component renders the same markup on the server as it does on the client's first \
                   render, and it only uses browser APIs once it has mounted." }
            </p>
            <ul>
                <li><code>{ "csr" }</code>{ " renders in the browser, for apps built with Trunk." }</li>
                <li><code>{ "ssr" }</code>{ " renders on the server, with or without a client in the browser." }</li>
                <li><code>{ "hydration" }</code>{ " is the browser half of a server-rendered app." }</li>
            </ul>
            <p>
                { "Two hooks keep the server and client renders in agreement. Use " }<code>{ "use_id" }</code>
                { " for element ids and " }<code>{ "use_now" }</code>
                { " for the current time. The server passes its request time through " }<code>{ "NowProvider" }</code>
                { " and sends the same value to the client." }
            </p>

            <h2>{ "Theme before first paint" }</h2>
            <p>
                { "Inline " }<code>{ "yewi::THEME_SCRIPT" }</code>
                { " in the document head so that a saved light or dark choice is applied before anything \
                   is drawn. The Theming guide explains how this works." }
            </p>
        </>
    }
}
