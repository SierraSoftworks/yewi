use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let outcome = use_state(|| "Rollout web-42 is paused");

    let show = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };
    let close = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };
    let resume = {
        let (open, outcome) = (open.clone(), outcome.clone());
        Callback::from(move |_| {
            outcome.set("Rollout web-42 resumed");
            open.set(false);
        })
    };

    let footer = html! {
        <>
            <Button on_click={close.reform(|_| ())}>{ "Not now" }</Button>
            <Button variant={ButtonVariant::Primary} on_click={resume}>{ "Resume rollout" }</Button>
        </>
    };

    html! {
        <Stack>
            <Cluster>
                <Button on_click={show}>{ "Review rollout" }</Button>
            </Cluster>
            <span role="status">{ *outcome }</span>
            <Dialog open={*open} title="Resume rollout web-42?" on_close={close} {footer}>
                <p>{ "The rollout paused after 4 of 12 hosts because one health check failed. That check has passed for the last 10 minutes." }</p>
                <p>{ "Resuming continues with the remaining 8 hosts, two at a time." }</p>
            </Dialog>
        </Stack>
    }
}
