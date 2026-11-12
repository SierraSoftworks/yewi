//! The site's own chrome: layout, navigation, example blocks and API tables.
//! These are site components, styled in `styles/site.scss` with yewi tokens.

use yew::prelude::*;
// Named explicitly: `yewi::prelude` also exports a `Link`.
use yew_router::prelude::{Link, *};
use yewi::prelude::*;

use crate::{
    EXAMPLE_NOW_MILLIS, Route,
    docs::{ApiModule, ApiProp, Category, Example, Page},
    guides::{self, Guide},
    highlight,
};

const REPOSITORY: &str = "https://github.com/SierraSoftworks/yewi";

/// Renders text with `backticks` as inline code.
pub fn inline(text: &str) -> Html {
    text.split('`')
        .enumerate()
        .map(|(i, part)| {
            if i % 2 == 1 {
                html! { <code>{ part }</code> }
            } else {
                html! { { part } }
            }
        })
        .collect()
}

// ---- Layout -----------------------------------------------------------------

#[derive(Properties, PartialEq)]
struct LayoutProps {
    #[prop_or_default]
    children: Html,
    /// `(anchor, label)` entries for the "On this page" column.
    #[prop_or_default]
    toc: Vec<(String, String)>,
}

#[function_component]
fn Layout(props: &LayoutProps) -> Html {
    let nav_open = use_state(|| false);
    let route = use_route::<Route>();
    {
        // Close the narrow-screen menu after navigating.
        let nav_open = nav_open.clone();
        use_effect_with(route, move |_| nav_open.set(false));
    }
    let toggle_nav = {
        let nav_open = nav_open.clone();
        Callback::from(move |_| nav_open.set(!*nav_open))
    };

    html! {
        <div class="docs">
            <a class="docs-skip" href="#content">{ "Skip to content" }</a>
            <header class="docs-header">
                <Button
                    class="docs-header__menu yi-button--icon-only"
                    variant={ButtonVariant::Subtle}
                    attrs={attrs! { "aria-label" => "Menu", "aria-expanded" => if *nav_open { "true" } else { "false" } }}
                    on_click={toggle_nav}
                >
                    <Icon icon={icons::MENU} />
                </Button>
                <Link<Route> classes="docs-brand" to={Route::Home}>{ "yewi" }</Link<Route>>
                <Search />
                <nav class="docs-header__links" aria-label="Site">
                    <Link<Route> to={Route::guide("getting-started")}>{ "Guide" }</Link<Route>>
                    <Link<Route> to={Route::component("button")}>{ "Components" }</Link<Route>>
                    <a href={REPOSITORY}>{ "GitHub" }</a>
                    <span class="docs-header__version">{ concat!("v", env!("CARGO_PKG_VERSION")) }</span>
                </nav>
                <ThemeToggle />
            </header>
            <div class="docs-body">
                <nav class={classes!("docs-sidebar", nav_open.then_some("docs-sidebar--open"))} aria-label="Pages">
                    <Sidebar />
                </nav>
                <main class="docs-main" id="content">{ props.children.clone() }</main>
                <aside class="docs-toc" aria-label="On this page">
                    if !props.toc.is_empty() {
                        <span class="docs-toc__title">{ "On this page" }</span>
                        { for props.toc.iter().map(|(anchor, label)| html! {
                            <a href={format!("#{anchor}")}>{ label }</a>
                        }) }
                    }
                </aside>
            </div>
        </div>
    }
}

#[function_component]
fn Sidebar() -> Html {
    let current = use_route::<Route>();
    let link = |route: Route, label: &str| {
        let active = current.as_ref() == Some(&route);
        html! {
            <Link<Route> classes={classes!("docs-sidebar__link", active.then_some("docs-sidebar__link--active"))} to={route}>
                { label }
            </Link<Route>>
        }
    };

    html! {
        <>
            <span class="docs-sidebar__group">{ "Guide" }</span>
            { for guides::GUIDES.iter().map(|guide| link(Route::guide(guide.slug), guide.title)) }
            { for Category::ALL.into_iter().map(|category| {
                let mut pages = Page::in_category(category).peekable();
                if pages.peek().is_none() {
                    return html! {};
                }
                html! {
                    <>
                        <span class="docs-sidebar__group">{ category.label() }</span>
                        { for pages.map(|page| link(Route::component(page.slug), page.def.title)) }
                    </>
                }
            }) }
        </>
    }
}

