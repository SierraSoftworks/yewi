use yew::prelude::*;

use super::plain;

pub fn render() -> Html {
    html! {
        <>
            <h2>{ "Tokens" }</h2>
            <p>
                { "Every colour, size, radius, shadow and duration a component uses is a CSS custom \
                   property named " }<code>{ "--yi-*" }</code>
                { ". Components never contain literal values, so changing a token changes the whole \
                   system. The Foundations pages list them." }
            </p>

            <h2>{ "One accent per app" }</h2>
            <p>
                { "Set " }<code>{ "--yi-accent" }</code>
                { " and everything follows: the primary button fill, links, focus rings and selection. \
                   The neutral greys lean towards the accent's hue, so a blue accent gives slate greys \
                   and a green one graphite." }
            </p>
            { plain(r#"
:root {
  --yi-accent: #b9852f;
  /* Optional: 0 for pure greys, 1 (default) for the full lean. */
  --yi-neutral-tint: 0.6;
}
"#) }
            <p>
                { "Fills and text derived from the accent have their lightness clamped, so any accent \
                   keeps readable contrast in both themes." }
            </p>

            <h2>{ "Light and dark" }</h2>
            <p>
                { "Colour tokens use " }<code>{ "light-dark()" }</code>
                { ", so the theme is decided by " }<code>{ "color-scheme" }</code>
                { " alone. With no attribute the page follows the system. " }<code>{ "ThemeToggle" }</code>
                { " sets " }<code>{ "data-theme" }</code>{ " on the root element and remembers the choice." }
            </p>
            { plain(r#"
<html data-theme="dark">           <!-- the whole page -->
<section data-theme="dark">        <!-- or just a region -->
"#) }

            <h2>{ "Overriding styles" }</h2>
            <p>
                { "All yewi rules live in cascade layers (" }<code>{ "@layer yewi.*" }</code>
                { "). Any rule an app writes outside a layer wins, whatever its specificity, so overrides \
                   never need " }<code>{ "!important" }</code>{ ". Prefer a component's own tokens, listed under \
                   CSS variables on its page:" }
            </p>
            { plain(r#"
.danger-zone .yi-button {
  --yi-button-border: var(--yi-danger);
}
"#) }

            <h2>{ "Density" }</h2>
            <p>
                { "Controls are 32px tall by default. Mark a dense region, such as a table toolbar, \
                   with " }<code>{ "data-yi-density=\"compact\"" }</code>{ " to drop them to 28px." }
            </p>
        </>
    }
}
