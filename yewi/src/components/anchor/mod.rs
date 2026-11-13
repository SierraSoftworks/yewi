//! Anchor positioning: places a floating element (a tooltip, popover or
//! menu) beside the element it belongs to and keeps it on screen.
//!
//! - [`place_anchored`] is the geometry: pure, and usable anywhere.
//! - [`use_anchored`] applies it to two elements while the floating one is
//!   open, and follows scrolling and resizing.
//!
//! The floating element carries the `yi-anchored` class and, to escape
//! clipping and stacking, `popover="manual"`:
//!
//! ```ignore
//! let (anchor, floating) = (use_node_ref(), use_node_ref());
//! use_anchored(&anchor, &floating, open, AnchorOptions::new(Placement::BottomStart));
//! html! {
//!     <>
//!         <button ref={anchor}>{ "Filters" }</button>
//!         if open {
//!             <div ref={floating} class="yi-anchored" popover="manual">{ "..." }</div>
//!         }
//!     </>
//! }
//! ```
//!
//! The hook writes to the floating element directly, without a render:
//! `translate` (its position), `data-yi-side` and `data-yi-align` (where it
//! ended up, after flipping), `--yi-anchor-arrow` (where along its edge the
//! anchor's centre is) and `--yi-anchor-offset` (the gap). Style an arrow or
//! a side-dependent animation from those. Do not set `style` on the floating
//! element from Yew, which would overwrite them.
//!
//! The floating element is hidden, and cannot take focus, until the hook has
//! placed it, which can be later than its content's own effects. Content that
//! moves focus into itself as it appears waits with [`when_placed`]; the hook
//! announces the first placement with a `yi-placed` event on the element.

use yew::prelude::*;

/// Where a floating element sits relative to its anchor: a side, and how it
/// lines up along that side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Placement {
    /// Above, centred.
    #[default]
    Top,
    /// Above, left edges aligned.
    TopStart,
    /// Above, right edges aligned.
    TopEnd,
    /// Below, centred.
    Bottom,
    /// Below, left edges aligned.
    BottomStart,
    /// Below, right edges aligned.
    BottomEnd,
    /// To the left, centred.
    Left,
    /// To the left, top edges aligned.
    LeftStart,
    /// To the left, bottom edges aligned.
    LeftEnd,
    /// To the right, centred.
    Right,
    /// To the right, top edges aligned.
    RightStart,
    /// To the right, bottom edges aligned.
    RightEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Along {
    Start,
    Center,
    End,
}

impl Placement {
    pub const ALL: [Placement; 12] = [
        Placement::Top,
        Placement::TopStart,
        Placement::TopEnd,
        Placement::Bottom,
        Placement::BottomStart,
        Placement::BottomEnd,
        Placement::Left,
        Placement::LeftStart,
        Placement::LeftEnd,
        Placement::Right,
        Placement::RightStart,
        Placement::RightEnd,
    ];

    const fn parts(self) -> (Side, Along) {
        match self {
            Placement::Top => (Side::Top, Along::Center),
            Placement::TopStart => (Side::Top, Along::Start),
            Placement::TopEnd => (Side::Top, Along::End),
            Placement::Bottom => (Side::Bottom, Along::Center),
            Placement::BottomStart => (Side::Bottom, Along::Start),
            Placement::BottomEnd => (Side::Bottom, Along::End),
            Placement::Left => (Side::Left, Along::Center),
            Placement::LeftStart => (Side::Left, Along::Start),
            Placement::LeftEnd => (Side::Left, Along::End),
            Placement::Right => (Side::Right, Along::Center),
            Placement::RightStart => (Side::Right, Along::Start),
            Placement::RightEnd => (Side::Right, Along::End),
        }
    }

    const fn from_parts(side: Side, along: Along) -> Self {
        match (side, along) {
            (Side::Top, Along::Center) => Placement::Top,
            (Side::Top, Along::Start) => Placement::TopStart,
            (Side::Top, Along::End) => Placement::TopEnd,
            (Side::Bottom, Along::Center) => Placement::Bottom,
            (Side::Bottom, Along::Start) => Placement::BottomStart,
            (Side::Bottom, Along::End) => Placement::BottomEnd,
            (Side::Left, Along::Center) => Placement::Left,
            (Side::Left, Along::Start) => Placement::LeftStart,
            (Side::Left, Along::End) => Placement::LeftEnd,
            (Side::Right, Along::Center) => Placement::Right,
            (Side::Right, Along::Start) => Placement::RightStart,
            (Side::Right, Along::End) => Placement::RightEnd,
        }
    }

