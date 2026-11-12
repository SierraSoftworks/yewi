use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let loading = use_state(|| true);
    let toggle = {
        let loading = loading.clone();
        Callback::from(move |_| loading.set(!*loading))
    };

    html! {
        <Stack align={Align::Start}>
            <section aria-label="Rollouts" style="inline-size: min(100%, 26rem); min-block-size: 9rem; display: grid; align-items: center; border: 1px solid var(--yi-line); border-radius: var(--yi-radius-card); background: var(--yi-surface)">
                if *loading {
                    <Loading layout={LoadingLayout::Block} label="Loading rollouts" delay=true />
                } else {
                    <p style="margin: 0; padding: var(--yi-space-lg)">{ "2 of 31 hosts done, started 35 minutes ago." }</p>
                }
            </section>
            <Button on_click={toggle}>{ if *loading { "Finish loading" } else { "Load again" } }</Button>
        </Stack>
    }
}
