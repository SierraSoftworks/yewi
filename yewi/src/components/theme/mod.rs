//! Light and dark themes: follow the system, let the user override.
//!
//! The theme is the `data-theme` attribute on `<html>` (absent means "follow
//! the system") and nothing else, so it works before any wasm has loaded:
//!
//! 1. Put [`THEME_SCRIPT`] inline in the `<head>` (or render [`ThemeScript`]
//!    there) so a stored choice applies before first paint.
//! 2. Render a [`ThemeToggle`] somewhere, or call [`set_theme`] yourself.
//! 3. Read the theme from Rust with [`use_theme`] only where code needs it,
//!    for example to restyle a canvas or a map.

use yew::prelude::*;

use crate::{Attrs, Button, ButtonVariant, Icon, Size, icons};

/// The `localStorage` key the user's choice is stored under.
pub const THEME_STORAGE_KEY: &str = "yewi-theme";

/// Applies a stored theme choice. Inline it in the document `<head>`, before
/// any stylesheet, so the first paint is already in the right theme.
pub const THEME_SCRIPT: &str = r#"(function(){try{var t=localStorage.getItem("yewi-theme");if(t==="light"||t==="dark")document.documentElement.setAttribute("data-theme",t)}catch(e){}})()"#;

/// The user's theme preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Theme {
    /// Follow the operating system.
    #[default]
    System,
    Light,
    Dark,
}

impl Theme {
    /// The value of the `data-theme` attribute, `None` for `System`.
    pub const fn as_attr(self) -> Option<&'static str> {
        match self {
            Theme::System => None,
            Theme::Light => Some("light"),
            Theme::Dark => Some("dark"),
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Theme::System => "System",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }

    /// The next theme in the toggle's cycle: system, light, dark.
    pub const fn next(self) -> Self {
        match self {
            Theme::System => Theme::Light,
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::System,
        }
    }

    /// Parses a `data-theme` attribute value; anything unknown is `System`.
    pub fn from_attr(value: Option<&str>) -> Self {
        match value {
            Some("light") => Theme::Light,
            Some("dark") => Theme::Dark,
            _ => Theme::System,
        }
    }
}

/// The theme as [`use_theme`] reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ThemeState {
    /// What the user chose.
    pub preference: Theme,
    /// Whether the page is currently dark, after resolving `System`.
    pub dark: bool,
}

/// The current theme, read from the document. Off the web, the default.
pub fn current_theme() -> ThemeState {
    #[cfg(target_arch = "wasm32")]
    {
        dom::read()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        ThemeState::default()
    }
}

/// Sets, stores and announces the theme. Does nothing off the web.
pub fn set_theme(theme: Theme) {
    #[cfg(target_arch = "wasm32")]
    dom::apply(theme);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = theme;
}

/// The current theme, re-rendering when the user or the system changes it.
///
/// The first render always reports the default (`System`, light) so that it
/// matches a server render; the real value arrives once mounted. Prefer CSS
/// (`light-dark()`, `[data-theme]`) wherever CSS can do the job.
#[hook]
pub fn use_theme() -> ThemeState {
    let state = use_state_eq(ThemeState::default);
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            state.set(current_theme());
            #[cfg(target_arch = "wasm32")]
            let listener = dom::listen(move || state.set(current_theme()));
            move || {
                #[cfg(target_arch = "wasm32")]
                drop(listener);
            }
        });
    }
    *state
}

/// Renders [`THEME_SCRIPT`] as an inline `<script>`, for server-rendered heads.
#[function_component]
pub fn ThemeScript() -> Html {
    html! { <script>{ THEME_SCRIPT }</script> }
}

