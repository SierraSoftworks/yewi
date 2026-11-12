use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let copied = use_state(|| false);

    let copy = {
        let copied = copied.clone();
        Callback::from(move |_| copied.set(true))
    };
    let reset = {
        let copied = copied.clone();
        Callback::from(move |_| copied.set(false))
    };

    html! {
        <Stack>
            <Cluster>
                // Held open to confirm the copy, and back to hover and focus otherwise.
                <Tooltip tip={if *copied { "Copied" } else { "Copy the deploy key" }} open={copied.then_some(true)}>
                    <Button icon={icons::COPY} on_click={copy}>{ "Copy key" }</Button>
                </Tooltip>
                <Button variant={ButtonVariant::Subtle} on_click={reset}>{ "Reset" }</Button>
            </Cluster>
            <span role="status">{ if *copied { "Key copied" } else { "Nothing copied yet" } }</span>
        </Stack>
    }
}
