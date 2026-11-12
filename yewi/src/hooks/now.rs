use yew::prelude::*;

/// A point in time: milliseconds since the Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Now(f64);

impl Now {
    pub const fn from_epoch_millis(millis: f64) -> Self {
        Self(millis)
    }

    pub const fn epoch_millis(self) -> f64 {
        self.0
    }

    /// The system clock, right now. Prefer [`use_now`] inside components.
    pub fn system() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self(js_sys::Date::now())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let elapsed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            Self(elapsed.as_secs_f64() * 1000.0)
        }
    }
}

/// How often a [`use_now`] caller wants a fresh value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tick {
    /// Read the clock once and never re-render for it.
    Never,
    /// Re-render when the wall-clock second changes.
    Second,
    /// Re-render when the wall-clock minute changes.
    Minute,
}

impl Tick {
    fn millis(self) -> Option<f64> {
        match self {
            Tick::Never => None,
            Tick::Second => Some(1_000.0),
            Tick::Minute => Some(60_000.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct NowContext {
    initial: Now,
    live: bool,
}

#[derive(Properties, PartialEq)]
pub struct NowProviderProps {
    /// The time every descendant sees on its first render. A server passes
    /// the request time and sends the same value to the client, so the
    /// hydrating render matches.
    pub now: Now,
    /// Whether descendants move on to the live clock once mounted. Turn this
    /// off to freeze time, for example in tests and screenshots.
    #[prop_or(true)]
    pub live: bool,
    #[prop_or_default]
    pub children: Html,
}

/// Injects the time that [`use_now`] reports to descendants.
#[function_component]
pub fn NowProvider(props: &NowProviderProps) -> Html {
    let context = NowContext { initial: props.now, live: props.live };
    html! {
        <ContextProvider<NowContext> {context}>{ props.children.clone() }</ContextProvider<NowContext>>
    }
}

/// The current time, re-rendering the caller as often as `tick` asks.
///
/// - The first render reports the [`NowProvider`]'s time when there is one,
///   so server and client agree; otherwise it reads the system clock.
/// - After mounting, every caller is driven by one shared timer, and callers
///   with the same `tick` update together on the wall-clock boundary.
#[hook]
pub fn use_now(tick: Tick) -> Now {
    let context = use_context::<NowContext>();
    let now = use_state(|| context.map_or_else(Now::system, |c| c.initial));
    let live = context.is_none_or(|c| c.live);

    {
        let now = now.clone();
        use_effect_with((tick, live), move |&(tick, live)| {
            let subscription = live.then(|| {
                now.set(Now::system());
                tick.millis().map(|period| ticker::subscribe(period, Callback::from(move |t| now.set(t))))
            });
            move || {
                let _unsubscribe = subscription;
            }
        });
    }

    *now
}

#[cfg(target_arch = "wasm32")]
mod ticker {
    //! One `setInterval` for every `use_now` caller on the page.

    use std::cell::RefCell;

    use wasm_bindgen::{JsCast, closure::Closure};
    use yew::Callback;

    use super::Now;

    struct Subscriber {
        id: u32,
        period: f64,
        bucket: f64,
        callback: Callback<Now>,
    }

    #[derive(Default)]
    struct Ticker {
        next_id: u32,
        subscribers: Vec<Subscriber>,
        interval: Option<(i32, Closure<dyn FnMut()>)>,
    }

    thread_local! {
        static TICKER: RefCell<Ticker> = RefCell::default();
    }

    pub struct Subscription(u32);

    pub fn subscribe(period: f64, callback: Callback<Now>) -> Subscription {
        TICKER.with_borrow_mut(|ticker| {
            let id = ticker.next_id;
            ticker.next_id += 1;
            let bucket = (Now::system().epoch_millis() / period).floor();
            ticker.subscribers.push(Subscriber { id, period, bucket, callback });

            if ticker.interval.is_none()
                && let Some(window) = web_sys::window()
            {
                let closure = Closure::<dyn FnMut()>::new(tick);
                if let Ok(handle) = window
                    .set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 1_000)
                {
                    ticker.interval = Some((handle, closure));
                }
            }
            Subscription(id)
        })
    }

    fn tick() {
        let now = Now::system();
        // Collect first: a callback may subscribe or unsubscribe.
        let due: Vec<Callback<Now>> = TICKER.with_borrow_mut(|ticker| {
            ticker
                .subscribers
                .iter_mut()
                .filter_map(|s| {
                    let bucket = (now.epoch_millis() / s.period).floor();
                    (bucket != s.bucket).then(|| {
                        s.bucket = bucket;
                        s.callback.clone()
                    })
                })
                .collect()
        });
        for callback in due {
            callback.emit(now);
        }
    }

    impl Drop for Subscription {
        fn drop(&mut self) {
            TICKER.with_borrow_mut(|ticker| {
                ticker.subscribers.retain(|s| s.id != self.0);
                if ticker.subscribers.is_empty()
                    && let Some((handle, _closure)) = ticker.interval.take()
                    && let Some(window) = web_sys::window()
                {
                    window.clear_interval_with_handle(handle);
                }
            });
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod ticker {
    use yew::Callback;

    use super::Now;

    pub struct Subscription;

    pub fn subscribe(_period: f64, _callback: Callback<Now>) -> Subscription {
        Subscription
    }
}
