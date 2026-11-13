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
- Add `tests/size-probe/src/features/<feature>.rs` with `pub fn view() -> yew::Html` so the size budget measures it. Core components count against the `stylesheet` budget only; their code is paid by the apps that use them.

## Styles (`<name>.scss`)

### Rules
- Tokens only: no literal colours, sizes, fonts, radii, shadows or durations. If a token is missing, add it to `yewi/scss/tokens.scss` and show it on a Foundations page.
- BEM classes prefixed `yi-`: `.yi-button`, `.yi-button--primary`, `.yi-button__spinner`. The build wraps the file in `@layer yewi.components`.
- `@use "api" as *;` for the shared mixins (`focus-ring`, `heading`, `card-surface`, `code-frame`, `truncate`, `motion`, `eyebrow`) and the tone lists (`$tones`, `$tones-coloured`).

### Component tokens
- Expose what an app may reasonably restyle as `--yi-<name>-*` custom properties, declared under a `// Component tokens` comment in a `:where(.yi-<name>) { ... }` rule ahead of the component's own rule. That block becomes the "CSS variables" table.
- `:where()` gives the defaults no specificity, so any single class sets them whatever order the files compile in: `.yi-menu-button__surface { --yi-popover-radius: ...; }`. Ordinary properties on another component's element still follow file order, so those need both classes (`.yi-popover__surface.yi-menu-button__surface`).
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
- `e2e/tests/<name>.spec.ts`: one spec per interaction (mouse, keyboard, focus), driven through the `example("<page>/<scenario>")` fixture and role-based locators. Each is recorded twice: on video, and as the input events it caused, which the review gallery replays.

### The review gallery
- `cargo xtask e2e` (or `cargo xtask gallery` after a run) writes `e2e/gallery/viewer.html`: one self-contained file with every example running live, a menu of components, and theme, accent and stage width controls.
- Each behaviour spec appears under the example it drove as a replay: the recorded events are dispatched again at the live component. Nothing needs writing for this, but a spec that reaches an example through `example(...)` and real input (`click`, `hover`, `keyboard.press`) replays best.
- `node e2e/verify-replays.mjs` plays every recording and reports those that end in a different state from the spec. Replays cannot reproduce a faked clock, files, the clipboard, events a spec dispatches itself, or viewport-sized layouts (`AppShell`).

### Updating recorded output
- Snapshots: `INSTA_UPDATE=always cargo test -p yewi-site`, then review the diff.
- Screenshots: `cargo xtask e2e --update-snapshots` compares with no pixel tolerance, so it records every visible change however small and leaves unchanged baselines alone. Then review `e2e/gallery/index.html`, which shows before and after for everything that changed. Baselines are made with the Chromium that Playwright 1.63 installs, on Linux.

### Checking a change
- `cargo xtask ci --changed` runs what the paths changed since `origin/main` call for (fetch it first; `--changed=<ref>` compares with another ref, `--dry-run` prints the plan and stops).
- `cargo xtask ci` runs everything. Use it for shared code and to verify `main`.

| Changed | Runs |
| --- | --- |
| Docs, licences, `.github/` | format check |
| `site/src/pages/<name>/`, its baselines and snapshots | `check`, that page's sweeps and behaviour spec |
| `yewi/src/components/<name>/` | the same for its page and every page that uses it, plus `size` |
| `e2e/tests/<name>.spec.ts` | that spec |
| The rest of `site/` (shell, guides, styles) | `check`, the site spec, every sweep |
| Anything else (`yewi/scss/`, `common.rs`, hooks, icons, manifests, `xtask/`, the e2e harness) | everything |

"Uses" is read from the source: components that import the changed one, then pages whose examples name any of them. A component that only restyles another through its `--yi-<name>-*` properties is not seen, so run the full check when you change how those behave.

## Definition of done

- `cargo xtask ci --changed` passes locally with the latest `main` merged in. While the repository is private this, not GitHub Actions, is the verification.
- The page has at least three examples and reads correctly in both themes.
- Keyboard and screen-reader behaviour is complete and covered by a behaviour spec.
- New screenshots are committed, and the pull request links or attaches the review gallery (`e2e/gallery/viewer.html`). The screenshot comparison is `e2e/gallery/index.html`; `node e2e/inline-gallery.mjs e2e/gallery gallery.html` embeds its images and videos for viewers that open a single file.
