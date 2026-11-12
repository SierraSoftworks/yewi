use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let saves = use_state(|| 0_u32);
    let saving = use_state(|| false);

    let on_click = {
        let (saves, saving) = (saves.clone(), saving.clone());
        Callback::from(move |_| {
            saves.set(*saves + 1);
            saving.set(true);
        })
    };
    let finish = {
        let saving = saving.clone();
        Callback::from(move |_| saving.set(false))
    };

    html! {
        <Stack>
            <Cluster>
                <Button variant={ButtonVariant::Primary} loading={*saving} {on_click}>{ "Save" }</Button>
                <Button disabled={!*saving} on_click={finish}>{ "Finish saving" }</Button>
                <Button disabled=true>{ "Unavailable" }</Button>
            </Cluster>
            <span role="status">{ format!("Saved {} times", *saves) }</span>
        </Stack>
    }
}
