use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Pagination label="Small" page=2 pages=8 size={Size::Small} />
            <Pagination label="Medium" page=2 pages=8 />
            <Pagination label="Large" page=2 pages=8 size={Size::Large} />
            <Pagination label="Loading" page=2 pages=8 disabled=true />
        </Stack>
    }
}
