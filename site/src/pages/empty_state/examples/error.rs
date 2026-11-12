use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let attempts = use_state(|| 0_u32);
    let retrying = use_state(|| false);

    let on_retry = {
        let (attempts, retrying) = (attempts.clone(), retrying.clone());
        Callback::from(move |()| {
            attempts.set(*attempts + 1);
            retrying.set(true);
        })
    };
    let fail = {
        let retrying = retrying.clone();
        Callback::from(move |_| retrying.set(false))
    };

    html! {
        <Stack>
            <ErrorState title="The server could not be reached" {on_retry} retrying={*retrying}>
                { "Check your connection. Nothing you entered has been lost." }
            </ErrorState>
            <Cluster justify={Justify::Center}>
                <Button variant={ButtonVariant::Subtle} disabled={!*retrying} on_click={fail}>{ "Fail the retry" }</Button>
                <span role="status">{ format!("Retried {} times", *attempts) }</span>
            </Cluster>
        </Stack>
    }
}
