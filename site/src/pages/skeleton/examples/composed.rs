use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let loaded = use_state(|| false);
    let toggle = {
        let loaded = loaded.clone();
        Callback::from(move |_| loaded.set(!*loaded))
    };
    let row = "display: flex; align-items: center; gap: var(--yi-space-md)";
    let avatar = "display: grid; place-items: center; inline-size: 2.5rem; block-size: 2.5rem; border-radius: var(--yi-radius-full); background: var(--yi-accent-soft); color: var(--yi-accent-text); font-weight: 600";

    html! {
        <Stack align={Align::Start}>
            <section
                aria-label="Latest deploy"
                aria-busy={(!*loaded).then_some("true")}
                style="inline-size: min(100%, 24rem); padding: var(--yi-space-lg); border: 1px solid var(--yi-line); border-radius: var(--yi-radius-card); background: var(--yi-surface)"
            >
                if *loaded {
                    <div style={row}>
                        <span style={avatar} aria-hidden="true">{ "AK" }</span>
                        <Stack gap={Space::None}>
                            <b>{ "Asha Kader" }</b>
                            <span style="color: var(--yi-fg-muted)">{ "Deployed api 4.12.0 to production" }</span>
                        </Stack>
                    </div>
                } else {
                    <VisuallyHidden attrs={attrs! { "role" => "status" }}>{ "Loading the latest deploy" }</VisuallyHidden>
                    <div style={row}>
                        <Skeleton shape={SkeletonShape::Circle} width="2.5rem" />
                        <div style="flex: 1; min-inline-size: 0">
                            <Skeleton width="6rem" />
                            <Skeleton width="15rem" />
                        </div>
                    </div>
                }
            </section>
            <Button on_click={toggle}>{ if *loaded { "Load again" } else { "Finish loading" } }</Button>
        </Stack>
    }
}
