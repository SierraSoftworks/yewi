//! The vocabulary pages are written in: [`page!`](crate::page), [`PageDef`]
//! and [`Example`], plus the generated API types.

use yew::Html;

/// Sidebar groups, in display order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Foundations,
    Layout,
    Basic,
    Form,
    Navigation,
    Overlay,
    Data,
    Visualisation,
    Utilities,
}

impl Category {
    pub const ALL: [Category; 9] = [
        Category::Foundations,
        Category::Layout,
        Category::Basic,
        Category::Form,
        Category::Navigation,
        Category::Overlay,
        Category::Data,
        Category::Visualisation,
        Category::Utilities,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Category::Foundations => "Foundations",
            Category::Layout => "Layout",
            Category::Basic => "Basic",
            Category::Form => "Form",
            Category::Navigation => "Navigation",
            Category::Overlay => "Overlay",
            Category::Data => "Data",
            Category::Visualisation => "Visualisation",
            Category::Utilities => "Utilities",
        }
    }
}

/// A page as its `mod.rs` declares it. Build one with [`page!`](crate::page).
pub struct PageDef {
    pub title: &'static str,
    pub category: Category,
    /// One sentence: what it is for.
    pub summary: &'static str,
    /// The Cargo feature that enables it, for opt-in components.
    pub feature: Option<&'static str>,
    /// Library component directories whose API this page documents. `None`
    /// means the directory with the page's own name.
    pub api: Option<&'static [&'static str]>,
    pub examples: &'static [Example],
}

/// A registered page: a [`PageDef`] plus where it lives.
pub struct Page {
    /// URL segment, e.g. `visually-hidden`.
    pub slug: &'static str,
    /// Directory name, e.g. `visually_hidden`.
    pub module: &'static str,
    pub def: &'static PageDef,
}

impl Page {
    pub fn find(slug: &str) -> Option<&'static Page> {
        crate::pages::PAGES.iter().find(|page| page.slug == slug)
    }

    pub fn in_category(category: Category) -> impl Iterator<Item = &'static Page> {
        let mut pages: Vec<_> = crate::pages::PAGES.iter().filter(|p| p.def.category == category).collect();
        pages.sort_by_key(|page| page.def.title);
        pages.into_iter()
    }

    pub fn example(&self, slug: &str) -> Option<&'static Example> {
        self.def.examples.iter().find(|example| example.slug() == slug)
    }

    /// The API modules this page documents.
    pub fn api(&self) -> impl Iterator<Item = &'static ApiModule> {
        let own = [self.module];
        let names: Vec<&str> = self.def.api.map_or_else(|| own.to_vec(), <[_]>::to_vec);
        crate::api::API.iter().filter(move |module| names.contains(&module.module))
    }
}

/// One scenario: a live render plus the source that produced it.
pub struct Example {
    /// The example's file name without `.rs`.
    pub name: &'static str,
    pub title: &'static str,
    /// A sentence or two of guidance. Backticks mark inline code.
    pub description: &'static str,
    /// The example file, verbatim.
    pub source: &'static str,
    pub render: fn() -> Html,
}

impl Example {
    /// URL segment, e.g. `with-icon`.
    pub fn slug(&self) -> String {
        self.name.replace('_', "-")
    }
}

/// Declares a documentation page. Use it once, in the page's `mod.rs`:
///
/// ```ignore
/// crate::page! {
///     title: "Button",
///     category: Basic,
///     summary: "Triggers an action.",
///     examples: [
///         variants => "Variants": "One primary action per view.",
///         sizes => "Sizes": "Match the density of the surrounding controls.",
///     ],
/// }
/// ```
///
/// Each example names a file in `examples/` that defines
/// `#[function_component] pub fn Example() -> Html`. The file is both mounted
/// and shown as the example's source. Optional fields, after `summary`:
/// `feature: "charts",` and `api: ["button", "icon"],`.
#[macro_export]
macro_rules! page {
    (
        title: $title:literal,
        category: $category:ident,
        summary: $summary:literal,
        $(feature: $feature:literal,)?
        $(api: [$($api:literal),* $(,)?],)?
        examples: [$($example:ident => $example_title:literal : $description:literal),* $(,)?] $(,)?
    ) => {
        mod examples {
            $(pub mod $example;)*
        }

        pub const PAGE: $crate::docs::PageDef = $crate::docs::PageDef {
            title: $title,
            category: $crate::docs::Category::$category,
            summary: $summary,
            feature: $crate::page!(@optional $($feature)?),
            api: $crate::page!(@optional $(&[$($api),*])?),
            examples: &[$(
                $crate::docs::Example {
                    name: stringify!($example),
                    title: $example_title,
                    description: $description,
                    source: include_str!(concat!("examples/", stringify!($example), ".rs")),
                    render: || ::yew::html! { <examples::$example::Example /> },
                },
            )*],
        };
    };
    (@optional) => { None };
    (@optional $value:expr) => { Some($value) };
}

// ---- Generated API reference ------------------------------------------------

/// The API of one library component directory.
pub struct ApiModule {
    pub module: &'static str,
    pub components: &'static [ApiComponent],
    pub enums: &'static [ApiEnum],
    /// `(token, default, note)` from the `// Component tokens` SCSS block.
    pub tokens: &'static [(&'static str, &'static str, &'static str)],
}

pub struct ApiComponent {
    pub name: &'static str,
    pub docs: &'static str,
    pub props: &'static [ApiProp],
}

pub struct ApiProp {
    pub name: &'static str,
    pub ty: &'static str,
    /// Empty for a required prop, `default` for `Default::default()`.
    pub default: &'static str,
    pub docs: &'static str,
}

impl ApiProp {
    pub fn is_event(&self) -> bool {
        self.ty.starts_with("Callback<")
    }
}

pub struct ApiEnum {
    pub name: &'static str,
    pub docs: &'static str,
    pub variants: &'static [(&'static str, &'static str)],
}
