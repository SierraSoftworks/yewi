//! Popovers: a floating surface attached to the element that opens it, for
//! detail too rich for a tooltip and too small for a dialog.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use yew::prelude::*;

use crate::{
    AnchorOptions, Attrs, Placement, components::server_only::use_server_only, is_empty, use_anchored, use_id,
};

/// What opens and closes a popover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum PopoverTrigger {
    /// Pressing the trigger toggles it. For content people act on.
    #[default]
    Click,
    /// Shown while the trigger or the popover is hovered, or the trigger has
    /// keyboard focus. For detail people only read.
    Hover,
    /// Shown while focus is inside the trigger or the popover: suggestions
    /// under an input, for example.
    Focus,
    /// Nothing opens it: drive it with `open`. <kbd>Esc</kbd> and a press
    /// outside still ask to close it through `on_open_change`.
    Manual,
}

impl PopoverTrigger {
    pub const ALL: [PopoverTrigger; 4] =
        [PopoverTrigger::Click, PopoverTrigger::Hover, PopoverTrigger::Focus, PopoverTrigger::Manual];
}

#[derive(Properties, PartialEq)]
pub struct PopoverProps {
    /// The trigger: the element the popover belongs to and points at.
    #[prop_or_default]
    pub children: Html,
    /// The popover's body. Its layout is the caller's: the popover adds only
    /// the surface and padding.
    #[prop_or_default]
    pub content: Html,
    /// An optional heading row above the body, such as a status dot and a
    /// short label.
    #[prop_or_default]
    pub header: Html,
    /// An optional muted row below the body, such as a timestamp or a link.
    #[prop_or_default]
    pub footer: Html,
    #[prop_or_default]
    pub trigger: PopoverTrigger,
    /// The preferred side. The popover flips to the opposite side and slides
    /// along it to stay on screen.
    #[prop_or_default]
    pub placement: Placement,
    /// Draws an arrow pointing at the trigger.
    #[prop_or(true)]
    pub arrow: bool,
    /// Removes the padding around the body, for content that runs edge to
    /// edge such as a list of options. The header and footer keep theirs.
    #[prop_or_default]
    pub flush: bool,
    /// Names the popover for assistive technology when its content does not.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// The popover's ARIA role. Defaults to `tooltip` for `Hover`, where the
    /// content describes the trigger, and `group` otherwise.
    #[prop_or_default]
    pub role: Option<AttrValue>,
    /// Takes control of visibility: `true` shows the popover and `false`
    /// hides it. Pair it with `on_open_change`.
    #[prop_or_default]
    pub open: Option<bool>,
    /// Whether an uncontrolled popover starts open.
    #[prop_or_default]
    pub default_open: bool,
    /// Called with the state the popover wants to move to, whenever the
    /// trigger, <kbd>Esc</kbd> or a press outside changes it.
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    /// Milliseconds the pointer rests before a `Hover` popover shows.
    /// Keyboard focus shows it at once.
    #[prop_or(150)]
    pub delay_ms: u32,
    /// Makes the wrapper a tab stop (and, for `Click`, a button), for
    /// triggers that cannot take focus themselves: an icon, a status dot, a
    /// chart segment.
    #[prop_or_default]
    pub focusable: bool,
    /// Never shows the popover.
    #[prop_or_default]
    pub disabled: bool,
    /// Extra classes for the floating surface, for example to set its width
    /// through `--yi-popover-max-width`.
    #[prop_or_default]
    pub surface_class: Classes,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Why a `Hover` popover is, or is about to be, shown.
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
    click: Callback<MouseEvent>,
    key_down: Callback<KeyboardEvent>,
    focus_in: Callback<FocusEvent>,
    focus_out: Callback<FocusEvent>,
    dismiss: Callback<()>,
}