// ---- Search -----------------------------------------------------------------

#[function_component]
fn Search() -> Html {
    let query = use_state(String::new);
    let on_input = {
        let query = query.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            query.set(input.value());
        })
    };
    let clear = {
        let query = query.clone();
        Callback::from(move |_: MouseEvent| query.set(String::new()))
    };

    let needle = query.trim().to_lowercase();
    let results: Vec<(Route, &str, &str)> = if needle.is_empty() {
        Vec::new()
    } else {
        let matches = |text: &str| text.to_lowercase().contains(&needle);
        let pages = crate::pages::PAGES
            .iter()
            .filter(|p| {
                matches(p.def.title) || matches(p.def.summary) || p.def.examples.iter().any(|e| matches(e.title))
            })
            .map(|p| (Route::component(p.slug), p.def.title, p.def.summary));
        let guides = guides::GUIDES
            .iter()
            .filter(|g| matches(g.title) || matches(g.summary))
            .map(|g| (Route::guide(g.slug), g.title, g.summary));
        pages.chain(guides).take(8).collect()
    };

    html! {
        <div class="docs-search" role="search">
            <Icon icon={icons::SEARCH} />
            <input
                type="search"
                placeholder="Search"
                aria-label="Search the documentation"
                value={(*query).clone()}
                oninput={on_input}
            />
            if !needle.is_empty() {
                <div class="docs-search__results">
                    if results.is_empty() {
                        <span class="docs-search__empty">{ "No matches" }</span>
                    }
                    { for results.into_iter().map(|(route, title, summary)| html! {
                        <span onclick={clear.clone()}>
                            <Link<Route> classes="docs-search__result" to={route}>
                                <b>{ title }</b>
                                <small>{ summary }</small>
                            </Link<Route>>
                        </span>
                    }) }
                </div>
            }
        </div>
    }
}

// ---- Pages ------------------------------------------------------------------

#[function_component]
pub fn Home() -> Html {
    html! {
        <Layout>
            <header class="docs-page-header">
                <h1>{ "yewi" }</h1>
                <p class="docs-lede">
                    { "A design system for Yew applications: calm by default, decisive where it matters. \
                       One component library, light and dark themes, an accent per app." }
                </p>
                <Cluster>
                    <Button variant={ButtonVariant::Primary} href={href(&Route::guide("getting-started"))}>
                        { "Get started" }
                    </Button>
                    <Button href={href(&Route::component("button"))}>{ "Browse components" }</Button>
                </Cluster>
            </header>
            { for Category::ALL.into_iter().map(|category| {
                let mut pages = Page::in_category(category).peekable();
                if pages.peek().is_none() {
                    return html! {};
                }
                html! {
                    <section class="docs-section">
                        <h2 class="docs-eyebrow">{ category.label() }</h2>
                        <Grid min="15rem" gap={Space::Md}>
                            { for pages.map(|page| html! {
                                <Link<Route> classes="docs-card" to={Route::component(page.slug)}>
                                    <b>{ page.def.title }</b>
                                    <span>{ page.def.summary }</span>
                                </Link<Route>>
                            }) }
                        </Grid>
                    </section>
                }
            }) }
        </Layout>
    }
}

/// A site-relative link target, including the base path.
fn href(route: &Route) -> String {
    format!("{}{}", crate::basename().unwrap_or_default(), route.to_path())
}

#[derive(Properties, PartialEq)]
pub struct SlugProps {
    pub slug: String,
}

