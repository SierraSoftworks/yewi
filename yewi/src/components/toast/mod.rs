//! Toasts: the outcome of an action, said where it will be noticed and then
//! gone. Failures stay until dismissed, because a refusal is something to
//! read, not to catch.
//!
//! Wrap the app in a [`ToastHost`], then report outcomes from anywhere below
//! it through the [`Toaster`] that [`use_toast`] returns.

use std::{cell::Cell, rc::Rc};

use yew::prelude::*;

use crate::{Attrs, Button, ButtonVariant, Icon, IconData, Size, Tone, icons};

/// How long a toast stays when neither it nor its host says otherwise.
pub const DEFAULT_TOAST_DURATION_MS: u32 = 5_000;

/// How a toast shows its tone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ToastAccent {
    /// A tone-coloured icon before the text.
    #[default]
    Icon,
    /// A tone-coloured stripe along the leading edge, with no icon.
    Stripe,
}

impl ToastAccent {
    const fn as_str(self) -> &'static str {
        match self {
            ToastAccent::Icon => "icon",
            ToastAccent::Stripe => "stripe",
        }
    }
}

/// The corner or edge of the viewport where a [`ToastHost`] stacks toasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ToastPlacement {
    TopStart,
    TopCenter,
    TopEnd,
    BottomStart,
    BottomCenter,
    /// The bottom corner at the end of the reading direction.
    #[default]
    BottomEnd,
}

impl ToastPlacement {
    pub const ALL: [ToastPlacement; 6] = [
        ToastPlacement::TopStart,
        ToastPlacement::TopCenter,
        ToastPlacement::TopEnd,
        ToastPlacement::BottomStart,
        ToastPlacement::BottomCenter,
        ToastPlacement::BottomEnd,
    ];

    const fn as_str(self) -> &'static str {
        match self {
            ToastPlacement::TopStart => "top-start",
            ToastPlacement::TopCenter => "top-center",
            ToastPlacement::TopEnd => "top-end",
            ToastPlacement::BottomStart => "bottom-start",
            ToastPlacement::BottomCenter => "bottom-center",
            ToastPlacement::BottomEnd => "bottom-end",
        }
    }
}

/// How long a toast stays on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ToastDuration {
    /// The host's `duration_ms`, except that `Tone::Danger` and
    /// `Tone::Active` stay until dismissed.
    #[default]
    Auto,
    /// Dismissed after this many milliseconds.
    Millis(u32),
    /// Stays until dismissed by the user or by [`Toaster::dismiss`].
    Sticky,
}

impl ToastDuration {
    /// The timeout to run for a toast of `tone` under a host default.
    fn resolve(self, tone: Tone, host_default: u32) -> Option<u32> {
        match self {
            ToastDuration::Auto => (!matches!(tone, Tone::Danger | Tone::Active)).then_some(host_default),
            ToastDuration::Millis(millis) => Some(millis),
            ToastDuration::Sticky => None,
        }
    }
}

/// One follow-up a toast offers, such as "Undo" or "View".
#[derive(Debug, Clone, PartialEq)]
pub struct ToastAction {
    pub label: AttrValue,
    /// Fired on activation. The toast is dismissed afterwards.
    pub on_click: Callback<()>,
}

impl ToastAction {
    pub fn new(label: impl Into<AttrValue>, on_click: impl Into<Callback<()>>) -> Self {
        Self { label: label.into(), on_click: on_click.into() }
    }
}

/// Identifies a toast shown by a [`Toaster`], to dismiss it later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToastId(u64);

/// What a toast says. Build one and pass it to [`Toaster::show`].
///
/// ```ignore
/// toast.show(ToastMessage::new(Tone::Ok, "3 hosts archived.").action(ToastAction::new("Undo", undo)));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ToastMessage {
    tone: Tone,
    title: Option<AttrValue>,
    content: Html,
    action: Option<ToastAction>,
    duration: ToastDuration,
}

