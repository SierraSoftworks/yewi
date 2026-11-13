//! [`Panel`]: a section inside a [`Card`](crate::Card), stacked flush with
//! its siblings and told apart by an alternating tonal fill.

use yew::prelude::*;

use crate::{Attrs, Icon, icons};

/// The fill behind a panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum PanelFill {
    /// Alternates with sibling panels: the first is `Strong`, the next
    /// `Soft`, and so on.
    #[default]
    Auto,
    /// No fill: the surface underneath shows through.
    Plain,
    /// The lighter tonal fill, `--yi-tone-2`.
    Soft,
    /// The stronger tonal fill, `--yi-tone-1`.
    Strong,
}

impl PanelFill {
    pub const ALL: [PanelFill; 4] = [PanelFill::Auto, PanelFill::Plain, PanelFill::Soft, PanelFill::Strong];

    const fn as_str(self) -> &'static str {
        match self {
            PanelFill::Auto => "auto",
            PanelFill::Plain => "plain",
            PanelFill::Soft => "soft",
            PanelFill::Strong => "strong",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Html,
    /// The heading. Required for a `collapsible` panel, where it is the
    /// control that opens and closes it.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// A supporting line beside the heading.
    #[prop_or_default]
    pub subtitle: Option<AttrValue>,
    /// Controls at the end of the heading row. Not shown on a `collapsible`
    /// panel, whose whole heading row is the toggle.
    #[prop_or_default]
    pub actions: Html,
    #[prop_or_default]
    pub fill: PanelFill,
    /// Lets the reader fold the body away. Uses a native `<details>`, so it
    /// works before, and without, the wasm loading.
    #[prop_or_default]
    pub collapsible: bool,
    /// Starts a `collapsible` panel closed.
    #[prop_or_default]
    pub collapsed: bool,
    /// Fired with `true` when a `collapsible` panel opens and `false` when
    /// it closes.
    #[prop_or_default]
    pub on_toggle: Callback<bool>,
    /// Removes the body padding, for content that runs edge to edge.
    #[prop_or_default]
    pub flush: bool,
    /// The heading level of the title, 1 to 6.
    #[prop_or(3)]
    pub level: u8,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A titled section that stacks flush with its sibling panels.
#[function_component]
pub fn Panel(props: &PanelProps) -> Html {
    let class = classes!(
        "yi-panel",
        format!("yi-panel--{}", props.fill.as_str()),
        props.collapsible.then_some("yi-panel--collapsible"),
        props.class.clone(),
    );
    let body = html! {
        <div class={classes!("yi-panel__body", props.flush.then_some("yi-panel__body--flush"))}>
            { props.children.clone() }
        </div>
    };

    // The browser fires `toggle` for the initial state too; only report changes.
    let was_open = use_mut_ref(|| !props.collapsed);

    if props.collapsible {
        let ontoggle = {
            let on_toggle = props.on_toggle.clone();
            Callback::from(move |event: Event| {
                if let Some(open) = is_open(&event)
                    && was_open.replace(open) != open
                {
                    on_toggle.emit(open);
                }
            })
        };

        return props.attrs.apply(html! {
            <details id={props.id.clone()} {class} open={!props.collapsed} {ontoggle}>
                <summary class="yi-panel__header">
                    <Icon icon={icons::CHEVRON_RIGHT} class="yi-panel__chevron" />
                    <span class="yi-panel__heading">
                        <span class="yi-panel__title">{ props.title.clone() }</span>
                        if let Some(subtitle) = &props.subtitle {
                            <span class="yi-panel__subtitle">{ subtitle.clone() }</span>
                        }
                    </span>
                </summary>
                { body }
            </details>
        });
    }

    let empty = Html::default();
    let has_header = props.title.is_some() || props.subtitle.is_some() || props.actions != empty;

    props.attrs.apply(html! {
        <section id={props.id.clone()} {class}>
            if has_header {
                <header class="yi-panel__header">
                    <div class="yi-panel__heading">
                        if let Some(title) = &props.title {
                            <@{format!("h{}", props.level.clamp(1, 6))} class="yi-panel__title">
                                { title.clone() }
                            </@>
                        }
                        if let Some(subtitle) = &props.subtitle {
                            <p class="yi-panel__subtitle">{ subtitle.clone() }</p>
                        }
                    </div>
                    if props.actions != empty {
                        <div class="yi-panel__actions">{ props.actions.clone() }</div>
                    }
                </header>
            }
            { body }
        </section>
    })
}

/// Whether the `<details>` a `toggle` event came from is now open.
#[cfg(target_arch = "wasm32")]
fn is_open(event: &Event) -> Option<bool> {
    let target = event.target()?;
    js_sys::Reflect::get(&target, &"open".into()).ok()?.as_bool()
}

#[cfg(not(target_arch = "wasm32"))]
fn is_open(_: &Event) -> Option<bool> {
    None
}

#[cfg(test)]
mod tests;