impl Handlers {
    fn new(
        trigger: PopoverTrigger,
        delay_ms: u32,
        anchor: NodeRef,
        surface: NodeRef,
        // The state last asked for, which a render may not have caught up with.
        wanted: Rc<Cell<bool>>,
        shown: UseStateSetter<bool>,
        on_open_change: Callback<bool>,
    ) -> Self {
        let intent = Rc::new(RefCell::new(Intent::default()));
        let request = {
            let wanted = wanted.clone();
            move |next: bool| {
                if wanted.replace(next) != next {
                    shown.set(next);
                    on_open_change.emit(next);
                }
            }
        };
        let toggle = {
            let request = request.clone();
            move || request(!wanted.get())
        };

        let enter = {
            let (intent, anchor, request) = (intent.clone(), anchor.clone(), request.clone());
            Callback::from(move |event: MouseEvent| {
                // Moving between the trigger and its popover is not a new hover.
                if trigger != PopoverTrigger::Hover || dom::came_from_inside(&event, &anchor) {
                    return;
                }
                let mut intent = intent.borrow_mut();
                intent.hover = true;
                if delay_ms == 0 {
                    intent.timer = None;
                    request(true);
                } else {
                    let request = request.clone();
                    intent.timer = dom::Timeout::start(delay_ms, move || request(true));
                }
            })
        };
        let leave = {
            let (intent, anchor, request) = (intent.clone(), anchor.clone(), request.clone());
            Callback::from(move |event: MouseEvent| {
                if trigger != PopoverTrigger::Hover || dom::came_from_inside(&event, &anchor) {
                    return;
                }
                let mut intent = intent.borrow_mut();
                intent.hover = false;
                intent.timer = None;
                if !intent.focus {
                    request(false);
                }
            })
        };
        let click = {
            let (surface, toggle) = (surface.clone(), toggle.clone());
            Callback::from(move |event: MouseEvent| {
                // A press inside the popover belongs to its content.
                if trigger == PopoverTrigger::Click && !dom::is_inside(&event, &surface) {
                    toggle();
                }
            })
        };
        // The wrapper acting as a button for a trigger that is not one.
        let key_down = {
            let anchor = anchor.clone();
            Callback::from(move |event: KeyboardEvent| {
                if trigger == PopoverTrigger::Click
                    && matches!(event.key().as_str(), "Enter" | " ")
                    && dom::is_target(&event, &anchor)
                {
                    event.prevent_default();
                    toggle();
                }
            })
        };
        let focus_in = {
            let (intent, surface, request) = (intent.clone(), surface.clone(), request.clone());
            Callback::from(move |event: FocusEvent| match trigger {
                PopoverTrigger::Focus => request(true),
                // Focus from a click is not a request for the popover.
                PopoverTrigger::Hover if dom::is_keyboard_focus(&event) && !dom::is_inside(&event, &surface) => {
                    let mut intent = intent.borrow_mut();
                    intent.focus = true;
                    intent.timer = None;
                    request(true);
                }
                _ => {}
            })
        };
        let focus_out = {
            let (intent, request) = (intent.clone(), request.clone());
            Callback::from(move |event: FocusEvent| {
                if dom::moved_inside(&event, &anchor) {
                    return;
                }
                match trigger {
                    PopoverTrigger::Hover => {
                        let mut intent = intent.borrow_mut();
                        intent.focus = false;
                        if !intent.hover {
                            request(false);
                        }
                    }
                    // Focus that went nowhere (a press on plain text, another
                    // window) is not a reason to close; a press outside is
                    // handled on its own.
                    PopoverTrigger::Click | PopoverTrigger::Focus if dom::moved_elsewhere(&event) => request(false),
                    _ => {}
                }
            })
        };
        let dismiss = Callback::from(move |()| {
            *intent.borrow_mut() = Intent::default();
            request(false);
        });

        Self { enter, leave, click, key_down, focus_in, focus_out, dismiss }
    }
}