impl ToastMessage {
    pub fn new(tone: Tone, text: impl Into<AttrValue>) -> Self {
        let text: AttrValue = text.into();
        Self { tone, title: None, content: html! { {text} }, action: None, duration: ToastDuration::Auto }
    }

    /// A short heading above the text.
    pub fn title(mut self, title: impl Into<AttrValue>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Replaces the text with arbitrary content, for example to include a link.
    pub fn content(mut self, content: Html) -> Self {
        self.content = content;
        self
    }

    pub fn action(mut self, action: ToastAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn duration(mut self, duration: ToastDuration) -> Self {
        self.duration = duration;
        self
    }

    /// Keeps the toast until it is dismissed.
    pub fn sticky(self) -> Self {
        self.duration(ToastDuration::Sticky)
    }
}

const fn tone_icon(tone: Tone) -> Option<IconData> {
    match tone {
        Tone::Neutral => None,
        Tone::Ok => Some(icons::CIRCLE_CHECK),
        Tone::Warn => Some(icons::TRIANGLE_ALERT),
        Tone::Danger => Some(icons::CIRCLE_ALERT),
        Tone::Info => Some(icons::INFO),
        Tone::Active => Some(icons::LOADER_CIRCLE),
    }
}

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    /// The message.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub tone: Tone,
    /// A short heading above the message.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub accent: ToastAccent,
    /// A follow-up shown as a button. Activating it also fires `on_dismiss`.
    #[prop_or_default]
    pub action: Option<ToastAction>,
    /// Fired by the dismiss button and by Escape while focus is inside the
    /// toast. Without it the toast has no dismiss button.
    #[prop_or_default]
    pub on_dismiss: Option<Callback<()>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A single toast. A [`ToastHost`] renders these for you; use it directly
/// only to show a toast in the flow of a page.
///
/// It is a `status` live region, or an `alert` when the tone is `Danger`.
#[function_component]
pub fn Toast(props: &ToastProps) -> Html {
    let class = classes!(
        "yi-toast",
        format!("yi-toast--{}", props.tone.as_str()),
        format!("yi-toast--{}", props.accent.as_str()),
        props.class.clone(),
    );
    let icon = tone_icon(props.tone).filter(|_| props.accent == ToastAccent::Icon);

    let dismiss = props.on_dismiss.clone().unwrap_or_default();
    let onkeydown = props.on_dismiss.clone().map(|on_dismiss| {
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                event.stop_propagation();
                on_dismiss.emit(());
            }
        })
    });
    let action = props.action.clone().map(|action| {
        let dismiss = dismiss.clone();
        let on_click = Callback::from(move |_: MouseEvent| {
            action.on_click.emit(());
            dismiss.emit(());
        });
        html! {
            <Button class="yi-toast__action" size={Size::Small} {on_click}>{ action.label.clone() }</Button>
        }
    });

    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            {class}
            role={if props.tone == Tone::Danger { "alert" } else { "status" }}
            {onkeydown}
        >
            if let Some(icon) = icon {
                <Icon {icon} class="yi-toast__icon" />
            }
            <div class="yi-toast__body">
                if let Some(title) = &props.title {
                    <div class="yi-toast__title">{ title.clone() }</div>
                }
                <div class="yi-toast__message">{ props.children.clone() }</div>
            </div>
            { action }
            if props.on_dismiss.is_some() {
                <Button
                    class="yi-toast__dismiss yi-button--icon-only"
                    variant={ButtonVariant::Subtle}
                    size={Size::Small}
                    icon={icons::X}
                    attrs={crate::attrs! { "aria-label" => "Dismiss" }}
                    on_click={move |_| dismiss.emit(())}
                />
            }
        </div>
    })
}

#[derive(Clone, PartialEq)]
struct Entry {
    id: ToastId,
    message: Rc<ToastMessage>,
}

#[derive(Clone, PartialEq, Default)]
struct Stack {
    list: Vec<Entry>,
}

