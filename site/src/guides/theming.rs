use yew::prelude::*;

use super::plain;

pub fn render() -> Html {
    html! {
        <>
            <h2>{ "Tokens" }</h2>
            <p>
                { "Every colour, size, radius, shadow and duration that a component uses is a CSS custom \
                   property named " }<code>{ "--yi-*" }</code>
                { ". Components never hard-code these values, so when you change a token the whole \
                   system follows. You'll find them all listed on the Foundations pages." }
            </p>

            <h2>{ "One accent per app" }</h2>
            <p>
                { "Set " }<code>{ "--yi-accent" }</code>
                { " and the rest of the system follows, including primary buttons, links, focus rings and \
                   selections. The neutral greys lean slightly towards the accent's hue, so a blue accent \
                   gives you slate greys and a green one gives you graphite." }
            </p>
            { plain(r#"
:root {
  --yi-accent: #b9852f;
  /* Optional: 0 for pure greys, 1 (default) for the full lean. */
  --yi-neutral-tint: 0.6;
}
"#) }
            <p>
                { "Fills and text derived from the accent have their lightness kept within a safe range, \
                   so whichever accent you pick stays readable in both themes." }
            </p>

            <h2>{ "Light and dark" }</h2>
            <p>
                { "Colour tokens use " }<code>{ "light-dark()" }</code>
                { ", so " }<code>{ "color-scheme" }</code>
                { " is all that decides the theme. If you don't set an attribute, the page follows the \
                   system setting. " }<code>{ "ThemeToggle" }</code>
                { " sets " }<code>{ "data-theme" }</code>{ " on the root element and remembers the reader's choice. \
                   You can also set it on a single region." }
            </p>
            { plain(r#"
<html data-theme="dark">           <!-- the whole page -->
<section data-theme="dark">        <!-- or just a region -->
"#) }

            <h2>{ "Overriding styles" }</h2>
            <p>
                { "All of yewi's rules live in cascade layers (" }<code>{ "@layer yewi.*" }</code>
                { "). Any rule your app writes outside a layer wins regardless of its specificity, so you \
                   never need " }<code>{ "!important" }</code>{ " to override a style. Where you can, change a \
                   component's own tokens, which are listed under CSS variables on its page." }
            </p>
            { plain(r#"
.danger-zone .yi-button {
  --yi-button-border: var(--yi-danger);
}
"#) }

            <h2>{ "Matching styles of your own" }</h2>
            <p>
                { "The parts of a page that are yours alone can be built from the same Sass mixins as the                    components: " }<code>{ "card-surface" }</code>{ ", " }<code>{ "focus-ring" }</code>{ ", " }
                <code>{ "heading" }</code>{ ", " }<code>{ "eyebrow" }</code>{ ", " }<code>{ "chip" }</code>{ ", " }
                <code>{ "truncate" }</code>{ " and " }<code>{ "motion" }</code>
                { ". Cargo gives the build script of a crate that depends on yewi the path of the module as " }
                <code>{ "DEP_YEWI_SCSS" }</code>{ ". Copy it beside your stylesheet (and keep the copy out of version                    control), then use it by name. Servers that compile Sass themselves can read the same text from " }
                <code>{ "yewi::SCSS" }</code>{ "." }
            </p>
            { plain(r#"
// build.rs
let mixins = std::env::var("DEP_YEWI_SCSS").expect("yewi is a direct dependency");
std::fs::copy(mixins, "_yewi.scss").expect("the mixins should be writable");
"#) }
            { plain(r#"
@use "yewi" as yi;

.result-row {
  @include yi.card-surface;
}
"#) }

            <h2>{ "Density" }</h2>
            <p>
                { "Controls are 32px tall by default. To fit more into a busy area, such as a table toolbar, \
                   mark it with " }<code>{ "data-yi-density=\"compact\"" }</code>{ " and its controls shrink to 28px." }
            </p>
        </>
    }
}
