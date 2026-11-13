//! Hints: the tooltip of a control that shows no text, such as an icon-only
//! button. It repeats the control's name for people who can see it.

use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use super::{Handlers, dom};
use crate::{AnchorOptions, Placement, use_anchored};

/// Milliseconds the pointer rests before a hint shows.
const DELAY_MS: u32 = 400;

/// Shows `text` in a tooltip while the element behind `anchor` is hovered or
/// has keyboard focus. Render the returned bubble inside that element.
///
/// Unlike [`Tooltip`](super::Tooltip) it adds no wrapper, so the element
/// keeps its place in its parent's layout and selectors.
///
/// - The element must already carry the text as its accessible name; the
///   bubble is hidden from assistive technology so it is not read twice.
/// - Keep a native `title` on the element as well: it is the hint on a page
///   without a client runtime, and is removed once this one takes over.
#[hook]
pub(crate) fn use_hint(anchor: &NodeRef, text: &AttrValue) -> Html {
    let bubble = use_node_ref();
    let shown = use_state(|| false);

    let handlers = {
        let (anchor, bubble, shown) = (anchor.clone(), bubble.clone(), shown.setter());
        use_memo((), move |()| Handlers::new(anchor, bubble, shown, DELAY_MS))
    };

    let open = *shown && !text.is_empty();
    use_anchored(anchor, &bubble, open, AnchorOptions::new(Placement::Top).offset(9.0));

    {
        let (anchor, dismiss) = (anchor.clone(), handlers.dismiss.clone());
        use_effect_with(open, move |&open| {
            let session = open.then(|| dom::Session::start(&anchor, None, dismiss));
            move || {
                let _end = session;
            }
        });
    }

    {
        let wiring = use_mut_ref(|| None::<dom::Wiring>);
        let anchor = anchor.clone();
        use_effect(move || wire(&wiring, &anchor, &handlers));
    }

    html! {
        if open {
            <span
                ref={bubble}
                class="yi-anchored yi-tooltip__bubble yi-tooltip__bubble--surface yi-tooltip__bubble--arrow yi-tooltip__bubble--hint"
                aria-hidden="true"
                popover="manual"
            >
                { text.clone() }
            </span>
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn wire(wiring: &Rc<RefCell<Option<dom::Wiring>>>, anchor: &NodeRef, handlers: &Handlers) {
    dom::Wiring::sync(wiring, anchor, handlers);
}

#[cfg(not(target_arch = "wasm32"))]
fn wire(_: &Rc<RefCell<Option<dom::Wiring>>>, _: &NodeRef, _: &Handlers) {}