enum StackAction {
    Push { entry: Entry, max: usize },
    Dismiss(ToastId),
    Clear,
}

impl Reducible for Stack {
    type Action = StackAction;

    fn reduce(self: Rc<Self>, action: StackAction) -> Rc<Self> {
        let mut list = self.list.clone();
        match action {
            StackAction::Push { entry, max } => {
                list.push(entry);
                // The oldest make room for the newest.
                let excess = list.len().saturating_sub(max.max(1));
                list.drain(..excess);
            }
            StackAction::Dismiss(id) => list.retain(|entry| entry.id != id),
            StackAction::Clear => list.clear(),
        }
        Rc::new(Stack { list })
    }
}

/// The handle that shows and dismisses toasts. Get it with [`use_toast`].
///
/// It is cheap to clone and safe to move into callbacks and futures.
#[derive(Clone)]
pub struct Toaster {
    dispatch: UseReducerDispatcher<Stack>,
    next_id: Rc<Cell<u64>>,
    max: usize,
}

impl PartialEq for Toaster {
    fn eq(&self, other: &Self) -> bool {
        self.dispatch == other.dispatch && self.max == other.max
    }
}

impl Toaster {
    /// Shows a toast and returns its id, for [`dismiss`](Self::dismiss).
    pub fn show(&self, message: ToastMessage) -> ToastId {
        let id = ToastId(self.next_id.replace(self.next_id.get() + 1));
        self.dispatch.dispatch(StackAction::Push { entry: Entry { id, message: Rc::new(message) }, max: self.max });
        id
    }

    /// Something worked. Dismisses itself.
    pub fn ok(&self, text: impl Into<AttrValue>) -> ToastId {
        self.show(ToastMessage::new(Tone::Ok, text))
    }

    /// Something worth knowing. Dismisses itself.
    pub fn info(&self, text: impl Into<AttrValue>) -> ToastId {
        self.show(ToastMessage::new(Tone::Info, text))
    }

    /// Something worked, with a caveat. Dismisses itself.
    pub fn warn(&self, text: impl Into<AttrValue>) -> ToastId {
        self.show(ToastMessage::new(Tone::Warn, text))
    }

    /// Something failed. Stays until dismissed.
    pub fn error(&self, text: impl Into<AttrValue>) -> ToastId {
        self.show(ToastMessage::new(Tone::Danger, text))
    }

    /// Removes one toast. Does nothing if it has already gone.
    pub fn dismiss(&self, id: ToastId) {
        self.dispatch.dispatch(StackAction::Dismiss(id));
    }

    /// Removes every toast.
    pub fn clear(&self) {
        self.dispatch.dispatch(StackAction::Clear);
    }
}

/// The [`Toaster`] of the nearest [`ToastHost`].
///
/// # Panics
/// Panics when no `ToastHost` is an ancestor of the calling component.
#[hook]
pub fn use_toast() -> Toaster {
    use_context::<Toaster>().expect("use_toast() needs a <ToastHost> around the app")
}

