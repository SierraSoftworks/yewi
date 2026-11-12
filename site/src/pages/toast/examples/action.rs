use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
fn Hosts() -> Html {
    let toast = use_toast();
    let archived = use_state(|| false);

    let archive = {
        let archived = archived.clone();
        Callback::from(move |_| {
            archived.set(true);
            let undo = {
                let archived = archived.clone();
                Callback::from(move |()| archived.set(false))
            };
            toast.show(ToastMessage::new(Tone::Ok, "3 hosts archived.").action(ToastAction::new("Undo", undo)));
        })
    };

    html! {
        <Cluster>
            <Button disabled={*archived} on_click={archive}>{ "Archive 3 hosts" }</Button>
            <span role="status">{ if *archived { "No active hosts" } else { "3 active hosts" } }</span>
        </Cluster>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <ToastHost>
            <Hosts />
        </ToastHost>
    }
}