/// A floating surface attached to its child, the trigger, with an arrow
/// pointing at it and optional `header` and `footer` rows around the content.
///
/// - `trigger` picks what opens it: a click, hover and keyboard focus, focus
///   alone, or nothing but the `open` prop. <kbd>Esc</kbd> and a press
///   outside close it, and popovers nest.
/// - The popover follows its trigger in the tab order, so <kbd>Tab</kbd>
///   moves from the trigger into the content and on to the rest of the page.
/// - A `Click` popover marks its trigger `aria-expanded`; a `Hover` popover
///   is its trigger's description.
/// - The surface exists only while open, so a strip of thousands of triggers
///   costs one wrapper element each.
#[function_component]
pub fn Popover(props: &PopoverProps) -> Html {
    let surface_id = use_id();
    let anchor = use_node_ref();
    let surface = use_node_ref();
    let shown = use_state(|| props.default_open);
    let wanted = use_mut_ref(|| Rc::new(Cell::new(props.default_open)));
    let wanted = wanted.borrow().clone();

    let handlers = {
        let (anchor, surface, wanted, shown) = (anchor.clone(), surface.clone(), wanted.clone(), shown.setter());
        use_memo((props.trigger, props.delay_ms, props.on_open_change.clone()), move |(trigger, delay_ms, on_change)| {
            Handlers::new(*trigger, *delay_ms, anchor, surface, wanted, shown, on_change.clone())
        })
    };

    let native = use_server_only() && matches!(props.trigger, PopoverTrigger::Click | PopoverTrigger::Hover);
    let open = !native && !props.disabled && props.open.unwrap_or(*shown);
    wanted.set(open);

    let gap = if props.arrow { 9.0 } else { 6.0 };
    use_anchored(&anchor, &surface, open, AnchorOptions::new(props.placement).offset(gap).arrow_padding(14.0));

    {
        let (anchor, surface, surface_id) = (anchor.clone(), surface.clone(), surface_id.clone());
        let dismiss = handlers.dismiss.clone();
        use_effect_with((open, props.trigger), move |&(open, trigger)| {
            let session = open.then(|| dom::Session::start(&anchor, &surface, &surface_id, trigger, dismiss));
            move || {
                let _end = session;
            }
        });
    }

    let hover = props.trigger == PopoverTrigger::Hover;
    let role = props.role.clone().unwrap_or(AttrValue::Static(if hover { "tooltip" } else { "group" }));
    let acts_as_button = props.focusable && props.trigger == PopoverTrigger::Click;
    let class = classes!("yi-popover", props.class.clone());
    let surface_class = classes!(
        "yi-anchored",
        "yi-popover__surface",
        props.arrow.then_some("yi-popover__surface--arrow"),
        props.flush.then_some("yi-popover__surface--flush"),
        props.surface_class.clone(),
    );

    let body = html! {
        <>
            if !is_empty(&props.header) {
                <span class="yi-popover__header">{ props.header.clone() }</span>
            }
            <span class="yi-popover__body">{ props.content.clone() }</span>
            if !is_empty(&props.footer) {
                <span class="yi-popover__footer">{ props.footer.clone() }</span>
            }
        </>
    };

    // Without a client the surface is always in the page, the browser shows
    // it as a native popover and the stylesheet places it. `BEHAVIOURS` does
    // the opening, since the trigger is whatever the caller passed.
    if native {
        let surface_class = classes!(
            "yi-anchored",
            "yi-anchored--native",
            "yi-popover__surface",
            props.flush.then_some("yi-popover__surface--flush"),
            props.surface_class.clone(),
        );
        return props.attrs.apply(html! {
            <span
                id={props.id.clone()}
                {class}
                role={acts_as_button.then_some("button")}
                tabindex={props.focusable.then_some("0")}
                data-yi-popover={if hover { "hover" } else { "click" }}
                data-yi-delay={hover.then(|| props.delay_ms.to_string())}
            >
                { props.children.clone() }
                if !props.disabled {
                    <span
                        id={surface_id}
                        class={surface_class}
                        {role}
                        aria-label={props.label.clone()}
                        popover={if hover { "manual" } else { "auto" }}
                        data-yi-side={props.placement.side()}
                        data-yi-align={props.placement.align()}
                        data-yi-open={props.open.unwrap_or(props.default_open).then_some("")}
                    >
                        { body }
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
            role={acts_as_button.then_some("button")}
            tabindex={props.focusable.then_some("0")}
            onmouseenter={handlers.enter.clone()}
            onmouseleave={handlers.leave.clone()}
            onclick={handlers.click.clone()}
            onkeydown={handlers.key_down.clone()}
            onfocusin={handlers.focus_in.clone()}
            onfocusout={handlers.focus_out.clone()}
        >
            { props.children.clone() }
            if open {
                <span
                    ref={surface}
                    id={surface_id}
                    class={surface_class}
                    {role}
                    aria-label={props.label.clone()}
                    popover="manual"
                >
                    { body }
                </span>
            }
        </span>
    })
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use std::cell::RefCell;

    use wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen};
    use web_sys::{Element, Event, EventTarget, FocusEvent, HtmlElement, KeyboardEvent, MouseEvent, Node};
    use yew::{AttrValue, Callback, NodeRef};

    use super::PopoverTrigger;

    /// What can take focus, leaving out anything inside a popover's surface.
    const FOCUSABLE: &str =
        ":is(a[href], button, input, select, textarea, summary, [tabindex]):not(.yi-popover__surface *)";

    thread_local! {
        /// The open popovers, oldest first: Escape closes only the newest.
        static OPEN: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
        static NEXT: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
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

    /// Whether focus moved to another element, rather than being dropped.
    pub fn moved_elsewhere(event: &FocusEvent) -> bool {
        event.related_target().is_some()
    }

    pub fn is_inside(event: &Event, container: &NodeRef) -> bool {
        contains(container, event.target())
    }

    /// Whether the event happened on `element` itself, not a descendant.
    pub fn is_target(event: &Event, element: &NodeRef) -> bool {
        let target = event.target().and_then(|t| t.dyn_into::<Node>().ok());
        element.get().zip(target).is_some_and(|(element, target)| element == target)
    }

    pub fn is_keyboard_focus(event: &FocusEvent) -> bool {
        event
            .target()
            .and_then(|t| t.dyn_into::<Element>().ok())
            .is_some_and(|target| target.matches(":focus-visible").unwrap_or(true))
    }

    // Bound directly so the handle stays an opaque value: not every
    // environment hands out timer ids that fit an `i32`.
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = setTimeout)]
        fn set_timeout(handler: &Closure<dyn FnMut()>, millis: u32) -> JsValue;
        #[wasm_bindgen(js_name = clearTimeout)]
        fn clear_timeout(handle: &JsValue);
    }

    /// A `setTimeout` that is cancelled when dropped.
    pub struct Timeout {
        handle: JsValue,
        _closure: Closure<dyn FnMut()>,
    }

    impl Timeout {
        pub fn start(millis: u32, run: impl FnOnce() + 'static) -> Option<Self> {
            let mut run = Some(run);
            let closure = Closure::<dyn FnMut()>::new(move || {
                if let Some(run) = run.take() {
                    run();
                }
            });
            let handle = set_timeout(&closure, millis);
            Some(Self { handle, _closure: closure })
        }
    }

    impl Drop for Timeout {
        fn drop(&mut self) {
            clear_timeout(&self.handle);
        }
    }

    /// The ARIA state a showing gives its trigger, and how to undo it.
    enum Wiring {
        /// `aria-describedby`, with the value it had before.
        Describes(Element, Option<String>),
        /// `aria-expanded` and `aria-controls`.
        Expands(Element),
    }

    /// One showing of a popover: wires up ARIA, Escape and presses outside,
    /// and undoes them when dropped.
    pub struct Session {
        key: u32,
        wiring: Option<Wiring>,
        key_down: Closure<dyn FnMut(KeyboardEvent)>,
        pointer_down: Closure<dyn FnMut(Event)>,
    }

    impl Session {
        pub fn start(
            anchor: &NodeRef,
            surface: &NodeRef,
            surface_id: &AttrValue,
            trigger: PopoverTrigger,
            dismiss: Callback<()>,
        ) -> Self {
            let key = NEXT.replace(NEXT.get().wrapping_add(1));
            OPEN.with_borrow_mut(|open| open.push(key));

            let control = anchor.cast::<Element>().and_then(|anchor| {
                let inner = anchor.query_selector(FOCUSABLE).ok().flatten();
                inner.or_else(|| anchor.has_attribute("tabindex").then_some(anchor))
            });
            let wiring = match trigger {
                PopoverTrigger::Hover => control.clone().or_else(|| anchor.cast::<Element>()).map(|target| {
                    let before = target.get_attribute("aria-describedby");
                    let value =
                        before.as_ref().map_or_else(|| surface_id.to_string(), |ids| format!("{ids} {surface_id}"));
                    let _ = target.set_attribute("aria-describedby", &value);
                    Wiring::Describes(target, before)
                }),
                PopoverTrigger::Click => control.clone().map(|target| {
                    let _ = target.set_attribute("aria-expanded", "true");
                    let _ = target.set_attribute("aria-controls", surface_id);
                    Wiring::Expands(target)
                }),
                // The caller owns the semantics: a combobox, for example.
                PopoverTrigger::Focus | PopoverTrigger::Manual => None,
            };

            let key_down = {
                let (surface, dismiss) = (surface.clone(), dismiss.clone());
                Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                    if event.key() != "Escape" || OPEN.with_borrow(|open| open.last() != Some(&key)) {
                        return;
                    }
                    // This Escape is spent: an enclosing dialog stays open.
                    event.prevent_default();
                    event.stop_propagation();
                    // Focus inside the popover would be lost with it.
                    let active = web_sys::window().and_then(|w| w.document()).and_then(|d| d.active_element());
                    let inside = surface.cast::<Node>().zip(active).is_some_and(|(s, a)| s.contains(Some(&a)));
                    if let Some(control) = control.as_ref().and_then(|c| c.dyn_ref::<HtmlElement>())
                        && inside
                    {
                        let _ = control.focus();
                    }
                    dismiss.emit(());
                })
            };
            let pointer_down = {
                let anchor = anchor.clone();
                Closure::<dyn FnMut(Event)>::new(move |event: Event| {
                    // The surface is inside the anchor, as are nested popovers.
                    if !is_inside(&event, &anchor) {
                        dismiss.emit(());
                    }
                })
            };
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let _ =
                    document.add_event_listener_with_callback_and_bool("keydown", key_down.as_ref().unchecked_ref(), true);
                let _ = document.add_event_listener_with_callback_and_bool(
                    "pointerdown",
                    pointer_down.as_ref().unchecked_ref(),
                    true,
                );
            }

            Self { key, wiring, key_down, pointer_down }
        }
    }

    impl Drop for Session {
        fn drop(&mut self) {
            OPEN.with_borrow_mut(|open| open.retain(|key| *key != self.key));
            match self.wiring.take() {
                Some(Wiring::Describes(target, Some(ids))) => {
                    let _ = target.set_attribute("aria-describedby", &ids);
                }
                Some(Wiring::Describes(target, None)) => {
                    let _ = target.remove_attribute("aria-describedby");
                }
                Some(Wiring::Expands(target)) => {
                    let _ = target.set_attribute("aria-expanded", "false");
                    let _ = target.remove_attribute("aria-controls");
                }
                None => {}
            }
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let _ = document.remove_event_listener_with_callback_and_bool(
                    "keydown",
                    self.key_down.as_ref().unchecked_ref(),
                    true,
                );
                let _ = document.remove_event_listener_with_callback_and_bool(
                    "pointerdown",
                    self.pointer_down.as_ref().unchecked_ref(),
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

    use super::PopoverTrigger;

    pub fn came_from_inside<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn moved_inside<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn moved_elsewhere<E>(_: &E) -> bool {
        false
    }

    pub fn is_inside<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn is_target<E>(_: &E, _: &NodeRef) -> bool {
        false
    }

    pub fn is_keyboard_focus<E>(_: &E) -> bool {
        false
    }

    pub struct Timeout;

    impl Timeout {
        pub fn start(_: u32, _: impl FnOnce() + 'static) -> Option<Self> {
            None
        }
    }

    pub struct Session;

    impl Session {
        pub fn start(_: &NodeRef, _: &NodeRef, _: &AttrValue, _: PopoverTrigger, _: Callback<()>) -> Self {
            Self
        }
    }
}

#[cfg(test)]
mod tests;
