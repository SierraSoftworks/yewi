use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let visible = use_state(|| true);
    let retries = use_state(|| 0_u32);

    let on_dismiss = {
        let visible = visible.clone();
        Callback::from(move |()| visible.set(false))
    };
    let show = {
        let visible = visible.clone();
        Callback::from(move |_| visible.set(true))
    };
    let retry = {
        let retries = retries.clone();
        Callback::from(move |_| retries.set(*retries + 1))
    };

    let status = match (*visible, *retries) {
        (false, _) => "Alert dismissed".to_owned(),
        (true, 0) => "Alert shown".to_owned(),
        (true, n) => format!("Retried {n} times"),
    };

    html! {
        <Stack>
            if *visible {
                <Alert
                    tone={Tone::Danger}
                    title="Could not save the mission"
                    {on_dismiss}
                    actions={html! {
                        <>
                            <Button size={Size::Small} on_click={retry}>{ "Retry" }</Button>
                            <Button size={Size::Small} variant={ButtonVariant::Subtle}>{ "Discard changes" }</Button>
                        </>
                    }}
                >
                    { "The server rejected the update because the mission changed while you were editing it." }
                </Alert>
            } else {
                <Cluster>
                    <Button on_click={show}>{ "Show the alert again" }</Button>
                </Cluster>
            }
            <Note attrs={attrs! { "role" => "status" }}>{ status }</Note>
        </Stack>
    }
}
