use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let retries = use_state(|| 0_u32);
    let on_click = {
        let retries = retries.clone();
        Callback::from(move |_| retries.set(*retries + 1))
    };

    html! {
        <Stack>
            <Text tag={TextTag::P}>
                { "The probe timed out. " }
                <Link on_click={on_click.clone()}>{ "Retry now" }</Link>
                { " or " }
                <Link disabled=true {on_click}>{ "edit the probe" }</Link>
                { " (read-only access)." }
            </Text>
            <Text tag={TextTag::P}>
                <Link href="#as-an-action" disabled=true>{ "Audit log" }</Link>
                { " is unavailable while offline." }
            </Text>
            <span role="status">{ format!("Retried {} times", *retries) }</span>
        </Stack>
    }
}
