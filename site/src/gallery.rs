//! The review gallery: every example of one component, live, with the theme,
//! accent and stage width under the reviewer's control.
//!
//! `e2e/viewer.mjs` packs the built site into a single HTML file that starts
//! here (a root element marked `data-gallery`) instead of at the documentation.
//! The recordings under each example are added by that file's replay player.

use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewi::prelude::*;

use crate::{
    Route,
    docs::{Category, Page},
    shell::{Stage, inline},
};

/// Accents a reviewer can preview: `(value, label)`. The colours live in
/// `site.scss` under `[data-gallery-accent]`.
const ACCENTS: [(&str, &str); 7] = [
    ("default", "Default blue"),
    ("violet", "Violet"),
    ("teal", "Teal"),
    ("green", "Green"),
    ("amber", "Amber"),
    ("rose", "Rose"),
    ("grey", "Pure grey"),
];

/// Stage widths: `(value, label)`.
const WIDTHS: [(&str, &str); 3] = [("full", "Full"), ("tablet", "Tablet"), ("phone", "Phone")];

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    /// The page to show; `None` shows the first one.
    #[prop_or_default]
    pub slug: Option<String>,
}

#[function_component]
pub fn Gallery(props: &GalleryProps) -> Html {
    let accent = use_state(|| AttrValue::from("default"));
    let width = use_state(|| AttrValue::from("full"));
    let theme = use_theme().preference;
    let navigator = use_navigator();

    {
        // The accent is an attribute on the document, like the theme, so it reaches overlays too.
        let accent = (*accent).clone();
        use_effect_with(accent, |accent| {
            #[cfg(target_arch = "wasm32")]
            if let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
                let _ = root.set_attribute("data-gallery-accent", accent);
            }
            let _ = accent;
        });
    }

    let page = props
        .slug
        .as_deref()
        .and_then(Page::find)
        .or_else(|| Category::ALL.into_iter().find_map(|c| Page::in_category(c).next()));

    let on_accent = {
        let accent = accent.clone();
        Callback::from(move |value| accent.set(value))
    };
    let on_width = {
        let width = width.clone();
        Callback::from(move |value| width.set(value))
    };
    let on_theme = Callback::from(|value: AttrValue| {
        set_theme(match value.as_str() {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::System,
        });
    });
    let accents = use_memo((), |_| SelectOptions::from(ACCENTS.map(|(value, label)| SelectOption::new(value, label))));

    html! {
        <div class="gallery">
            <header class="gallery-header">
                <span class="gallery-brand">{ "yewi" }<small>{ "review gallery" }</small></span>
                <span class="gallery-header__build" data-gallery-build=""></span>
                <div class="gallery-header__controls">
                    <SegmentedControl label="Stage width" size={Size::Small} value={(*width).clone()} on_change={on_width}>
                        { for WIDTHS.iter().map(|(value, label)| html! { <Segment value={*value}>{ *label }</Segment> }) }
                    </SegmentedControl>
                    <div class="gallery-header__accent">
                        <Select
                            options={(*accents).clone()}
                            size={Size::Small}
                            label="Accent"
                            fit=true
                            value={(*accent).clone()}
                            on_change={on_accent}
                        />
                    </div>
                    <SegmentedControl
                        label="Colour mode"
                        size={Size::Small}
                        value={theme.as_attr().unwrap_or("system")}
                        on_change={on_theme}
                    >
                        <Segment value="system">{ "System" }</Segment>
                        <Segment value="light">{ "Light" }</Segment>
                        <Segment value="dark">{ "Dark" }</Segment>
                    </SegmentedControl>
                </div>
            </header>
            <div class="gallery-body">
                <nav class="docs-sidebar gallery-sidebar" aria-label="Components">
                    { for Category::ALL.into_iter().map(|category| {
                        let mut pages = Page::in_category(category).peekable();
                        if pages.peek().is_none() {
                            return html! {};
                        }
                        html! {
                            <>
                                <span class="docs-sidebar__group">{ category.label() }</span>
                                { for pages.map(|entry| {
                                    let active = page.is_some_and(|page| std::ptr::eq(page, entry));
                                    let go = {
                                        let navigator = navigator.clone();
                                        let slug = entry.slug;
                                        Callback::from(move |_: MouseEvent| {
                                            if let Some(navigator) = &navigator {
                                                navigator.push(&Route::Gallery { slug: slug.to_owned() });
                                            }
                                        })
                                    };
                                    // A button, not a link: file viewers open every link in a new window.
                                    html! {
                                        <button
                                            type="button"
                                            class={classes!("docs-sidebar__link", active.then_some("docs-sidebar__link--active"))}
                                            aria-current={active.then_some("page")}
                                            data-gallery-link={entry.slug}
                                            onclick={go}
                                        >
                                            { entry.def.title }
                                        </button>
                                    }
                                }) }
                            </>
                        }
                    }) }
                </nav>
                <main class={classes!("gallery-main", "docs-main", format!("gallery-main--{}", *width))} id="content">
                    if let Some(page) = page {
                        <header class="docs-page-header" key={page.slug}>
                            <span class="docs-eyebrow">{ page.def.category.label() }</span>
                            <h1>{ page.def.title }</h1>
                            <p class="docs-lede">{ inline(page.def.summary) }</p>
                        </header>
                        { for page.def.examples.iter().map(|example| html! {
                            <GalleryExample key={format!("{}/{}", page.slug, example.name)} {page} {example} />
                        }) }
                    }
                </main>
            </div>
        </div>
    }
}

#[derive(Properties)]
struct GalleryExampleProps {
    page: &'static Page,
    example: &'static crate::docs::Example,
}

impl PartialEq for GalleryExampleProps {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.page, other.page) && std::ptr::eq(self.example, other.example)
    }
}

#[function_component]
fn GalleryExample(props: &GalleryExampleProps) -> Html {
    let GalleryExampleProps { page, example } = *props;
    let id = format!("{}/{}", page.slug, example.slug());
    // Bumping the key mounts the example afresh: its initial state, as a recording expects it.
    let generation = use_state(|| 0_u32);
    let reset = {
        let generation = generation.clone();
        Callback::from(move |_| generation.set(*generation + 1))
    };

    html! {
        <section class="gallery-example" id={example.slug()}>
            <header class="gallery-example__header">
                <h2>{ example.title }</h2>
                <code>{ id.clone() }</code>
                <Button
                    size={Size::Small}
                    variant={ButtonVariant::Subtle}
                    icon={icons::ROTATE_CCW}
                    attrs={attrs! { "data-gallery-reset" => "" }}
                    on_click={reset}
                >
                    { "Reset" }
                </Button>
            </header>
            <p>{ inline(example.description) }</p>
            // Filled by the replay player in the packed gallery file. Above the stage, so an open menu does not cover it.
            <div class="gallery-example__replays" data-gallery-replays={id.clone()} />
            <div class="gallery-example__frame">
                <div class="gallery-example__stage" data-example={id} key={*generation}>
                    <Stage {example} />
                </div>
            </div>
        </section>
    }
}
