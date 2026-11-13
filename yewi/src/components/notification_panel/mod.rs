//! Notifications: a bell with a count that opens the list of what needs
//! attention, and the list itself for use in a page.

use yew::prelude::*;

use crate::{
    Attrs, Badge, ButtonVariant, IconButton, IconData, NavTarget, Placement, Popover, StatusDot, Tone, icons,
    nav_target::use_nav_link, is_empty};

#[derive(Properties, PartialEq)]
pub struct NotificationPanelProps {
    /// The notifications: [`NotificationItem`]s. They are built into the page
    /// only while the panel is open.
    #[prop_or_default]
    pub children: Html,
    /// The panel's heading, and the name of the bell button unless `label`
    /// is set.
    #[prop_or(AttrValue::Static("Notifications"))]
    pub title: AttrValue,
    /// The bell's accessible name and hover hint, when it should say more
    /// than `title`: "3 hosts need attention".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// A muted line under the heading saying what the list covers.
    #[prop_or_default]
    pub description: Html,
    /// The number on the bell. Zero hides it.
    #[prop_or_default]
    pub count: u32,
    /// Counts above this show as "99+".
    #[prop_or(99)]
    pub max: u32,
    /// The colour of the count: the most severe tone in the list.
    #[prop_or(Tone::Info)]
    pub tone: Tone,
    /// The icon on the button.
    #[prop_or(icons::BELL)]
    pub icon: IconData,
    /// Shown in place of the list when there are no children.
    #[prop_or_else(|| html! { "Nothing needs your attention." })]
    pub empty: Html,
    /// An optional row under the list, such as a link to every notification
    /// or a "mark all as read" action.
    #[prop_or_default]
    pub footer: Html,
    /// Where the panel opens. It flips and slides to stay on screen.
    #[prop_or(Placement::BottomEnd)]
    pub placement: Placement,
    /// Takes control of visibility. Pair it with `on_open_change`.
    #[prop_or_default]
    pub open: Option<bool>,
    /// Called with the state the panel wants to move to: the bell,
    /// <kbd>Esc</kbd>, a press outside or following a link in the list.
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A bell button with a count that opens a panel listing what needs
/// attention.
///
/// - The panel is a [`Popover`]: the bell toggles it, <kbd>Esc</kbd> and a
///   press outside close it, and it follows the bell in the tab order.
/// - Following a link inside the panel closes it.
/// - The list exists in the page only while the panel is open, so a long
///   list costs nothing until it is asked for.
#[function_component]
pub fn NotificationPanel(props: &NotificationPanelProps) -> Html {
    let shown = use_state(|| false);
    let open = props.open.unwrap_or(*shown);

    let on_open_change = {
        let (shown, on_open_change) = (shown.setter(), props.on_open_change.clone());
        Callback::from(move |next: bool| {
            shown.set(next);
            on_open_change.emit(next);
        })
    };
    // A link in the list leads elsewhere: the panel has done its job.
    let follow = {
        let on_open_change = on_open_change.clone();
        Callback::from(move |event: MouseEvent| {
            if dom::is_link(&event) {
                on_open_change.emit(false);
            }
        })
    };

    let header = html! {
        <span class="yi-notification-panel__header">
            <span class="yi-notification-panel__title">{ props.title.clone() }</span>
            if !is_empty(&props.description) {
                <span class="yi-notification-panel__description">{ props.description.clone() }</span>
            }
        </span>
    };
    let content = html! {
        <div class="yi-notification-panel__content" onclick={follow}>
            if is_empty(&props.children) {
                <p class="yi-notification-panel__empty">{ props.empty.clone() }</p>
            } else {
                <NotificationList>{ props.children.clone() }</NotificationList>
            }
        </div>
    };

    html! {
        <Popover
            open={open}
            {on_open_change}
            {header}
            {content}
            footer={props.footer.clone()}
            placement={props.placement}
            arrow=false
            flush=true
            label={props.title.clone()}
            surface_class="yi-notification-panel__surface"
            class={classes!("yi-notification-panel", props.class.clone())}
            id={props.id.clone()}
            attrs={props.attrs.clone()}
        >
            <span class="yi-notification-panel__trigger">
                <IconButton
                    icon={props.icon}
                    label={props.label.clone().unwrap_or_else(|| props.title.clone())}
                    variant={ButtonVariant::Subtle}
                />
                // The button's name already says what the number means.
                if props.count > 0 {
                    <span class="yi-notification-panel__count" aria-hidden="true">
                        <Badge count={props.count} max={props.max} tone={props.tone} bold=true />
                    </span>
                }
            </span>
        </Popover>
    }
}

#[derive(Properties, PartialEq)]
pub struct NotificationListProps {
    /// The [`NotificationItem`]s.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The list inside a [`NotificationPanel`], for showing the same items in a
/// page or a card.
#[function_component]
pub fn NotificationList(props: &NotificationListProps) -> Html {
    props.attrs.apply(html! {
        // Safari drops the list role once the markers are removed.
        <ul id={props.id.clone()} class={classes!("yi-notification-list", props.class.clone())} role="list">
            { props.children.clone() }
        </ul>
    })
}

#[derive(Properties, PartialEq)]
pub struct NotificationItemProps {
    /// What the notification is about: a host, a build, a person.
    pub title: Html,
    /// Why it needs attention: plain text, or one [`NotificationReason`] per
    /// cause.
    #[prop_or_default]
    pub children: Html,
    /// How serious it is, shown as the rule along the leading edge.
    #[prop_or_default]
    pub tone: Tone,
    /// Makes the title a link to the subject.
    #[prop_or_default]
    pub href: NavTarget,
    /// A short note at the end of the title row, such as a `Timestamp`.
    #[prop_or_default]
    pub meta: Html,
    /// Fired when the title link is followed.
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One notification: a subject, why it needs attention and how serious it is.
#[function_component]
pub fn NotificationItem(props: &NotificationItemProps) -> Html {
    let link = use_nav_link(&props.href, props.on_click.clone());
    let class = classes!("yi-notification", format!("yi-notification--{}", props.tone.as_str()), props.class.clone());

    props.attrs.apply(html! {
        <li id={props.id.clone()} {class}>
            <span class="yi-notification__row">
                if let Some(href) = link.href {
                    <a class="yi-notification__title" {href} onclick={link.onclick}>{ props.title.clone() }</a>
                } else {
                    <span class="yi-notification__title">{ props.title.clone() }</span>
                }
                if !is_empty(&props.meta) {
                    <span class="yi-notification__meta">{ props.meta.clone() }</span>
                }
            </span>
            if !is_empty(&props.children) {
                <span class="yi-notification__body">{ props.children.clone() }</span>
            }
        </li>
    })
}

#[derive(Properties, PartialEq)]
pub struct NotificationReasonProps {
    /// The cause, in a few words: "Offline for 12 minutes".
    #[prop_or_default]
    pub children: Html,
    /// How serious this cause is, shown as a dot before the text.
    #[prop_or_default]
    pub tone: Tone,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One cause inside a [`NotificationItem`] that has several, each with its
/// own tone.
#[function_component]
pub fn NotificationReason(props: &NotificationReasonProps) -> Html {
    props.attrs.apply(html! {
        <span id={props.id.clone()} class={classes!("yi-notification__reason", props.class.clone())}>
            <StatusDot tone={props.tone} />
            <span>{ props.children.clone() }</span>
        </span>
    })
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use wasm_bindgen::JsCast;
    use web_sys::{Element, MouseEvent};

    /// Whether the press landed on a link.
    pub fn is_link(event: &MouseEvent) -> bool {
        event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .is_some_and(|target| target.closest("a[href]").ok().flatten().is_some())
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    //! Events never run outside a browser.

    pub fn is_link<E>(_: &E) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
