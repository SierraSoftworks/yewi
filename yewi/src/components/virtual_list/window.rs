//! The windowing behind [`VirtualList`](super::VirtualList) and
//! [`VirtualTable`](super::VirtualTable): which rows to draw for a scroll
//! position.

use std::ops::Range;

use yew::prelude::*;

/// What [`use_virtual_window`] needs to know about the rows.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualOptions {
    /// How many rows there are in all.
    pub count: usize,
    /// The height of one row in CSS pixels. Rows are assumed to share one
    /// height; if the first drawn row measures differently, the measurement
    /// is used.
    pub row_height: f64,
    /// Extra rows drawn above and below the visible ones, so a fast scroll
    /// lands on rows that already exist.
    pub overscan: usize,
    /// How many rows to assume are visible before the browser has been
    /// asked: on the server, and on the first client render.
    pub initial_rows: usize,
    /// A selector for the element that scrolls: the anchor or its closest
    /// ancestor matching it.
    pub scroller: &'static str,
    /// A selector for a drawn row inside the anchor, to measure.
    pub row: &'static str,
}

/// The rows to draw, and what the rest are worth in space.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualWindow {
    /// The rows to draw, as positions in display order.
    pub range: Range<usize>,
    /// The height of one row in CSS pixels, as measured: multiply by the
    /// number of rows left out to size what stands in for them.
    pub row_height: f64,
    /// The height asked for in [`VirtualOptions`]: the least a row may take.
    pub min_row_height: f64,
    /// Attach to the element holding the rows.
    pub anchor: NodeRef,
}

/// What the browser last reported, rounded so that scrolling within a few
/// rows changes nothing and so costs no render.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Seen {
    first: usize,
    visible: usize,
    row_height: Option<f64>,
}

/// Rows scrolled before the window moves. Even, so a window always starts on
/// an even row and striped rows keep their colour.
fn step(overscan: usize) -> usize {
    (overscan / 2).max(2).next_multiple_of(2)
}

fn seen(first: usize, visible: usize, overscan: usize) -> (usize, usize) {
    let step = step(overscan);
    (first - first % step, visible.max(1).next_multiple_of(step))
}

/// The rows to draw when `first` (a multiple of the step) is the earliest row
/// that may be at the top of the view and `visible` rows fit in it.
fn range(first: usize, visible: usize, count: usize, overscan: usize) -> Range<usize> {
    let overscan = overscan.next_multiple_of(2);
    let end = (first + step(overscan) + visible + overscan).min(count);
    first.saturating_sub(overscan).min(end)..end
}

/// The rows to draw for a scroll offset and view height in pixels: the
/// visible rows plus `overscan` either side, moving a few rows at a time.
pub fn virtual_range(scroll_top: f64, view_height: f64, row_height: f64, count: usize, overscan: usize) -> Range<usize> {
    let row_height = row_height.max(1.0);
    let first = (scroll_top.max(0.0) / row_height) as usize;
    let visible = (view_height.max(0.0) / row_height).ceil() as usize + 1;
    let (first, visible) = seen(first.min(count), visible, overscan);
    range(first, visible, count, overscan)
}

