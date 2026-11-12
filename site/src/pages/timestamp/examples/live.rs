use yew::prelude::*;
use yewi::chrono::{DateTime, Utc};
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let recorded = use_state(|| None::<Now>);
    let record = {
        let recorded = recorded.clone();
        Callback::from(move |_| recorded.set(Some(Now::system())))
    };

    html! {
        <Cluster>
            <Button on_click={record}>{ "Record an event" }</Button>
            <span role="status">
                if let Some(now) = *recorded {
                    // This site stops the clock for its examples; a provider
                    // starts it again here. An app needs none of this.
                    <NowProvider {now}>
                        { "Recorded " }
                        <Timestamp at={DateTime::<Utc>::from(now)} />
                    </NowProvider>
                } else {
                    { "Nothing recorded yet" }
                }
            </span>
        </Cluster>
    }
}
