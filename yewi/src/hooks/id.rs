use std::cell::Cell;

use yew::prelude::*;

thread_local! {
    static NEXT_ID: Cell<u32> = const { Cell::new(0) };
}

/// A document-unique element id that is stable for the component's lifetime,
/// for wiring `for`, `aria-controls`, `aria-describedby` and anchors.
///
/// Ids are allocated in render order, and Yew renders depth-first on the
/// server but level by level on the client, so a hydrated page can number its
/// components differently from the server render. Hydration rewrites every
/// attribute built from the id in the same pass, so `for`, `aria-*` and anchor
/// references stay consistent; only the number may change. Where the value
/// itself must be stable (a fragment link, a test hook), pass an explicit `id`
/// to the component instead.
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
