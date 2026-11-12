//! Vocabulary shared by every component: sizes, tones, spacing and
//! pass-through attributes.

use std::rc::Rc;

use yew::{AttrValue, Html, virtual_dom::VNode};

/// Control size. `Medium` is the 32px default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

impl Size {
    /// The BEM modifier suffix: `sm`, `md` or `lg`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Size::Small => "sm",
            Size::Medium => "md",
            Size::Large => "lg",
        }
    }
}

/// Semantic status tone. Tones carry meaning and are never decoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Tone {
    #[default]
    Neutral,
    Ok,
    Warn,
    Danger,
    Info,
    /// Work in flight: rolling, running, syncing.
    Active,
}

impl Tone {
    pub const ALL: [Tone; 6] = [Tone::Neutral, Tone::Ok, Tone::Warn, Tone::Danger, Tone::Info, Tone::Active];

    /// The token and BEM modifier suffix, e.g. `ok` for `--yw-ok`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Tone::Neutral => "neutral",
            Tone::Ok => "ok",
            Tone::Warn => "warn",
            Tone::Danger => "danger",
            Tone::Info => "info",
            Tone::Active => "active",
        }
    }
}

/// A step on the spacing scale, for `gap` and similar props.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Space {
    None,
    Xxs,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
}

impl Space {
    /// The CSS value: a `var(--yw-space-*)` reference, or `0`.
    pub const fn css(self) -> &'static str {
        match self {
            Space::None => "0",
            Space::Xxs => "var(--yw-space-2xs)",
            Space::Xs => "var(--yw-space-xs)",
            Space::Sm => "var(--yw-space-sm)",
            Space::Md => "var(--yw-space-md)",
            Space::Lg => "var(--yw-space-lg)",
            Space::Xl => "var(--yw-space-xl)",
            Space::Xxl => "var(--yw-space-2xl)",
            Space::Xxxl => "var(--yw-space-3xl)",
        }
    }
}

/// Extra attributes for a component's root element: `aria-*`, `data-*`,
/// `title` and the like. Build one with [`attrs!`](crate::attrs).
///
/// Every yewi component takes `class`, `id` and `attrs`, so anything it has
/// no dedicated prop for can still reach the DOM.
#[derive(Debug, Clone, Default)]
pub struct Attrs(Option<Rc<[(&'static str, AttrValue)]>>);

impl Attrs {
    pub fn new(attrs: impl IntoIterator<Item = (&'static str, AttrValue)>) -> Self {
        Self(Some(attrs.into_iter().collect()))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &AttrValue)> {
        self.0.iter().flat_map(|attrs| attrs.iter().map(|(k, v)| (*k, v)))
    }

    /// Adds these attributes to `node`'s root element. Attributes the
    /// component already set are overwritten.
    pub fn apply(&self, mut node: Html) -> Html {
        if let VNode::VTag(tag) = &mut node {
            let tag = Rc::make_mut(tag);
            for (key, value) in self.iter() {
                tag.add_attribute(key, value.clone());
            }
        }
        node
    }
}

impl PartialEq for Attrs {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => Rc::ptr_eq(a, b) || a == b,
            _ => false,
        }
    }
}

/// Builds [`Attrs`]: `attrs! { "aria-label" => "Close", "data-row" => id }`.
#[macro_export]
macro_rules! attrs {
    ($($key:literal => $value:expr),* $(,)?) => {
        $crate::Attrs::new([$(($key, ::yew::AttrValue::from($value))),*])
    };
}