#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A button that cycles the theme: system, light, dark.
///
/// The icon is chosen in CSS from the document's `data-theme`, so it is right
/// on first paint and server renders never mismatch.
#[function_component]
pub fn ThemeToggle(props: &ThemeToggleProps) -> Html {
    let theme = use_theme().preference;
    let on_click = Callback::from(|_| set_theme(current_theme().preference.next()));
    let label = format!("Theme: {}. Switch to {}.", theme.label(), theme.next().label());

    let mut attrs = vec![("aria-label", AttrValue::from(label.clone())), ("title", AttrValue::from(label))];
    attrs.extend(props.attrs.iter().map(|(k, v)| (k, v.clone())));

    html! {
        <Button
            id={props.id.clone()}
            class={classes!("yi-theme-toggle", "yi-button--icon-only", props.class.clone())}
            variant={ButtonVariant::Subtle}
            size={props.size}
            attrs={Attrs::new(attrs)}
            {on_click}
        >
            <Icon icon={icons::MONITOR} class="yi-theme-toggle__icon yi-theme-toggle__icon--system" />
            <Icon icon={icons::SUN} class="yi-theme-toggle__icon yi-theme-toggle__icon--light" />
            <Icon icon={icons::MOON} class="yi-theme-toggle__icon yi-theme-toggle__icon--dark" />
        </Button>
    }
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{Element, EventTarget, MediaQueryList, Window};

    use super::{THEME_STORAGE_KEY, Theme, ThemeState};

    const EVENT: &str = "yewi:theme";
    const TRANSITION_ATTR: &str = "data-yi-theme-transition";
    const DARK_QUERY: &str = "(prefers-color-scheme: dark)";

    fn root() -> Option<(Window, Element)> {
        let window = web_sys::window()?;
        let root = window.document()?.document_element()?;
        Some((window, root))
    }

    fn dark_query(window: &Window) -> Option<MediaQueryList> {
        window.match_media(DARK_QUERY).ok().flatten()
    }

    pub fn read() -> ThemeState {
        let Some((window, root)) = root() else { return ThemeState::default() };
        let preference = Theme::from_attr(root.get_attribute("data-theme").as_deref());
        let dark = match preference {
            Theme::System => dark_query(&window).is_some_and(|query| query.matches()),
            Theme::Light => false,
            Theme::Dark => true,
        };
        ThemeState { preference, dark }
    }

    pub fn apply(theme: Theme) {
        let Some((window, root)) = root() else { return };

        // Ease colours for the length of the change, then get out of the way.
        let _ = root.set_attribute(TRANSITION_ATTR, "");
        let cleanup = Closure::once_into_js({
            let root = root.clone();
            move || {
                let _ = root.remove_attribute(TRANSITION_ATTR);
            }
        });
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(cleanup.unchecked_ref(), 250);

        let storage = window.local_storage().ok().flatten();
        match theme.as_attr() {
            Some(value) => {
                let _ = root.set_attribute("data-theme", value);
                if let Some(storage) = storage {
                    let _ = storage.set_item(THEME_STORAGE_KEY, value);
                }
            }
            None => {
                let _ = root.remove_attribute("data-theme");
                if let Some(storage) = storage {
                    let _ = storage.remove_item(THEME_STORAGE_KEY);
                }
            }
        }

        if let Ok(event) = web_sys::Event::new(EVENT) {
            let _ = window.dispatch_event(&event);
        }
    }

    /// Calls `on_change` when the theme is set or the system setting changes,
    /// until dropped.
    pub struct Listener {
        closure: Closure<dyn FnMut()>,
        targets: Vec<(EventTarget, &'static str)>,
    }

    pub fn listen(on_change: impl FnMut() + 'static) -> Listener {
        let closure = Closure::<dyn FnMut()>::new(on_change);
        let mut targets: Vec<(EventTarget, &'static str)> = Vec::new();
        if let Some(window) = web_sys::window() {
            if let Some(query) = dark_query(&window) {
                targets.push((query.into(), "change"));
            }
            targets.push((window.into(), EVENT));
        }
        for (target, event) in &targets {
            let _ = target.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref());
        }
        Listener { closure, targets }
    }

    impl Drop for Listener {
        fn drop(&mut self) {
            for (target, event) in &self.targets {
                let _ = target.remove_event_listener_with_callback(event, self.closure.as_ref().unchecked_ref());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_cycles_through_every_theme() {
        let mut theme = Theme::System;
        let mut seen = vec![theme];
        for _ in 0..2 {
            theme = theme.next();
            seen.push(theme);
        }
        assert_eq!(seen, [Theme::System, Theme::Light, Theme::Dark]);
        assert_eq!(theme.next(), Theme::System);
    }

    #[test]
    fn toggle_has_an_accessible_name_when_server_rendered() {
        let html = yewi_test::render(|| html! { <ThemeToggle /> });
        assert!(html.contains("aria-label=\"Theme: System."));
    }
}
