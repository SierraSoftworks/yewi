use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let attempts = use_state(|| 0_u32);
    let outcome = use_state(|| "Rollout web-42 is running");

    let show = {
        let (open, attempts) = (open.clone(), attempts.clone());
        Callback::from(move |_| {
            attempts.set(0);
            open.set(true);
        })
    };
    let on_cancel = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };
    // The first attempt fails, as a request might; the second succeeds.
    let on_confirm = {
        let (open, attempts, outcome) = (open.clone(), attempts.clone(), outcome.clone());
        Callback::from(move |()| {
            if *attempts == 0 {
                attempts.set(1);
            } else {
                outcome.set("Rollout web-42 was aborted");
                open.set(false);
            }
        })
    };

    let error = (*attempts > 0).then_some("The controller did not answer. Nothing was changed.");

    html! {
        <Stack>
            <Cluster>
                <Button on_click={show}>{ "Abort rollout" }</Button>
            </Cluster>
            <span role="status">{ *outcome }</span>
            <ConfirmDialog
                open={*open}
                title="Abort rollout web-42?"
                confirm_label="Abort rollout"
                cancel_label="Keep running"
                {error}
                {on_confirm}
                {on_cancel}
            >
                <p>{ "The 4 hosts already updated stay on the new version. The remaining 8 are left untouched." }</p>
            </ConfirmDialog>
        </Stack>
    }
}
