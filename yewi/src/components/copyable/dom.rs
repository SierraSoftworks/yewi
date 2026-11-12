//! The browser side of copying. Everything here is a no-op off the web.
//!
//! `navigator.clipboard` and `execCommand` are reached through reflection
//! rather than `web-sys` bindings, which keeps the crate's `web-sys` feature
//! list (a shared file) unchanged.

#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(not(target_arch = "wasm32"))]
pub use server::*;

#[cfg(target_arch = "wasm32")]
mod web {
    use std::cell::RefCell;

    use js_sys::{Function, Promise, Reflect};
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};
    use web_sys::{Element, HtmlElement};
    use yew::NodeRef;

    /// Writes `text` to the clipboard and reports whether it got there.
    pub fn write(text: &str, done: impl FnOnce(bool) + 'static) {
        let Some(promise) = write_async(text) else {
            return done(write_legacy(text));
        };

        // Exactly one of the two handlers runs, so they share the callback.
        let done = std::rc::Rc::new(RefCell::new(Some(done)));
        let resolved = {
            let done = done.clone();
            Closure::once_into_js(move |_: JsValue| {
                if let Some(done) = done.borrow_mut().take() {
                    done(true);
                }
            })
        };
        let rejected = {
            let text = text.to_owned();
            Closure::once_into_js(move |_: JsValue| {
                if let Some(done) = done.borrow_mut().take() {
                    done(write_legacy(&text));
                }
            })
        };
        call(&promise, "then", &[resolved, rejected]);
    }

    /// `navigator.clipboard.writeText`, where the browser offers it: a secure
    /// context with permission to write.
    fn write_async(text: &str) -> Option<JsValue> {
        let navigator = Reflect::get(&web_sys::window()?.into(), &"navigator".into()).ok()?;
        let clipboard = Reflect::get(&navigator, &"clipboard".into()).ok()?;
        call(&clipboard, "writeText", &[text.into()]).filter(|promise| promise.has_type::<Promise>())
    }

    /// `document.execCommand("copy")` on a scratch `<textarea>`: deprecated,
    /// but the only route on a page served over plain HTTP.
    fn write_legacy(text: &str) -> bool {
        let copy = || {
            let document = web_sys::window()?.document()?;
            let focused = document.active_element();

            let scratch = document.create_element("textarea").ok()?;
            scratch.set_class_name("yi-copyable-scratch");
            scratch.set_attribute("readonly", "").ok()?;
            scratch.set_attribute("aria-hidden", "true").ok()?;
            scratch.set_text_content(Some(text));
            document.body()?.append_child(&scratch).ok()?;

            call(&scratch, "select", &[]);
            let copied = call(&document, "execCommand", &["copy".into()]).and_then(|ok| ok.as_bool());
            scratch.remove();

            if let Some(focused) = focused.and_then(|element| element.dyn_into::<HtmlElement>().ok()) {
                let _ = focused.focus();
            }
            copied
        };
        copy().unwrap_or(false)
    }

    /// Selects the contents of `node`, ready for the user to copy by hand.
    pub fn select(node: &NodeRef) {
        let select = || {
            let selection = call(&web_sys::window()?.into(), "getSelection", &[])?;
            call(&selection, "selectAllChildren", &[node.get()?.into()])
        };
        select();
    }

    thread_local! {
        static ANNOUNCER: RefCell<Option<Element>> = const { RefCell::new(None) };
    }

    /// Says `message` to assistive technology through one polite live region
    /// shared by the page. An empty message clears it, so that the same
    /// outcome is announced again next time.
    pub fn announce(message: &str) {
        ANNOUNCER.with_borrow_mut(|announcer| {
            if announcer.as_ref().is_none_or(|region| !region.is_connected()) {
                if message.is_empty() {
                    return;
                }
                *announcer = create_announcer();
            }
            if let Some(region) = announcer {
                region.set_text_content(Some(message));
            }
        });
    }

    fn create_announcer() -> Option<Element> {
        let document = web_sys::window()?.document()?;
        let region = document.create_element("div").ok()?;
        region.set_class_name("yi-visually-hidden");
        region.set_attribute("role", "status").ok()?;
        region.set_attribute("data-yi-announcer", "").ok()?;
        document.body()?.append_child(&region).ok()?;
        Some(region)
    }

    /// Calls `target[method](...args)`, `None` if it is missing or throws.
    fn call(target: &JsValue, method: &str, args: &[JsValue]) -> Option<JsValue> {
        let function: Function = Reflect::get(target, &method.into()).ok()?.dyn_into().ok()?;
        function.apply(target, &args.iter().collect()).ok()
    }

    /// A pending `setTimeout`, cancelled when dropped.
    pub struct Timer {
        handle: i32,
        _closure: Closure<dyn FnMut()>,
    }

    impl Timer {
        pub fn after(millis: i32, run: impl FnOnce() + 'static) -> Option<Self> {
            let closure = Closure::once(run);
            let handle = web_sys::window()?
                .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), millis)
                .ok()?;
            Some(Self { handle, _closure: closure })
        }
    }

    impl Drop for Timer {
        fn drop(&mut self) {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(self.handle);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod server {
    use yew::NodeRef;

    pub fn write(_text: &str, _done: impl FnOnce(bool) + 'static) {}

    pub fn select(_node: &NodeRef) {}

    pub fn announce(_message: &str) {}

    pub struct Timer;

    impl Timer {
        pub fn after(_millis: i32, _run: impl FnOnce() + 'static) -> Option<Self> {
            None
        }
    }
}
