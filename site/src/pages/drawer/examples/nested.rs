use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let confirming = use_state(|| false);
    let outcome = use_state(|| "Project Website exists");

    let show = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };
    let close = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };
    let ask = {
        let confirming = confirming.clone();
        Callback::from(move |_| confirming.set(true))
    };
    let on_cancel = {
        let confirming = confirming.clone();
        Callback::from(move |()| confirming.set(false))
    };
    let on_confirm = {
        let (open, confirming, outcome) = (open.clone(), confirming.clone(), outcome.clone());
        Callback::from(move |()| {
            outcome.set("Project Website was deleted");
            confirming.set(false);
            open.set(false);
        })
    };

    html! {
        <Stack>
            <Cluster>
                <Button on_click={show}>{ "Manage project" }</Button>
            </Cluster>
            <span role="status">{ *outcome }</span>
            <Drawer
                open={*open}
                title="Project Website"
                on_close={close.clone()}
                footer={html! { <Button on_click={close.reform(|_| ())}>{ "Done" }</Button> }}
            >
                <p>{ "Collects page views from 2 sources: sierrasoftworks.com and the documentation site." }</p>
                <p>{ "Deleting the project removes its dashboards. The page views stay with their sources." }</p>
                <Cluster>
                    <Button variant={ButtonVariant::Danger} on_click={ask}>{ "Delete project" }</Button>
                </Cluster>
                <ConfirmDialog
                    open={*confirming}
                    title="Delete project Website?"
                    confirm_label="Delete project"
                    danger=true
                    {on_confirm}
                    {on_cancel}
                >
                    <p>{ "Its 3 dashboards are removed. This cannot be undone." }</p>
                </ConfirmDialog>
            </Drawer>
        </Stack>
    }
}
