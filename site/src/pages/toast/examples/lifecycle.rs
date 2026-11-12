use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
fn Rollout() -> Html {
    let toast = use_toast();
    let running = use_state(|| None::<ToastId>);

    let start = {
        let (toast, running) = (toast.clone(), running.clone());
        Callback::from(move |_| {
            running.set(Some(toast.show(ToastMessage::new(Tone::Active, "Rolling out to 12 hosts."))));
        })
    };
    // In an app this runs when the request completes.
    let finish = {
        let running = running.clone();
        Callback::from(move |_| {
            if let Some(id) = *running {
                toast.dismiss(id);
                toast.ok("Rollout finished on 12 hosts.");
                running.set(None);
            }
        })
    };

    html! {
        <Cluster>
            <Button variant={ButtonVariant::Primary} loading={running.is_some()} on_click={start}>
                { "Roll out" }
            </Button>
            <Button disabled={running.is_none()} on_click={finish}>{ "Finish rollout" }</Button>
        </Cluster>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <ToastHost>
            <Rollout />
        </ToastHost>
    }
}