/// Tracks a scrolling element and returns the rows worth drawing. The work
/// per scroll event is three layout reads; a render happens only when the
/// window moves, every few rows.
///
/// Attach the returned `anchor` to the element that holds the rows, draw
/// `range`, and reserve `row_height` times the number of rows left out
/// above and below. The server, and the first client render, draw the first
/// `initial_rows` rows.
#[hook]
pub fn use_virtual_window(options: VirtualOptions) -> VirtualWindow {
    let anchor = use_node_ref();
    let state = use_state_eq(|| {
        let (first, visible) = seen(0, options.initial_rows, options.overscan);
        Seen { first, visible, row_height: None }
    });

    #[cfg(target_arch = "wasm32")]
    {
        let latest = use_mut_ref(|| options.clone());
        *latest.borrow_mut() = options.clone();
        let measure = {
            let (anchor, latest, set) = (anchor.clone(), latest.clone(), state.setter());
            std::rc::Rc::new(move || {
                if let Some(seen) = dom::measure(&anchor, &latest.borrow()) {
                    set.set(seen);
                }
            })
        };
        // Scrolling and resizing after mount; the rows themselves (their
        // number, their height) after every render.
        {
            let (anchor, latest, measure) = (anchor.clone(), latest.clone(), measure.clone());
            use_effect_with((), move |()| {
                let watch = dom::Watch::new(&anchor, latest.borrow().scroller, measure);
                move || drop(watch)
            });
        }
        use_effect(move || {
            measure();
            || ()
        });
    }

    VirtualWindow {
        range: range(state.first.min(options.count), state.visible, options.count, options.overscan),
        row_height: state.row_height.unwrap_or(options.row_height),
        min_row_height: options.row_height,
        anchor,
    }
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use std::rc::Rc;

    use js_sys::{Array, Function, Reflect};
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};
    use web_sys::Element;
    use yew::NodeRef;

    use super::{Seen, VirtualOptions, seen};

    pub fn measure(anchor: &NodeRef, options: &VirtualOptions) -> Option<Seen> {
        let anchor = anchor.cast::<Element>()?;
        let scroller = anchor.closest(options.scroller).ok()??;
        let view = f64::from(scroller.client_height());
        if view <= 0.0 {
            return None; // not laid out: hidden, or detached
        }
        let measured = anchor
            .query_selector(options.row)
            .ok()?
            .map(|row| row.get_bounding_client_rect().height())
            .filter(|height| *height >= 1.0);
        let row_height = measured.unwrap_or(options.row_height).max(1.0);
        let first = (f64::from(scroller.scroll_top()).max(0.0) / row_height) as usize;
        let visible = (view / row_height).ceil() as usize + 1;
        let (first, visible) = seen(first.min(options.count), visible, options.overscan);
        // Kept only when it differs, so the usual case renders exactly what
        // the server did.
        let row_height = measured.filter(|height| (height - options.row_height).abs() > 1.0 / 128.0);
        Some(Seen { first, visible, row_height })
    }

    /// Calls back when the scroller scrolls or changes size, until dropped.
    pub struct Watch {
        scroller: Option<Element>,
        observer: Option<JsValue>,
        callback: Closure<dyn FnMut()>,
    }

    impl Watch {
        pub fn new(anchor: &NodeRef, scroller: &str, measure: Rc<dyn Fn()>) -> Self {
            let callback = Closure::<dyn FnMut()>::new(move || measure());
            let scroller = anchor.cast::<Element>().and_then(|anchor| anchor.closest(scroller).ok().flatten());
            let observer = scroller.as_ref().and_then(|scroller| {
                let _ = scroller.add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref());
                // `ResizeObserver` by reflection: no extra `web-sys` feature.
                let constructor: Function =
                    Reflect::get(&js_sys::global(), &"ResizeObserver".into()).ok()?.dyn_into().ok()?;
                let observer = Reflect::construct(&constructor, &Array::of1(callback.as_ref())).ok()?;
                call(&observer, "observe", Some(scroller));
                Some(observer)
            });
            Self { scroller, observer, callback }
        }
    }

    impl Drop for Watch {
        fn drop(&mut self) {
            if let Some(scroller) = &self.scroller {
                let _ = scroller.remove_event_listener_with_callback("scroll", self.callback.as_ref().unchecked_ref());
            }
            if let Some(observer) = &self.observer {
                call(observer, "disconnect", None);
            }
        }
    }

    fn call(target: &JsValue, method: &str, argument: Option<&Element>) {
        let Some(function) = Reflect::get(target, &method.into()).ok().and_then(|f| f.dyn_into::<Function>().ok())
        else {
            return;
        };
        let _ = match argument {
            Some(argument) => function.call1(target, argument),
            None => function.call0(target),
        };
    }
}
