use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let outcome = use_state(|| "Cluster prod-eu is healthy");

    let show = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };
    let on_cancel = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };
    let on_confirm = {
        let (open, outcome) = (open.clone(), outcome.clone());
        Callback::from(move |()| {
            outcome.set("Cluster prod-eu was deleted");
            open.set(false);
        })
    };

    html! {
        <Stack>
            <Cluster>
                <Button variant={ButtonVariant::Danger} on_click={show}>{ "Delete cluster" }</Button>
            </Cluster>
            <span role="status">{ *outcome }</span>
            <ConfirmDialog
                open={*open}
                title="Delete cluster prod-eu?"
                confirm_label="Delete cluster"
                danger=true
                acknowledge="I have reviewed the 4 destructive steps"
                type_to_confirm="prod-eu"
                {on_confirm}
                {on_cancel}
            >
                <p>{ "All 12 hosts are drained and removed, and the cluster's history is deleted. This cannot be undone." }</p>
            </ConfirmDialog>
        </Stack>
    }
}
