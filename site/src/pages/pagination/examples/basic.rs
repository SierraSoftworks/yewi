use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let page = use_state(|| 1_u32);
    let on_change = {
        let page = page.clone();
        Callback::from(move |to: u32| page.set(to))
    };

    html! {
        <Stack>
            <Pagination page={*page} pages=12 {on_change} />
            <span role="status">{ format!("Showing page {} of 12", *page) }</span>
        </Stack>
    }
}
