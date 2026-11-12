use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let added = use_state(|| 0_u32);
    let add = {
        let added = added.clone();
        Callback::from(move |_| added.set(*added + 1))
    };

    html! {
        <Stack>
            <EmptyState
                title="No clusters yet"
                icon={icons::SERVER}
                actions={html! {
                    <>
                        <Button variant={ButtonVariant::Primary} icon={icons::PLUS} on_click={add}>{ "Add a cluster" }</Button>
                        <Button variant={ButtonVariant::Subtle} href="#clusters">{ "Read the guide" }</Button>
                    </>
                }}
            >
                { "Clusters appear here once an agent connects. Adding one takes about two minutes." }
            </EmptyState>
            <span role="status">{ format!("Started adding {} clusters", *added) }</span>
        </Stack>
    }
}
