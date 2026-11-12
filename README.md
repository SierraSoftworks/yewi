<h1>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo-dark.svg" />
    <img src="assets/logo-light.svg" alt="yewi" height="56" />
  </picture>
</h1>

A design system and component library for [Yew](https://yew.rs): one visual language, light and dark themes, an accent per app, and a reference site that doubles as the test bed.

```rust
use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
fn App() -> Html {
    html! { <Button variant={ButtonVariant::Primary} icon={icons::PLAY}>{ "Start rollout" }</Button> }
}

fn main() {
    yewi::install_styles();
    yew::Renderer::<App>::new().render();
}
```

## Using it

### Rendering modes
- `csr`, `ssr` and `hydration` forward to the Yew features of the same name.
- Components render identical markup on the server and on the first client render.
- `use_id` and `use_now` keep ids and time consistent across the two.

### Styles and fonts
- The stylesheet for the enabled features is compiled into the crate: `install_styles()`, `<Styles />` or `STYLESHEET`.
- Everything is a `--yi-*` token inside `@layer yewi.*`; set `--yi-accent` and any unlayered app rule wins.
- Fonts ship in the dependency-free `yewi-fonts` crate, with a system fallback when they are not served.

### Opt-in features
- Heavier components sit behind features (`charts`, `map`, `code`, `markdown`, ...; `full` for all).
- A disabled feature adds no Rust, no CSS and no dependencies.
- `cargo xtask size` enforces budgets for the core wasm, the stylesheet and each feature.

## Working on it

Requires Rust with the `wasm32-unknown-unknown` target, [Trunk](https://trunkrs.dev) and Node 22.

| Command | Does |
|---|---|
| `cargo xtask serve` | The reference site with live reload |
| `cargo xtask check` | Format, lint, unit tests, snapshots, feature matrix |
| `cargo xtask e2e` | Build and prerender the site, run the browser tests, build the review gallery |
| `cargo xtask size` | Enforce `size-budget.toml` |
| `cargo xtask package` | Package the published crates as `cargo publish` would, uploading nothing |
| `cargo xtask ci` | All of the above |
| `cargo xtask ci --changed` | Only what the paths changed since `origin/main` call for |

### Layout
- `yewi/`: the library. `scss/` holds tokens and base styles, `src/components/<name>/` one component each.
- `site/`: the reference site. `src/pages/<name>/` holds a page and its examples.
- `e2e/`: Playwright tests, screenshot baselines and the review gallery.

### Adding a component
See [CONTRIBUTING.md](CONTRIBUTING.md).

## Licence

MIT. Bundled icons and fonts are covered by [LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY).