    /// The same alignment on the opposite side.
    pub const fn flipped(self) -> Self {
        let (side, along) = self.parts();
        let opposite = match side {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        };
        Self::from_parts(opposite, along)
    }

    /// The side as written to `data-yi-side`: `top`, `bottom`, `left` or `right`.
    pub const fn side(self) -> &'static str {
        match self.parts().0 {
            Side::Top => "top",
            Side::Bottom => "bottom",
            Side::Left => "left",
            Side::Right => "right",
        }
    }

    /// The alignment as written to `data-yi-align`: `start`, `center` or `end`.
    pub const fn align(self) -> &'static str {
        match self.parts().1 {
            Along::Start => "start",
            Along::Center => "center",
            Along::End => "end",
        }
    }

    /// Whether the floating element sits above or below, rather than beside.
    pub const fn is_vertical(self) -> bool {
        matches!(self.parts().0, Side::Top | Side::Bottom)
    }
}

/// A rectangle in viewport coordinates, in CSS pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AnchorRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl AnchorRect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    const fn right(&self) -> f64 {
        self.x + self.width
    }

    const fn bottom(&self) -> f64 {
        self.y + self.height
    }
}

/// How a floating element is placed. Build one with [`AnchorOptions::new`]
/// and adjust it: `AnchorOptions::new(Placement::Bottom).offset(8.0)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnchorOptions {
    /// The preferred placement.
    pub placement: Placement,
    /// The gap between the anchor and the floating element, in pixels.
    pub offset: f64,
    /// The distance kept from the viewport's edges, in pixels.
    pub padding: f64,
    /// Move to the opposite side when the preferred one lacks room.
    pub flip: bool,
    /// Slide along the anchor's side to stay inside the viewport.
    pub shift: bool,
    /// How close an arrow may come to the floating element's corners.
    pub arrow_padding: f64,
}

impl Default for AnchorOptions {
    fn default() -> Self {
        Self { placement: Placement::default(), offset: 6.0, padding: 8.0, flip: true, shift: true, arrow_padding: 10.0 }
    }
}

impl AnchorOptions {
    pub fn new(placement: Placement) -> Self {
        Self { placement, ..Self::default() }
    }

    pub const fn offset(mut self, offset: f64) -> Self {
        self.offset = offset;
        self
    }

    pub const fn padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }

    pub const fn flip(mut self, flip: bool) -> Self {
        self.flip = flip;
        self
    }

    pub const fn shift(mut self, shift: bool) -> Self {
        self.shift = shift;
        self
    }

    pub const fn arrow_padding(mut self, arrow_padding: f64) -> Self {
        self.arrow_padding = arrow_padding;
        self
    }
}

/// Where [`place_anchored`] put a floating element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Anchored {
    /// The floating element's left edge, in viewport coordinates.
    pub x: f64,
    /// The floating element's top edge, in viewport coordinates.
    pub y: f64,
    /// The placement used, which is the opposite side when it flipped.
    pub placement: Placement,
    /// Where the anchor's centre falls along the floating element's facing
    /// edge, measured from that edge's left or top end: the arrow position.
    pub arrow: f64,
    /// Whether any part of the anchor is inside the viewport.
    pub anchor_visible: bool,
}

