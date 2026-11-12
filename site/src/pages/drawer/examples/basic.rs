use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let outcome = use_state(|| "pi-1 is in service");

    let show = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };
    let close = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };
    let drain = {
        let (open, outcome) = (open.clone(), outcome.clone());
        Callback::from(move |_| {
            outcome.set("pi-1 is draining");
            open.set(false);
        })
    };

    let footer = html! {
        <>
            <Button on_click={close.reform(|_| ())}>{ "Close" }</Button>
            <Button variant={ButtonVariant::Primary} on_click={drain}>{ "Drain host" }</Button>
        </>
    };

    html! {
        <Stack>
            <Cluster>
                <Button on_click={show}>{ "Show host" }</Button>
            </Cluster>
            <span role="status">{ *outcome }</span>
            <Drawer open={*open} title="Host pi-1" on_close={close} {footer}>
                <p>{ "Raspberry Pi 5 in rack 2, running agent 1.8.2. It last reported 12 seconds ago." }</p>
                <p>{ "It serves 3 workloads: web, metrics and the nightly backup. Draining moves web and metrics to pi-2 and waits for the backup to finish." }</p>
            </Drawer>
        </Stack>
    }
}
