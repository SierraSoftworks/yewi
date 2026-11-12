//! Tabs: switch between views of one subject ([`Tabs`]), or between pages
//! that share a header ([`TabNav`]).

use std::rc::Rc;

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, NavTarget, Size, nav_target::use_nav_link, use_id};

/// How a row of tabs is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TabsVariant {
    /// Tabs sit on a rule and the selected one is underlined. For the main
    /// views of a page.
    #[default]
    Line,
    /// The selected tab is a soft filled pill, with no rule. For a card head
    /// or toolbar, where a rule would compete with the surrounding edges.
    Pill,
}

impl TabsVariant {
    pub const ALL: [TabsVariant; 2] = [TabsVariant::Line, TabsVariant::Pill];

    const fn as_str(self) -> &'static str {
        match self {
            TabsVariant::Line => "line",
            TabsVariant::Pill => "pill",
        }
    }
}

fn root_class(variant: TabsVariant, size: Size, class: &Classes) -> Classes {
    classes!(
        "yi-tabs",
        format!("yi-tabs--{}", variant.as_str()),
        format!("yi-tabs--{}", size.as_str()),
        class.clone()
    )
}

/// What a tab shows: icon, label and count.
fn tab_content(icon: Option<IconData>, label: Html, count: Option<&AttrValue>) -> Html {
    html! {
        <>
            if let Some(icon) = icon {
                <Icon {icon} />
            }
            <span class="yi-tabs__label">{ label }</span>
            if let Some(count) = count {
                <span class="yi-tabs__count">{ count.clone() }</span>
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabProps {
    /// Identifies the tab in `selected`, `default_selected` and `on_change`.
    pub value: AttrValue,
    /// The tab's label.
    pub label: AttrValue,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// A count shown after the label, such as the number of items inside.
    #[prop_or_default]
    pub count: Option<AttrValue>,
    /// Cannot be selected, and is skipped by the arrow keys.
    #[prop_or_default]
    pub disabled: bool,
    /// The panel shown while this tab is selected.
    #[prop_or_default]
    pub children: Html,
}

/// One tab of a [`Tabs`] and the panel it shows.
#[function_component]
pub fn Tab(props: &TabProps) -> Html {
    props.children.clone()
}

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    /// The [`Tab`]s, in order.
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab>,
    /// Accessible name of the tab list, such as "Account sections".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// The selected tab's `value`. Set it to control the selection from
    /// outside; leave it unset and the component keeps its own state.
    #[prop_or_default]
    pub selected: Option<AttrValue>,
    /// The tab selected at first when uncontrolled. Defaults to the first
    /// tab that is not disabled.
    #[prop_or_default]
    pub default_selected: Option<AttrValue>,
    #[prop_or_default]
    pub variant: TabsVariant,
    #[prop_or_default]
    pub size: Size,
    /// Arrow keys only move focus, and Enter or Space selects. Use it when
    /// showing a panel is slow, for example when it loads data.
    #[prop_or_default]
    pub manual: bool,
    /// Keeps every panel in the document, hiding the unselected ones, so
    /// their state survives a switch. By default only the selected panel is
    /// rendered.
    #[prop_or_default]
    pub keep_mounted: bool,
    /// Fired with the `value` of the tab the user selects.
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A tab list and the panel of its selected [`Tab`].
///
/// Follows the ARIA tabs pattern: one tab stop, arrow keys, Home and End move
/// between tabs, and by default the selection follows focus.
#[function_component]
pub fn Tabs(props: &TabsProps) -> Html {
    let base = use_id();
    let own = use_state(|| props.default_selected.clone());

    let tabs: Vec<Rc<TabProps>> = props.children.iter().map(|tab| tab.props).collect();
    let wanted = props.selected.as_ref().or(own.as_ref());
    let selected = wanted
        .and_then(|value| tabs.iter().position(|tab| tab.value == *value))
        .or_else(|| tabs.iter().position(|tab| !tab.disabled));

    let select = {
        let (own, on_change) = (own.clone(), props.on_change.clone());
        Callback::from(move |value: AttrValue| {
            own.set(Some(value.clone()));
            on_change.emit(value);
        })
    };

    let tab_id = |index: usize| format!("{base}-tab-{index}");
    let panel_id = |index: usize| format!("{base}-panel-{index}");
    // (element id, value) of each tab the arrow keys can reach.
    let reachable: Rc<[(String, AttrValue)]> =
        tabs.iter().enumerate().filter(|(_, tab)| !tab.disabled).map(|(i, tab)| (tab_id(i), tab.value.clone())).collect();

    let buttons = tabs.iter().enumerate().map(|(index, tab)| {
        let is_selected = selected == Some(index);
        let onclick = {
            let (select, value) = (select.clone(), tab.value.clone());
            Callback::from(move |_: MouseEvent| select.emit(value.clone()))
        };
        let onkeydown = {
            let (select, reachable, manual, id) = (select.clone(), reachable.clone(), props.manual, tab_id(index));
            Callback::from(move |event: KeyboardEvent| {
                let Some(at) = reachable.iter().position(|(other, _)| *other == id) else { return };
                let last = reachable.len() - 1;
                let to = match event.key().as_str() {
                    "ArrowRight" => (at + 1) % reachable.len(),
                    "ArrowLeft" => at.checked_sub(1).unwrap_or(last),
                    "Home" => 0,
                    "End" => last,
                    _ => return,
                };
                event.prevent_default();
                let (id, value) = &reachable[to];
                focus(id);
                if !manual {
                    select.emit(value.clone());
                }
            })
        };
        let rendered = is_selected || props.keep_mounted;

        html! {
            <button
                key={index}
                type="button"
                role="tab"
                id={tab_id(index)}
                class="yi-tabs__tab"
                aria-selected={if is_selected { "true" } else { "false" }}
                aria-controls={rendered.then(|| panel_id(index))}
                tabindex={if is_selected { "0" } else { "-1" }}
                disabled={tab.disabled}
                {onclick}
                {onkeydown}
            >
                { tab_content(tab.icon, tab.label.clone().into(), tab.count.as_ref()) }
            </button>
        }
    });

    let panels = tabs.iter().enumerate().filter(|(index, _)| props.keep_mounted || selected == Some(*index)).map(
        |(index, tab)| {
            html! {
                <div
                    key={index}
                    role="tabpanel"
                    id={panel_id(index)}
                    class="yi-tabs__panel"
                    aria-labelledby={tab_id(index)}
                    tabindex="0"
                    hidden={selected != Some(index)}
                >
                    { tab.children.clone() }
                </div>
            }
        },
    );

    props.attrs.apply(html! {
        <div id={props.id.clone()} class={root_class(props.variant, props.size, &props.class)}>
            <div class="yi-tabs__list" role="tablist" aria-label={props.label.clone()}>
                { for buttons }
            </div>
            { for panels }
        </div>
    })
}

/// Moves keyboard focus to the element with `id`. Does nothing off the web.
fn focus(id: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;

        let element = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(id));
        if let Some(element) = element.and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok()) {
            let _ = element.focus();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = id;
}

#[derive(Properties, PartialEq)]
pub struct TabNavProps {
    /// The [`TabLink`]s, in order.
    #[prop_or_default]
    pub children: Html,
    /// Accessible name of the navigation landmark, such as "Account sections".
    pub label: AttrValue,
    #[prop_or_default]
    pub variant: TabsVariant,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Tabs that are links: each one is its own page or route.
///
/// Renders a navigation landmark of real links, so tabs can be opened in a
/// new browser tab and work before any script has loaded.
#[function_component]
pub fn TabNav(props: &TabNavProps) -> Html {
    props.attrs.apply(html! {
        <nav
            id={props.id.clone()}
            class={root_class(props.variant, props.size, &props.class)}
            aria-label={props.label.clone()}
        >
            <ul class="yi-tabs__list" role="list">{ props.children.clone() }</ul>
        </nav>
    })
}

#[derive(Properties, PartialEq)]
pub struct TabLinkProps {
    /// Where the tab leads: a URL or, with the `router` feature,
    /// `NavTarget::route(..)`.
    pub href: NavTarget,
    /// The label.
    #[prop_or_default]
    pub children: Html,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// A count shown after the label.
    #[prop_or_default]
    pub count: Option<AttrValue>,
    /// Whether this is the page being shown. A route target works this out
    /// from the router's location; set it for plain links, or to override.
    #[prop_or_default]
    pub current: Option<bool>,
    /// Fired on click, before any navigation.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One link of a [`TabNav`].
#[function_component]
pub fn TabLink(props: &TabLinkProps) -> Html {
    let link = use_nav_link(&props.href, props.on_click.clone());
    let current = props.current.unwrap_or(link.current);

    let anchor = props.attrs.apply(html! {
        <a
            id={props.id.clone()}
            class={classes!("yi-tabs__tab", props.class.clone())}
            href={link.href}
            aria-current={current.then_some("page")}
            onclick={link.onclick}
        >
            { tab_content(props.icon, props.children.clone(), props.count.as_ref()) }
        </a>
    });

    html! { <li class="yi-tabs__item">{ anchor }</li> }
}

#[cfg(test)]
mod tests;