/// Places a floating element of `size` (width, height) beside `anchor`,
/// inside `viewport`. All three share one coordinate space.
pub fn place_anchored(anchor: AnchorRect, size: (f64, f64), viewport: AnchorRect, options: &AnchorOptions) -> Anchored {
    let (width, height) = size;
    let (mut side, along) = options.placement.parts();

    // Room on each side of the anchor for the floating element and its gap.
    let room = |side: Side| match side {
        Side::Top => anchor.y - viewport.y - options.padding - options.offset - height,
        Side::Bottom => viewport.bottom() - anchor.bottom() - options.padding - options.offset - height,
        Side::Left => anchor.x - viewport.x - options.padding - options.offset - width,
        Side::Right => viewport.right() - anchor.right() - options.padding - options.offset - width,
    };
    let mut placement = options.placement;
    if options.flip && room(side) < 0.0 {
        let (opposite, _) = placement.flipped().parts();
        if room(opposite) > room(side) {
            placement = placement.flipped();
            side = opposite;
        }
    }

    let vertical = placement.is_vertical();
    // Along the anchor's side: the anchor's and the floating element's extent.
    let (anchor_start, anchor_len, len, min, max) = if vertical {
        (anchor.x, anchor.width, width, viewport.x, viewport.right())
    } else {
        (anchor.y, anchor.height, height, viewport.y, viewport.bottom())
    };
    let mut cross = match along {
        Along::Start => anchor_start,
        Along::Center => anchor_start + (anchor_len - len) / 2.0,
        Along::End => anchor_start + anchor_len - len,
    };
    if options.shift {
        // When it cannot fit, keep its start on screen.
        cross = cross.min(max - options.padding - len).max(min + options.padding);
    }

    let (x, y) = match side {
        Side::Top => (cross, anchor.y - options.offset - height),
        Side::Bottom => (cross, anchor.bottom() + options.offset),
        Side::Left => (anchor.x - options.offset - width, cross),
        Side::Right => (anchor.right() + options.offset, cross),
    };

    let arrow_max = (len - options.arrow_padding).max(len / 2.0);
    let arrow_min = options.arrow_padding.min(len / 2.0);
    let arrow = (anchor_start + anchor_len / 2.0 - cross).clamp(arrow_min, arrow_max);

    let anchor_visible = anchor.right() > viewport.x
        && anchor.x < viewport.right()
        && anchor.bottom() > viewport.y
        && anchor.y < viewport.bottom();

    Anchored { x, y, placement, arrow, anchor_visible }
}

/// Keeps the element in `floating` placed beside the one in `anchor` while
/// `open` is true.
///
/// - Positions it after every render while open, so a change of content or
///   options is picked up, and again whenever anything scrolls or the window
///   resizes.
/// - If the floating element has a `popover` attribute it is shown in the top
///   layer, above dialogs and outside any clipping ancestor.
/// - It stays hidden until first placed, and while the anchor is scrolled out
///   of the viewport.
///
/// Render the floating element only while open, with the `yi-anchored` class.
/// Nothing happens during a server render.
///
/// The first placement after opening fires a `yi-placed` event on the
/// floating element; see [`when_placed`].
#[hook]
pub fn use_anchored(anchor: &NodeRef, floating: &NodeRef, open: bool, options: AnchorOptions) {
    {
        let (anchor, floating) = (anchor.clone(), floating.clone());
        use_effect(move || {
            if open {
                dom::update(&anchor, &floating, &options);
            }
        });
    }

    let (anchor, floating) = (anchor.clone(), floating.clone());
    use_effect_with((open, options), move |&(open, options)| {
        let following = open.then(|| dom::follow(anchor, floating, options));
        move || {
            let _stop = following;
        }
    });
}

/// Runs `ready` once the element in `within` can be seen and take focus.
///
/// - Inside an anchored element that [`use_anchored`] has not placed yet,
///   that is its first placement.
/// - Anywhere else, including an anchored element already placed, it is now.
///
/// Call it from the effect that would otherwise focus straight away, and
/// return the handle from that effect: dropping it cancels a wait that is
/// still pending.
///
/// ```ignore
/// use_effect_with((), move |_| {
///     let pending = when_placed(&node, move || focus_first_item(&node));
///     move || {
///         let _cancel = pending;
///     }
/// });
/// ```
#[must_use = "dropping the handle cancels the wait"]
pub fn when_placed(within: &NodeRef, ready: impl FnOnce() + 'static) -> WhenPlaced {
    WhenPlaced(dom::when_placed(within, ready))
}

