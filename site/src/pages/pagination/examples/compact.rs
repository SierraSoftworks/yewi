use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let page = use_state(|| 3_u32);
    let on_change = {
        let page = page.clone();
        Callback::from(move |to: u32| page.set(to))
    };

    html! {
        <Stack>
            <Pagination label="Results" page={*page} pages=12 compact=true on_change={on_change.clone()} />
            // A cursor-based list: the total is unknown and this happens to be the last page.
            <Pagination label="Audit log" page=4 has_next=false />
        </Stack>
    }
}
