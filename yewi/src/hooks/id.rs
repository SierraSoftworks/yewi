use std::cell::Cell;

use yew::prelude::*;

thread_local! {
    static NEXT_ID: Cell<u32> = const { Cell::new(0) };
}

/// A document-unique element id that is stable for the component's lifetime,
/// for wiring `for`, `aria-controls`, `aria-describedby` and anchors.
///
/// Ids are allocated in render order. A server render and the client render
/// that hydrates it normally agree; if they do not, hydration rewrites the
/// attributes, so references between elements always stay consistent.
#[hook]
pub fn use_id() -> AttrValue {
    (*use_memo((), |_| {
        let id = NEXT_ID.with(|next| next.replace(next.get() + 1));
        AttrValue::from(format!("yi-{id}"))
    }))
    .clone()
}

/// Restarts id allocation. Call before each independent server render (a
/// request, a prerendered page, a snapshot test) so its output is reproducible.
pub fn reset_ids() {
    NEXT_ID.with(|next| next.set(0));
}
