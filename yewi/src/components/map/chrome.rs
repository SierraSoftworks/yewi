//! What is laid over a map: a toolbar of tools, floating panels and notes.

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, Tone, icons};

#[derive(Properties, PartialEq)]
pub struct MapToolbarProps {
    /// [`MapToolGroup`]s.
    #[prop_or_default]
    pub children: Html,
    /// The accessible name of the toolbar.
    #[prop_or(AttrValue::Static("Map tools"))]
    pub label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A strip of tool groups for the map's `toolbar` slot. Arrow keys move
/// between its buttons.
#[function_component]
pub fn MapToolbar(props: &MapToolbarProps) -> Html {
    let onkeydown = Callback::from(|event: KeyboardEvent| {
        #[cfg(target_arch = "wasm32")]
        super::engine::toolbar_key(event.as_ref());
        #[cfg(not(target_arch = "wasm32"))]
        let _ = event;
    });

    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!("yi-map-toolbar", props.class.clone())}
            role="toolbar"
            aria-label={props.label.clone()}
            {onkeydown}
        >
            { props.children.clone() }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct MapToolGroupProps {
    /// [`MapTool`]s, or a status such as a pill and a count.
    #[prop_or_default]
    pub children: Html,
    /// The accessible name of the group, when its tools belong together.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A group of tools in a [`MapToolbar`], ruled off from its neighbours.
#[function_component]
pub fn MapToolGroup(props: &MapToolGroupProps) -> Html {
    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!("yi-map-toolbar__group", props.class.clone())}
            role={props.label.is_some().then_some("group")}
            aria-label={props.label.clone()}
        >
            { props.children.clone() }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct MapToolProps {
    pub icon: IconData,
    /// What the tool does: its accessible name and its tooltip.
    pub label: AttrValue,
    /// For a tool that stays chosen: whether it is the one in hand. Leave
    /// unset for a tool that acts once, such as "fit everything".
    #[prop_or_default]
    pub pressed: Option<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// An icon button in a [`MapToolGroup`].
#[function_component]
pub fn MapTool(props: &MapToolProps) -> Html {
    props.attrs.apply(html! {
        <button
            id={props.id.clone()}
            type="button"
            class={classes!("yi-map-tool", props.class.clone())}
            aria-label={props.label.clone()}
            aria-pressed={props.pressed.map(|pressed| if pressed { "true" } else { "false" })}
            title={props.label.clone()}
            disabled={props.disabled}
            onclick={props.on_click.clone()}
        >
            <Icon icon={props.icon} />
        </button>
    })
}

#[derive(Properties, PartialEq)]
pub struct MapPanelProps {
    /// The body. It scrolls when it is taller than the map allows.
    #[prop_or_default]
    pub children: Html,
    pub title: AttrValue,
    /// A short note beside the title, such as a count.
    #[prop_or_default]
    pub meta: Option<AttrValue>,
    /// Lets the panel fold down to its title, to get it out of the map's way.
    #[prop_or_default]
    pub collapsible: bool,
    /// Whether a collapsible panel starts open.
    #[prop_or(true)]
    pub open: bool,
    /// Adds a close button and reports its use. Not shown on a collapsible
    /// panel.
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A card floating over the map, for the `sidebar` and `panel` slots.
#[function_component]
pub fn MapPanel(props: &MapPanelProps) -> Html {
    let title = html! {
        <>
            <span class="yi-map-panel__title">{ props.title.clone() }</span>
            if let Some(meta) = &props.meta {
                <span class="yi-map-panel__meta">{ meta.clone() }</span>
            }
        </>
    };
    let body = html! { <div class="yi-map-panel__body">{ props.children.clone() }</div> };
    let class = classes!("yi-map-panel", props.class.clone());

    props.attrs.apply(if props.collapsible {
        // The browser keeps the open state, so folding works before hydration.
        html! {
            <details id={props.id.clone()} {class} open={props.open}>
                <summary class="yi-map-panel__head">
                    { title }
                    <Icon icon={icons::CHEVRON_DOWN} class="yi-map-panel__chevron" />
                </summary>
                { body }
            </details>
        }
    } else {
        let close = props.on_close.clone().map(|on_close| {
            let onclick = Callback::from(move |_: MouseEvent| on_close.emit(()));
            html! {
                <button type="button" class="yi-map-tool yi-map-panel__close" aria-label="Close" {onclick}>
                    <Icon icon={icons::X} />
                </button>
            }
        });
        html! {
            <section id={props.id.clone()} {class} aria-label={props.title.clone()}>
                <header class="yi-map-panel__head">
                    { title }
                    { for close }
                </header>
                { body }
            </section>
        }
    })
}

#[derive(Properties, PartialEq)]
pub struct MapNoteProps {
    #[prop_or_default]
    pub children: Html,
    /// `Neutral` for a hint; a status tone when something needs attention.
    #[prop_or_default]
    pub tone: Tone,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A line of guidance or status for the map's `overlay` slot, announced
/// when it appears.
#[function_component]
pub fn MapNote(props: &MapNoteProps) -> Html {
    props.attrs.apply(html! {
        <p
            id={props.id.clone()}
            class={classes!("yi-map-note", format!("yi-map-note--{}", props.tone.as_str()), props.class.clone())}
            role="status"
        >
            { props.children.clone() }
        </p>
    })
}

#[derive(Properties, PartialEq)]
pub struct MapListProps {
    /// [`MapListItem`]s.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The things on the map as a list, for a [`MapPanel`] in the sidebar.
#[function_component]
pub fn MapList(props: &MapListProps) -> Html {
    props.attrs.apply(html! {
        <ul id={props.id.clone()} class={classes!("yi-map-list", props.class.clone())}>
            { props.children.clone() }
        </ul>
    })
}

#[derive(Properties, PartialEq)]
pub struct MapListItemProps {
    pub label: AttrValue,
    /// A quieter second line: a type, a place, a time.
    #[prop_or_default]
    pub detail: Option<AttrValue>,
    /// Draws a dot in this tone before the label, matching its marker.
    #[prop_or_default]
    pub tone: Option<Tone>,
    /// Whether this is the feature the map has selected.
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One row of a [`MapList`]: a button that selects a feature.
#[function_component]
pub fn MapListItem(props: &MapListItemProps) -> Html {
    let dot = props.tone.map(|tone| {
        html! { <span class={classes!("yi-map-list__dot", format!("yi-map-list__dot--{}", tone.as_str()))} /> }
    });

    props.attrs.apply(html! {
        <li id={props.id.clone()} class={classes!("yi-map-list__row", props.class.clone())}>
            <button
                type="button"
                class="yi-map-list__item"
                aria-current={props.selected.then_some("true")}
                onclick={props.on_click.clone()}
            >
                { for dot }
                <span class="yi-map-list__label">{ props.label.clone() }</span>
                if let Some(detail) = &props.detail {
                    <span class="yi-map-list__detail">{ detail.clone() }</span>
                }
            </button>
        </li>
    })
}
