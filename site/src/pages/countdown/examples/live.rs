use yew::prelude::*;
use yewi::chrono::{DateTime, TimeDelta, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let started = use_state(|| None::<Now>);
    let finished = use_state(|| false);

    let start = {
        let (started, finished) = (started.clone(), finished.clone());
        Callback::from(move |_| {
            finished.set(false);
            started.set(Some(Now::system()));
        })
    };
    let on_done = {
        let finished = finished.clone();
        Callback::from(move |()| finished.set(true))
    };

    html! {
        <Cluster>
            <Button on_click={start}>{ "Start a 10 second bake" }</Button>
            if let Some(now) = *started {
                // This site stops the clock for its examples; a provider
                // starts it again here. An app needs none of this.
                <NowProvider {now}>
                    <Countdown
                        until={DateTime::<Utc>::from(now) + TimeDelta::seconds(10)}
                        focusable=true
                        {on_done}
                    />
                </NowProvider>
            }
            <span role="status">
                if *finished {
                    { "Bake finished" }
                } else if started.is_some() {
                    { "Baking" }
                } else {
                    { "Not started" }
                }
            </span>
        </Cluster>
    }
}
