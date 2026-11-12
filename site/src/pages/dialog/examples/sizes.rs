use yew::prelude::*;
use yewi::prelude::*;

const SIZES: [(Size, &str); 3] = [(Size::Small, "Small"), (Size::Medium, "Medium"), (Size::Large, "Large")];

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| None::<Size>);

    let close = {
        let open = open.clone();
        Callback::from(move |()| open.set(None))
    };

    html! {
        <Stack>
            <Cluster>
                { for SIZES.into_iter().map(|(size, label)| {
                    let open = open.clone();
                    html! { <Button on_click={move |_| open.set(Some(size))}>{ label }</Button> }
                }) }
            </Cluster>
            { for SIZES.into_iter().map(|(size, label)| html! {
                <Dialog
                    open={*open == Some(size)}
                    title={format!("{label} dialog")}
                    {size}
                    on_close={close.clone()}
                    footer={html! { <Button on_click={close.reform(|_| ())}>{ "Done" }</Button> }}
                >
                    <p>{ "The width is a maximum. The dialog is as tall as its content, up to the height of the screen." }</p>
                </Dialog>
            }) }
        </Stack>
    }
}
