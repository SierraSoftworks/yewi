//! The browser side of [`SecretInput`](super::SecretInput). Everything here
//! is a no-op off the web.
//!
//! `crypto.getRandomValues` is reached through reflection rather than
//! `web-sys` bindings, which keeps the crate's `web-sys` feature list (a
//! shared file) unchanged.

use yew::NodeRef;

/// `count` bytes from the browser's cryptographic generator, or `None` where
/// there is not one.
#[cfg(target_arch = "wasm32")]
pub fn random_bytes(count: usize) -> Option<Vec<u8>> {
    use js_sys::{Function, Reflect, Uint8Array};
    use wasm_bindgen::JsCast;

    let crypto = Reflect::get(&web_sys::window()?.into(), &"crypto".into()).ok()?;
    let fill: Function = Reflect::get(&crypto, &"getRandomValues".into()).ok()?.dyn_into().ok()?;
    let buffer = Uint8Array::new_with_length(u32::try_from(count).ok()?);
    fill.call1(&crypto, &buffer).ok()?;
    Some(buffer.to_vec())
}

/// Writes `value` into the `<input>` behind `node`, for an input whose text
/// the browser owns.
#[cfg(target_arch = "wasm32")]
pub fn set_value(node: &NodeRef, value: &str) {
    if let Some(input) = node.get() {
        let _ = js_sys::Reflect::set(&input, &"value".into(), &value.into());
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_bytes(_count: usize) -> Option<Vec<u8>> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_value(_node: &NodeRef, _value: &str) {}
