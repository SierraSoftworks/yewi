use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Pagination label="No siblings" page=10 pages=20 siblings=0 />
            <Pagination label="One sibling" page=10 pages=20 />
            <Pagination label="Two siblings" page=10 pages=20 siblings=2 />
            <Pagination label="Two boundaries" page=10 pages=20 boundaries=2 />
        </Stack>
    }
}
