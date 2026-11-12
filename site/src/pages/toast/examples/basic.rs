use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
fn Settings() -> Html {
    let toast = use_toast();
    let save = {
        let toast = toast.clone();
        Callback::from(move |_| {
            toast.ok("Settings saved.");
        })
    };
    let publish = Callback::from(move |_| {
        toast.error("Publishing failed: the registry refused the upload.");
    });

    html! {
        <Cluster>
            <Button variant={ButtonVariant::Primary} on_click={save}>{ "Save" }</Button>
            <Button on_click={publish}>{ "Publish" }</Button>
        </Cluster>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <ToastHost>
            <Settings />
        </ToastHost>
    }
}