#[function_component]
pub fn GuidePage(props: &SlugProps) -> Html {
    let Some(guide) = Guide::find(&props.slug) else {
        return html! { <NotFound /> };
    };
    html! {
        <Layout>
            <header class="docs-page-header">
                <span class="docs-eyebrow">{ "Guide" }</span>
                <h1>{ guide.title }</h1>
                <p class="docs-lede">{ guide.summary }</p>
            </header>
            <div class="docs-prose">{ (guide.render)() }</div>
        </Layout>
    }
}

#[function_component]
pub fn ComponentPage(props: &SlugProps) -> Html {
    let Some(page) = Page::find(&props.slug) else {
        return html! { <NotFound /> };
    };
    let api: Vec<&ApiModule> = page.api().collect();

    let mut toc: Vec<(String, String)> =
        page.def.examples.iter().map(|example| (example.slug(), example.title.to_owned())).collect();
    if !api.is_empty() {
        toc.push(("api".into(), "API".into()));
    }

    html! {
        <Layout {toc}>
            <header class="docs-page-header">
                <span class="docs-eyebrow">{ page.def.category.label() }</span>
                <h1>{ page.def.title }</h1>
                <p class="docs-lede">{ inline(page.def.summary) }</p>
                if let Some(feature) = page.def.feature {
                    <p class="docs-feature">
                        { "Opt-in: enable the " }<code>{ feature }</code>{ " feature." }
                    </p>
                }
            </header>
            { for page.def.examples.iter().map(|example| html! { <ExampleBlock {page} {example} /> }) }
            if !api.is_empty() {
                <section class="docs-section" id="api">
                    <h2>{ "API" }</h2>
                    { for api.into_iter().map(api_module) }
                </section>
            }
        </Layout>
    }
}

#[derive(Properties)]
struct ExampleBlockProps {
    page: &'static Page,
    example: &'static Example,
}

impl PartialEq for ExampleBlockProps {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.page, other.page) && std::ptr::eq(self.example, other.example)
    }
}

#[function_component]
fn ExampleBlock(props: &ExampleBlockProps) -> Html {
    let ExampleBlockProps { page, example } = *props;
    let slug = example.slug();
    let source_id = format!("{slug}-source");
    let isolated = Route::Example { page: page.slug.to_owned(), example: slug.clone() };

    let open = use_state(|| false);
    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };
    let copy = Callback::from(move |_| copy_to_clipboard(example.source));

    html! {
        <section class="docs-example" id={slug}>
            <h2>{ example.title }</h2>
            <p>{ inline(example.description) }</p>
            <div class="docs-example__frame">
                <div class="docs-example__stage">
                    <Stage {example} />
                </div>
                <div class="docs-example__tools">
                    <Button
                        size={Size::Small}
                        variant={ButtonVariant::Subtle}
                        icon={icons::CODE}
                        attrs={attrs! {
                            "aria-expanded" => if *open { "true" } else { "false" },
                            "aria-controls" => source_id.clone(),
                        }}
                        on_click={toggle}
                    >
                        { if *open { "Hide source" } else { "Show source" } }
                    </Button>
                    <Button size={Size::Small} variant={ButtonVariant::Subtle} icon={icons::COPY} on_click={copy}>
                        { "Copy" }
                    </Button>
                    <Button size={Size::Small} variant={ButtonVariant::Subtle} icon={icons::MAXIMIZE_2} href={href(&isolated)}>
                        { "Open isolated" }
                    </Button>
                </div>
                <pre class="docs-example__source" id={source_id} hidden={!*open}>
                    <code>{ highlight::rust(example.source.trim_end()) }</code>
                </pre>
            </div>
        </section>
    }
}

#[derive(Properties)]
struct StageProps {
    example: &'static Example,
}

impl PartialEq for StageProps {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.example, other.example)
    }
}

/// Mounts an example with time frozen, so renders are reproducible.
#[function_component]
fn Stage(props: &StageProps) -> Html {
    html! {
        <NowProvider now={Now::from_epoch_millis(EXAMPLE_NOW_MILLIS)} live=false>
            { (props.example.render)() }
        </NowProvider>
    }
}

