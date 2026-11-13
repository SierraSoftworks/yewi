//! Tooltips: a short description shown beside an element while it is hovered
//! or has keyboard focus.

use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::{
    AnchorOptions, Attrs, Placement, components::server_only::use_server_only, is_empty, use_anchored, use_id,
};

mod hint;

pub(crate) use hint::use_hint;

/// How a tooltip is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TooltipVariant {
    /// A small card on the page's surface colour that follows the theme.
    /// Suits rich content such as pills and secondary lines.
    #[default]
    Surface,
    /// A near-black bubble in both themes, for the strongest separation from
    /// the page.
    Dark,
}

impl TooltipVariant {
    pub const ALL: [TooltipVariant; 2] = [TooltipVariant::Surface, TooltipVariant::Dark];

    const fn as_str(self) -> &'static str {
        match self {
            TooltipVariant::Surface => "surface",
            TooltipVariant::Dark => "dark",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    /// The element the tooltip describes.
    #[prop_or_default]
    pub children: Html,
    /// What the tooltip says: a string or any phrasing content. When empty,
    /// no tooltip is shown.
    #[prop_or_default]
    pub tip: Html,
    /// The preferred side. The tooltip flips to the opposite side and slides
    /// along it to stay on screen.
    #[prop_or_default]
    pub placement: Placement,
    #[prop_or_default]
    pub variant: TooltipVariant,
    /// Draws an arrow pointing at the element.
    #[prop_or(true)]
    pub arrow: bool,
    /// Milliseconds the pointer rests before the tooltip shows. Keyboard
    /// focus shows it at once, as does moving on from another tooltip.
    #[prop_or(400)]
    pub delay_ms: u32,
    /// Makes the wrapper a tab stop, for children that cannot take focus
    /// themselves (an icon, a status dot, truncated text), so keyboard users
    /// can reach the tooltip too.
    #[prop_or_default]
    pub focusable: bool,
    /// Takes control of visibility: `true` keeps the tooltip shown and
    /// `false` hidden, whatever the pointer and focus do.
    #[prop_or_default]
    pub open: Option<bool>,
    /// Never shows the tooltip.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Why the tooltip is, or is about to be, shown.
#[derive(Default)]
struct Intent {
    hover: bool,
    focus: bool,
    /// The pending hover delay.
    timer: Option<dom::Timeout>,
}

struct Handlers {
    enter: Callback<MouseEvent>,
    leave: Callback<MouseEvent>,
    press: Callback<PointerEvent>,
    focus_in: Callback<FocusEvent>,
    focus_out: Callback<FocusEvent>,
    dismiss: Callback<()>,
}

impl Handlers {
    fn new(anchor: NodeRef, bubble: NodeRef, shown: UseStateSetter<bool>, delay_ms: u32) -> Self {
        let intent = Rc::new(RefCell::new(Intent::default()));
        let hide = {
            let shown = shown.clone();
            move || {
                dom::note_closed();
                shown.set(false);
            }
        };

        let enter = {
            let (intent, anchor, shown) = (intent.clone(), anchor.clone(), shown.clone());
            Callback::from(move |event: MouseEvent| {
                // Moving between the element and its tooltip is not a new hover.
                if dom::came_from_inside(&event, &anchor) {
                    return;
                }
                let mut intent = intent.borrow_mut();
                intent.hover = true;
                let delay = if dom::recently_closed() { 0 } else { delay_ms };
                if delay == 0 {
                    intent.timer = None;
                    shown.set(true);
                } else {
                    let shown = shown.clone();
                    intent.timer = Some(dom::Timeout::new(delay, move || shown.set(true)));
                }
            })
        };
        let leave = {
            let (intent, anchor, hide) = (intent.clone(), anchor.clone(), hide.clone());
            Callback::from(move |event: MouseEvent| {
                if dom::came_from_inside(&event, &anchor) {
                    return;
                }
                let mut intent = intent.borrow_mut();
                intent.hover = false;
                intent.timer = None;
                if !intent.focus {
                    hide();
                }
            })
        };
        // Pressing the element means the user is done reading about it.
        let press = {
            let (intent, hide) = (intent.clone(), hide.clone());
            Callback::from(move |event: PointerEvent| {
                if dom::is_inside(&event, &bubble) {
                    return;
                }
                let mut intent = intent.borrow_mut();
                intent.hover = false;
                intent.timer = None;
                if !intent.focus {
                    hide();
                }
            })
        };
        let focus_in = {
            let (intent, shown) = (intent.clone(), shown.clone());
            Callback::from(move |event: FocusEvent| {
                // Focus from a click is not a request for the tooltip.
                if dom::is_keyboard_focus(&event) {
                    let mut intent = intent.borrow_mut();
                    intent.focus = true;
                    intent.timer = None;
                    shown.set(true);
                }
            })
        };
        let focus_out = {
            let (intent, hide) = (intent.clone(), hide.clone());
            Callback::from(move |event: FocusEvent| {
                if dom::moved_inside(&event, &anchor) {
                    return;
                }
                let mut intent = intent.borrow_mut();
                intent.focus = false;
                if !intent.hover {
                    hide();
                }
            })
        };
        let dismiss = Callback::from(move |()| {
            *intent.borrow_mut() = Intent::default();
            hide();
        });

        Self { enter, leave, press, focus_in, focus_out, dismiss }
    }
}

/// Describes its child in a small bubble while the child is hovered or has
/// keyboard focus.
///
/// - The tooltip can itself be hovered, so its text can be selected and its
///   links followed, and <kbd>Esc</kbd> dismisses it. Only one tooltip
///   shows at a time.
/// - While shown it is the accessible description of the first focusable
///   element inside. It supplements a label and does not replace one: an
///   icon-only button still needs its own `aria-label`.
/// - The bubble exists only while shown, so a table of thousands of
///   tooltips costs one wrapper element each.
#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let tip_id = use_id();
    let anchor = use_node_ref();
    let bubble = use_node_ref();
    let shown = use_state(|| false);

    let handlers = {
        let (anchor, bubble, shown) = (anchor.clone(), bubble.clone(), shown.setter());
        use_memo(props.delay_ms, move |&delay_ms| Handlers::new(anchor, bubble, shown, delay_ms))
    };

    let native = use_server_only();
    let available = !props.disabled && !is_empty(&props.tip);
    let open = !native && available && props.open.unwrap_or(*shown);

    let gap = if props.arrow { 9.0 } else { 6.0 };
    use_anchored(&anchor, &bubble, open, AnchorOptions::new(props.placement).offset(gap));

    {
        let (anchor, tip_id, dismiss) = (anchor.clone(), tip_id.clone(), handlers.dismiss.clone());
        use_effect_with(open, move |&open| {
            let session = open.then(|| dom::Session::start(&anchor, Some(&tip_id), dismiss));
            move || {
                let _end = session;
            }
        });
    }

    let class = classes!("yi-tooltip", props.class.clone());
    let bubble_class = classes!(
        "yi-anchored",
        "yi-tooltip__bubble",
        format!("yi-tooltip__bubble--{}", props.variant.as_str()),
        props.arrow.then_some("yi-tooltip__bubble--arrow"),
    );

    // Without a client the bubble is always in the page as a native popover
    // the stylesheet places, and `BEHAVIOURS` shows it on hover and focus.
    if native {
        let bubble_class = classes!(
            "yi-anchored",
            "yi-anchored--native",
            "yi-tooltip__bubble",
            format!("yi-tooltip__bubble--{}", props.variant.as_str()),
        );
        return props.attrs.apply(html! {
            <span
                id={props.id.clone()}
                {class}
                tabindex={props.focusable.then_some("0")}
                data-yi-popover="hover"
                data-yi-delay={props.delay_ms.to_string()}
            >
                { props.children.clone() }
                if available && props.open != Some(false) {
                    <span
                        id={tip_id}
                        class={bubble_class}
                        role="tooltip"
                        popover="manual"
                        data-yi-side={props.placement.side()}
                        data-yi-align={props.placement.align()}
                        data-yi-open={(props.open == Some(true)).then_some("")}
                    >
                        { props.tip.clone() }
                    </span>
                }
            </span>
        });
    }

    props.attrs.apply(html! {
        <span
            ref={anchor}
            id={props.id.clone()}
            {class}
            tabindex={props.focusable.then_some("0")}
            onmouseenter={handlers.enter.clone()}
            onmouseleave={handlers.leave.clone()}
            onpointerdown={handlers.press.clone()}
            onfocusin={handlers.focus_in.clone()}
            onfocusout={handlers.focus_out.clone()}
        >
            { props.children.clone() }
            if open {
                <span ref={bubble} id={tip_id} class={bubble_class} role="tooltip" popover="manual">
                    { props.tip.clone() }
                </span>
            }
        </span>
    })
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use std::cell::{Cell, RefCell};

    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{Element, Event, EventTarget, FocusEvent, KeyboardEvent, MouseEvent, Node};
    use yew::{AttrValue, Callback, NodeRef};

    /// How long after one tooltip closes the next opens without a delay.
    const WARM_MS: f64 = 300.0;
    /// What can take focus, leaving out anything inside a tooltip.
    const FOCUSABLE: &str =
        ":is(a[href], button, input, select, textarea, summary, [tabindex]):not([role=tooltip] *)";

    thread_local! {
        static LAST_CLOSED: Cell<f64> = const { Cell::new(f64::NEG_INFINITY) };
        /// Dismisses the tooltip now showing: only one shows at a time.
        static CURRENT: RefCell<Option<Callback<()>>> = const { RefCell::new(None) };
    }

    pub fn note_closed() {
        LAST_CLOSED.set(js_sys::Date::now());
    }

    pub fn recently_closed() -> bool {
        js_sys::Date::now() - LAST_CLOSED.get() < WARM_MS
    }

    fn contains(container: &NodeRef, target: Option<EventTarget>) -> bool {
        let target = target.and_then(|t| t.dyn_into::<Node>().ok());
        container.cast::<Node>().zip(target).is_some_and(|(container, target)| container.contains(Some(&target)))
    }

    /// Whether the pointer came from (or went to) somewhere inside `container`.
    pub fn came_from_inside(event: &MouseEvent, container: &NodeRef) -> bool {
        contains(container, event.related_target())
    }

    /// Whether focus moved to somewhere inside `container`.
    pub fn moved_inside(event: &FocusEvent, container: &NodeRef) -> bool {
        contains(container, event.related_target())
    }

    pub fn is_inside(event: &Event, container: &NodeRef) -> bool {
        contains(container, event.target())
    }

    pub fn is_keyboard_focus(event: &FocusEvent) -> bool {
        event
            .target()
            .and_then(|t| t.dyn_into::<Element>().ok())
            .is_some_and(|target| target.matches(":focus-visible").unwrap_or(true))
    }

    /// A `setTimeout` that is cancelled when dropped. Shared with Toast: its
    /// handle is an opaque value, since not every environment hands out
    /// timer ids that fit an `i32`.
    pub struct Timeout(#[allow(dead_code)] Option<crate::components::toast::timer::Timeout>);

    impl Timeout {
        pub fn new(millis: u32, run: impl FnOnce() + 'static) -> Self {
            Self(crate::components::toast::timer::Timeout::start(millis, run))
        }
    }

    /// One showing of a tooltip: wires up the description and Escape, and
    /// undoes both when dropped.
    pub struct Session {
        described: Option<(Element, Option<String>)>,
        keydown: Closure<dyn FnMut(KeyboardEvent)>,
        dismiss: Callback<()>,
    }

    impl Session {
        /// With `tip_id` the tooltip becomes the description of the element;
        /// a hint repeats the element's name, so it passes none.
        pub fn start(anchor: &NodeRef, tip_id: Option<&AttrValue>, dismiss: Callback<()>) -> Self {
            if let Some(previous) = CURRENT.replace(Some(dismiss.clone())) {
                previous.emit(());
            }

            let described = anchor.cast::<Element>().zip(tip_id).map(|(anchor, tip_id)| {
                let target = anchor.query_selector(FOCUSABLE).ok().flatten().unwrap_or(anchor);
                let before = target.get_attribute("aria-describedby");
                let value = before.as_ref().map_or_else(|| tip_id.to_string(), |ids| format!("{ids} {tip_id}"));
                let _ = target.set_attribute("aria-describedby", &value);
                (target, before)
            });

            let keydown = {
                let dismiss = dismiss.clone();
                Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                    if event.key() == "Escape" {
                        dismiss.emit(());
                    }
                })
            };
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let _ =
                    document.add_event_listener_with_callback_and_bool("keydown", keydown.as_ref().unchecked_ref(), true);
            }

            Self { described, keydown, dismiss }
        }
    }

    /// The listeners a hint puts on its anchor. A hint has no wrapper of its
    /// own to carry them, so they are attached to the element directly.
    pub struct Wiring {
        target: EventTarget,
        listeners: Vec<(&'static str, Listener)>,
    }

    type Listener = Closure<dyn FnMut(Event)>;

    impl Wiring {
        /// Attaches the handlers to the element `anchor` points at, unless
        /// they already are. Called after every render, since the element
        /// can come, go or be replaced.
        pub fn sync(slot: &RefCell<Option<Wiring>>, anchor: &NodeRef, handlers: &super::Handlers) {
            let target = anchor.cast::<Element>().map(EventTarget::from);
            let mut slot = slot.borrow_mut();
            if slot.as_ref().map(|wiring| &wiring.target) == target.as_ref() {
                return;
            }
            *slot = target.map(|target| Self::attach(target, handlers));
        }

        fn attach(target: EventTarget, handlers: &super::Handlers) -> Self {
            // The native `title` is the hint where there is no runtime. Here
            // it would show a second tooltip, so it goes as soon as ours can.
            fn untitle(event: &Event) {
                if let Some(element) = event.current_target().and_then(|t| t.dyn_into::<Element>().ok()) {
                    let _ = element.remove_attribute("title");
                }
            }
            fn listener<E: JsCast + 'static>(callback: &Callback<E>, untitles: bool) -> Listener {
                let callback = callback.clone();
                Closure::new(move |event: Event| {
                    if untitles {
                        untitle(&event);
                    }
                    callback.emit(event.unchecked_into());
                })
            }

            let listeners = vec![
                ("mouseenter", listener(&handlers.enter, true)),
                ("mouseleave", listener(&handlers.leave, false)),
                ("pointerdown", listener(&handlers.press, false)),
                ("focusin", listener(&handlers.focus_in, true)),
                ("focusout", listener(&handlers.focus_out, false)),
                // Activating the control, from the keyboard too, ends the hint.
                ("click", listener(&handlers.dismiss.reform(|_: Event| ()), false)),
            ];
            for (kind, closure) in &listeners {
                let _ = target.add_event_listener_with_callback(kind, closure.as_ref().unchecked_ref());
            }
            Self { target, listeners }
        }
    }

    impl Drop for Wiring {
        fn drop(&mut self) {
            for (kind, closure) in &self.listeners {
                let _ = self.target.remove_event_listener_with_callback(kind, closure.as_ref().unchecked_ref());
            }
        }
    }

    impl Drop for Session {
        fn drop(&mut self) {
            CURRENT.with_borrow_mut(|current| {
                if current.as_ref() == Some(&self.dismiss) {
                    *current = None;
                }
            });
            if let Some((target, before)) = self.described.take() {
                let _ = match before {
                    Some(ids) => target.set_attribute("aria-describedby", &ids),
                    None => target.remove_attribute("aria-describedby"),
                };
            }
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let _ = document.remove_event_listener_with_callback_and_bool(
                    "keydown",
                    self.keydown.as_ref().unchecked_ref(),
                    true,
                );
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    //! Events and effects never run outside a browser.

    use yew::{AttrValue, Callback, NodeRef};

    pub fn note_closed() {}

    pub fn recently_closed() -> bool {
        false
    }

    pub fn came_from_inside<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn moved_inside<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn is_inside<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn is_keyboard_focus<E>(_: &E) -> bool {
        false
    }

    pub struct Timeout;

    impl Timeout {
        pub fn new(_: u32, _: impl FnOnce() + 'static) -> Self {
            Self
        }
    }

    pub struct Session;

    impl Session {
        pub fn start(_: &NodeRef, _: Option<&AttrValue>, _: Callback<()>) -> Self {
            Self
        }
    }

    pub struct Wiring;
}

#[cfg(test)]
mod tests;