#[derive(Properties, PartialEq)]
pub struct ToastHostProps {
    /// The app. Everything inside can call [`use_toast`].
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub placement: ToastPlacement,
    /// How many toasts show at once. The oldest make room for the newest.
    #[prop_or(4)]
    pub max: usize,
    /// How long a toast stays unless it sets its own [`ToastDuration`].
    #[prop_or(DEFAULT_TOAST_DURATION_MS)]
    pub duration_ms: u32,
    #[prop_or_default]
    pub accent: ToastAccent,
    /// The accessible name of the toast region.
    #[prop_or(AttrValue::Static("Notifications"))]
    pub label: AttrValue,
    /// Added to the toast region.
    #[prop_or_default]
    pub class: Classes,
    /// The id of the toast region.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Added to the toast region.
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Provides a [`Toaster`] to its children and stacks the toasts they show in
/// a corner of the viewport.
///
/// - Timers pause while the pointer or keyboard focus is on the stack.
/// - The region is rendered empty on the server, so it is announced reliably
///   once toasts arrive.
#[function_component]
pub fn ToastHost(props: &ToastHostProps) -> Html {
    let stack = use_reducer(Stack::default);
    let next_id = use_memo((), |_| Cell::new(0));
    let hovered = use_state(|| false);
    let focused = use_state(|| false);

    let toaster = Toaster { dispatch: stack.dispatcher(), next_id, max: props.max };

    let on_dismiss = {
        let (dispatch, focused) = (stack.dispatcher(), focused.clone());
        Callback::from(move |id: ToastId| {
            // The focused control leaves with its toast, and a removed
            // element does not report losing focus.
            focused.set(false);
            dispatch.dispatch(StackAction::Dismiss(id));
        })
    };

    let paused = *hovered || *focused;
    let region = props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!(
                "yi-toast-region",
                format!("yi-toast-region--{}", props.placement.as_str()),
                props.class.clone(),
            )}
            role="region"
            aria-label={props.label.clone()}
            aria-live="polite"
            onpointerenter={let hovered = hovered.clone(); move |_| hovered.set(true)}
            onpointerleave={let hovered = hovered.clone(); move |_| hovered.set(false)}
            onfocusin={let focused = focused.clone(); move |_| focused.set(true)}
            onfocusout={let focused = focused.clone(); move |_| focused.set(false)}
        >
            { for stack.list.iter().map(|entry| html! {
                <HostedToast
                    key={entry.id.0}
                    id={entry.id}
                    message={entry.message.clone()}
                    duration={entry.message.duration.resolve(entry.message.tone, props.duration_ms)}
                    accent={props.accent}
                    {paused}
                    on_dismiss={on_dismiss.clone()}
                />
            }) }
        </div>
    });

    html! {
        <ContextProvider<Toaster> context={toaster}>
            { props.children.clone() }
            { region }
        </ContextProvider<Toaster>>
    }
}

#[derive(Properties, PartialEq)]
struct HostedToastProps {
    id: ToastId,
    message: Rc<ToastMessage>,
    duration: Option<u32>,
    accent: ToastAccent,
    paused: bool,
    on_dismiss: Callback<ToastId>,
}

/// One toast in a host's stack, with its own dismissal timer.
#[function_component]
fn HostedToast(props: &HostedToastProps) -> Html {
    let dismiss = {
        let (id, on_dismiss) = (props.id, props.on_dismiss.clone());
        Callback::from(move |()| on_dismiss.emit(id))
    };

    // The timer restarts in full after a pause, so a toast the user was
    // reading never vanishes the moment they look away.
    {
        let dismiss = dismiss.clone();
        use_effect_with((props.duration, props.paused), move |&(duration, paused)| {
            let timeout =
                duration.filter(|_| !paused).and_then(|millis| timer::Timeout::start(millis, move || dismiss.emit(())));
            move || {
                let _cancelled = timeout;
            }
        });
    }

    let message = &props.message;
    html! {
        <Toast
            tone={message.tone}
            title={message.title.clone()}
            accent={props.accent}
            action={message.action.clone()}
            on_dismiss={dismiss}
        >
            { message.content.clone() }
        </Toast>
    }
}

#[cfg(target_arch = "wasm32")]
mod timer {
    use wasm_bindgen::{JsValue, closure::Closure, prelude::wasm_bindgen};

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
        pub fn start(millis: u32, callback: impl FnOnce() + 'static) -> Option<Self> {
            let mut callback = Some(callback);
            let closure = Closure::<dyn FnMut()>::new(move || {
                if let Some(callback) = callback.take() {
                    callback();
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
}

#[cfg(not(target_arch = "wasm32"))]
mod timer {
    /// Effects never run on the server, so there is nothing to time.
    pub struct Timeout;

    impl Timeout {
        pub fn start(_millis: u32, _callback: impl FnOnce() + 'static) -> Option<Self> {
            None
        }
    }
}

#[cfg(test)]
mod tests;