/// A wait started by [`when_placed`], cancelled when dropped.
pub struct WhenPlaced(#[allow(dead_code)] Option<dom::Waiting>);

#[cfg(target_arch = "wasm32")]
mod dom {
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{AddEventListenerOptions, Element, Event, HtmlElement};
    use yew::NodeRef;

    use super::{AnchorOptions, AnchorRect, place_anchored};

    /// Fired on the floating element the first time it is placed.
    const PLACED: &str = "yi-placed";
    /// Set by every placement, so its absence means "not placed yet".
    const SIDE: &str = "data-yi-side";

    pub fn update(anchor: &NodeRef, floating: &NodeRef, options: &AnchorOptions) {
        let (Some(anchor), Some(floating)) = (anchor.cast::<Element>(), floating.cast::<HtmlElement>()) else {
            return;
        };
        let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) else {
            return;
        };

        if floating.has_attribute("popover") && !floating.matches(":popover-open").unwrap_or(true) {
            let _ = floating.show_popover();
        }

        let rect = anchor.get_bounding_client_rect();
        let placed = place_anchored(
            AnchorRect::new(rect.x(), rect.y(), rect.width(), rect.height()),
            // The layout size: unlike the bounding rect, an entry animation does not scale it.
            (f64::from(floating.offset_width()), f64::from(floating.offset_height())),
            AnchorRect::new(0.0, 0.0, f64::from(root.client_width()), f64::from(root.client_height())),
            options,
        );

        let style = floating.style();
        let _ = style.set_property("translate", &format!("{}px {}px", placed.x.round(), placed.y.round()));
        let _ = style.set_property("--yi-anchor-arrow", &format!("{}px", placed.arrow.round()));
        let _ = style.set_property("--yi-anchor-offset", &format!("{}px", options.offset));
        let first = !floating.has_attribute(SIDE);
        let _ = floating.set_attribute(SIDE, placed.placement.side());
        let _ = floating.set_attribute("data-yi-align", placed.placement.align());
        let _ = floating.toggle_attribute_with_force("data-yi-anchor-hidden", !placed.anchor_visible);
        if first && let Ok(event) = Event::new(PLACED) {
            let _ = floating.dispatch_event(&event);
        }
    }

    /// A listener for the first placement, removed when dropped.
    pub struct Waiting {
        host: Element,
        listener: Closure<dyn FnMut()>,
    }

    pub fn when_placed(within: &NodeRef, ready: impl FnOnce() + 'static) -> Option<Waiting> {
        let host = within.cast::<Element>().and_then(|element| element.closest(".yi-anchored").ok().flatten());
        let Some(host) = host.filter(|host| !host.has_attribute(SIDE)) else {
            ready();
            return None;
        };
        let listener = Closure::once(ready);
        let once = AddEventListenerOptions::new();
        once.set_once(true);
        let _ = host.add_event_listener_with_callback_and_add_event_listener_options(
            PLACED,
            listener.as_ref().unchecked_ref(),
            &once,
        );
        Some(Waiting { host, listener })
    }

    impl Drop for Waiting {
        fn drop(&mut self) {
            let _ = self.host.remove_event_listener_with_callback(PLACED, self.listener.as_ref().unchecked_ref());
        }
    }

    /// Repositions on scroll (of any ancestor) and resize until dropped.
    pub struct Following(Closure<dyn FnMut()>);

    pub fn follow(anchor: NodeRef, floating: NodeRef, options: AnchorOptions) -> Following {
        let closure = Closure::<dyn FnMut()>::new(move || update(&anchor, &floating, &options));
        if let Some(window) = web_sys::window() {
            let listener = closure.as_ref().unchecked_ref();
            let scroll = AddEventListenerOptions::new();
            scroll.set_capture(true);
            scroll.set_passive(true);
            let _ = window.add_event_listener_with_callback_and_add_event_listener_options("scroll", listener, &scroll);
            let _ = window.add_event_listener_with_callback("resize", listener);
        }
        Following(closure)
    }

    impl Drop for Following {
        fn drop(&mut self) {
            if let Some(window) = web_sys::window() {
                let listener = self.0.as_ref().unchecked_ref();
                let _ = window.remove_event_listener_with_callback_and_bool("scroll", listener, true);
                let _ = window.remove_event_listener_with_callback("resize", listener);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    //! Effects never run outside a browser.

    use yew::NodeRef;

    use super::AnchorOptions;

    pub struct Following;

    pub struct Waiting;

    pub fn when_placed(_: &NodeRef, _: impl FnOnce() + 'static) -> Option<Waiting> {
        None
    }

    pub fn update(_: &NodeRef, _: &NodeRef, _: &AnchorOptions) {}

    pub fn follow(_: NodeRef, _: NodeRef, _: AnchorOptions) -> Following {
        Following
    }
}

#[cfg(test)]
mod tests;
