use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let outcome = use_state(|| "pi-1 is registered");

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
            outcome.set("pi-1 was forgotten");
            open.set(false);
        })
    };

    html! {
        <Stack>
            <Cluster>
                <Button variant={ButtonVariant::Danger} on_click={show}>{ "Forget host" }</Button>
            </Cluster>
            <span role="status">{ *outcome }</span>
            <ConfirmDialog
                open={*open}
                title="Forget pi-1?"
                confirm_label="Forget pi-1"
                danger=true
                {on_confirm}
                {on_cancel}
            >
                <p>{ "The host is removed from the cluster and its history is deleted. It can register again with a new token." }</p>
            </ConfirmDialog>
        </Stack>
    }
}