fn copy_to_clipboard(text: &'static str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(text);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = text;
}

#[derive(Properties, PartialEq)]
pub struct IsolatedExampleProps {
    pub page: String,
    pub example: String,
}

/// One example and nothing else. The browser tests screenshot the element
/// marked `data-example`.
#[function_component]
pub fn IsolatedExample(props: &IsolatedExampleProps) -> Html {
    let found = Page::find(&props.page).and_then(|page| Some((page, page.example(&props.example)?)));
    let Some((page, example)) = found else {
        return html! { <NotFound /> };
    };
    html! {
        <main class="docs-isolated">
            <h1 class="yi-visually-hidden">{ format!("{}: {}", page.def.title, example.title) }</h1>
            <div class="docs-isolated__stage" data-example={format!("{}/{}", page.slug, example.slug())}>
                <Stage {example} />
            </div>
        </main>
    }
}

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <Layout>
            <header class="docs-page-header">
                <h1>{ "Page not found" }</h1>
                <p class="docs-lede">{ "Nothing lives at this address." }</p>
                <Cluster>
                    <Button href={href(&Route::Home)}>{ "Back to the start" }</Button>
                </Cluster>
            </header>
        </Layout>
    }
}

// ---- API tables ---------------------------------------------------------------

fn api_module(module: &'static ApiModule) -> Html {
    html! {
        <>
            { for module.components.iter().map(|component| {
                let (events, props): (Vec<&ApiProp>, Vec<&ApiProp>) = component.props.iter().partition(|p| p.is_event());
                html! {
                    <div class="docs-api">
                        <h3><code>{ component.name }</code></h3>
                        <p>{ inline(component.docs) }</p>
                        { prop_table("Props", &props) }
                        { prop_table("Events", &events) }
                    </div>
                }
            }) }
            if !module.enums.is_empty() {
                <div class="docs-api">
                    <h3>{ "Types" }</h3>
                    { table(&["Type", "Values", "Description"], module.enums.iter().map(|e| html! {
                        <tr>
                            <td><code>{ e.name }</code></td>
                            <td>
                                { for e.variants.iter().map(|(name, docs)| html! {
                                    <code class="docs-api__variant" title={*docs}>{ name }</code>
                                }) }
                            </td>
                            <td>{ inline(e.docs) }</td>
                        </tr>
                    }).collect()) }
                </div>
            }
            if !module.tokens.is_empty() {
                <div class="docs-api">
                    <h3>{ "CSS variables" }</h3>
                    { table(&["Variable", "Default", "Notes"], module.tokens.iter().map(|(name, default, note)| html! {
                        <tr>
                            <td><code>{ name }</code></td>
                            <td><code>{ default }</code></td>
                            <td>{ note }</td>
                        </tr>
                    }).collect()) }
                </div>
            }
        </>
    }
}

fn prop_table(title: &str, props: &[&ApiProp]) -> Html {
    if props.is_empty() {
        return html! {};
    }
    html! {
        <>
            <h4>{ title }</h4>
            { table(&["Name", "Type", "Default", "Description"], props.iter().map(|prop| html! {
                <tr>
                    <td><code>{ prop.name }</code></td>
                    <td><code>{ prop.ty }</code></td>
                    <td>
                        if prop.default.is_empty() {
                            <em>{ "required" }</em>
                        } else if prop.default != "default" {
                            <code>{ prop.default }</code>
                        }
                    </td>
                    <td>{ inline(prop.docs) }</td>
                </tr>
            }).collect()) }
        </>
    }
}

fn table(headers: &[&str], rows: Html) -> Html {
    html! {
        <div class="docs-table">
            <table>
                <thead><tr>{ for headers.iter().map(|h| html! { <th scope="col">{ h }</th> }) }</tr></thead>
                <tbody>{ rows }</tbody>
            </table>
        </div>
    }
}
