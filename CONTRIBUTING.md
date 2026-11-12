# Contributing a component

A component is three directories and no shared files. Build scripts discover each one, so components developed in parallel never touch the same line. `Button` is the reference implementation: copy its shape.

```
yewi/src/components/<name>/      the component
    mod.rs                       props, enums, the function component
    <name>.scss                  styles
    tests.rs                     unit and server-render tests
site/src/pages/<name>/           its documentation
    mod.rs                       page! { title, category, summary, examples }
    examples/<scenario>.rs       one scenario each
e2e/tests/<name>.spec.ts         behaviour specs
```

`<name>` is a Rust module name (`split_button`). Related components share a directory (`layout` holds `Stack`, `Cluster` and `Grid`).

## The component

### Rust (`mod.rs`)
- A function component with a `#[derive(Properties, PartialEq)]` props struct. Doc comments on the component, each prop and each enum variant become the API tables.
- Every component takes `class: Classes`, `id: Option<AttrValue>` and `attrs: Attrs`, and wraps its root element in `props.attrs.apply(...)`. Events are `Callback` props named `on_*`.
- Reuse the shared vocabulary in `yewi/src/common.rs` (`Size`, `Tone`, `Space`) before defining a new enum. Large collections are `Rc<[T]>` or `IArray`, text is `AttrValue`.

### Server-safe rendering
- The first render must not depend on the browser. Touch `web_sys` only in effects and event handlers, behind `#[cfg(target_arch = "wasm32")]`.
- Use `use_id()` for element ids and `use_now()` for the time.
- Prefer state the browser already keeps (native elements, `:has()`, `popover`, `<dialog>`) over state in Rust.

### Opt-in features
- A component that needs a new dependency goes behind a Cargo feature. Start `mod.rs` with `#![cfg(feature = "<feature>")]`; the build script then skips its Rust and SCSS unless the feature is on.
- Features are already declared in `yewi/Cargo.toml` and `tests/size-probe/Cargo.toml`. Add the optional dependency to the feature's list.
- Add `tests/size-probe/src/features/<feature>.rs` with `pub fn view() -> yew::Html` so the size budget measures it.

## Styles (`<name>.scss`)

### Rules
- Tokens only: no literal colours, sizes, fonts, radii, shadows or durations. If a token is missing, add it to `yewi/scss/tokens.scss` and show it on a Foundations page.
- BEM classes prefixed `yw-`: `.yw-button`, `.yw-button--primary`, `.yw-button__spinner`. The build wraps the file in `@layer yewi.components`.
- `@use "api" as *;` for the shared mixins (`focus-ring`, `heading`, `card-surface`, `truncate`, `motion`, `eyebrow`).

### Component tokens
- Expose what an app may reasonably restyle as `--yw-<name>-*` custom properties, declared first under a `// Component tokens` comment. That block becomes the "CSS variables" table.
- Variants set those tokens rather than repeating properties.

### Both themes, all inputs
- Colours come from `light-dark()` tokens, so dark mode needs no extra rules. Check it anyway.
- Animate only inside `@include motion { ... }`. Give every interactive element a `:focus-visible` style.
- Use logical properties (`inline-size`, `padding-inline`) and container queries rather than viewport breakpoints.

## Documentation (`site/src/pages/<name>/`)

### The page
- `mod.rs` is one `crate::page! { ... }` call: `title`, `category`, `summary`, then the examples in display order. See `site/src/docs.rs` for the optional `feature` and `api` fields.
- At least three examples, each a real scenario with one or two sentences of guidance.

### Examples
- Each `examples/<scenario>.rs` defines `#[function_component] pub fn Example() -> Html`. The file is mounted live and shown verbatim as the source, so write it as the code a user should copy.
- Examples must be deterministic: no randomness, no network, no real clock (`use_now` is frozen to 2026-01-15 12:00 UTC on the site).
- Show state changes in the example itself (a `role="status"` line), so behaviour specs can assert on what a user sees.

## Tests

### What you get without writing anything
- A server-render snapshot per example (`site/tests/snapshots.rs`).
- Per example: screenshots in light and dark at desktop and phone widths, an axe accessibility scan, and a check that hydration leaves the server markup unchanged (`e2e/tests/sweeps/`).

### What you write
- `tests.rs`: pure logic and server-render assertions with `yewi_test::render`. Assert on text, roles and ARIA state, not class names or exact markup. Parameterise over variants.
- `e2e/tests/<name>.spec.ts`: one spec per interaction (mouse, keyboard, focus), driven through the `example("<page>/<scenario>")` fixture and role-based locators. These are recorded on video.

### Updating recorded output
- Snapshots: `INSTA_UPDATE=always cargo test -p yewi-site`, then review the diff.
- Screenshots: `cargo xtask e2e --update-snapshots`, then review `e2e/gallery/index.html`, which shows before and after for everything that changed. Baselines are made with the Chromium that Playwright 1.63 installs, on Linux.

## Definition of done

- `cargo xtask ci` passes locally. While the repository is private this, not GitHub Actions, is the verification.
- The page has at least three examples and reads correctly in both themes.
- Keyboard and screen-reader behaviour is complete and covered by a behaviour spec.
- New screenshots are committed, and the pull request links or attaches the review gallery.
