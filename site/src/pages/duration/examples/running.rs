use yew::prelude::*;
use yewi::chrono::{DateTime, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let started = use_state(|| None::<Now>);
    let start = {
        let started = started.clone();
        Callback::from(move |_| started.set(Some(Now::system())))
    };

    html! {
        <Cluster>
            <Button on_click={start}>{ "Start a run" }</Button>
            <span role="status">
                if let Some(now) = *started {
                    // This site stops the clock for its examples; a provider
                    // starts it again here. An app needs none of this.
                    <NowProvider {now}>
                        <Pill tone={Tone::Active} active=true>{ "running" }</Pill>
                        { " for " }
                        <Duration since={DateTime::<Utc>::from(now)} />
                    </NowProvider>
                } else {
                    { "Not started" }
                }
            </span>
        </Cluster>
    }
}
