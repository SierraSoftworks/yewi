use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let refreshing = use_state(|| true);
    let restarts = use_state(|| 0_u32);

    let set = |value: bool| {
        let refreshing = refreshing.clone();
        Callback::from(move |_| refreshing.set(value))
    };
    let restart = {
        let restarts = restarts.clone();
        Callback::from(move |_| restarts.set(*restarts + 1))
    };

    html! {
        <Stack align={Align::Start}>
            <LoadingOverlay active={*refreshing} label="Refreshing hosts">
                <section aria-label="edge-ams-01" style="inline-size: 20rem; padding: var(--yi-space-lg); border: 1px solid var(--yi-line); border-radius: var(--yi-radius-card); background: var(--yi-surface)">
                    <Stack gap={Space::Sm} align={Align::Start}>
                        <b>{ "edge-ams-01" }</b>
                        <span style="color: var(--yi-fg-muted)">{ "Healthy, 14 workloads, agent 2.8.1" }</span>
                        <Button size={Size::Small} on_click={restart}>{ "Restart agent" }</Button>
                    </Stack>
                </section>
            </LoadingOverlay>
            <Cluster>
                <Button variant={ButtonVariant::Primary} disabled={*refreshing} on_click={set(true)}>{ "Refresh" }</Button>
                <Button disabled={!*refreshing} on_click={set(false)}>{ "Finish refreshing" }</Button>
            </Cluster>
            <span role="status">{ format!("Restarted {} times", *restarts) }</span>
        </Stack>
    }
}
