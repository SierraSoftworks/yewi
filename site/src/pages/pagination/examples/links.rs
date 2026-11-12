use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let href = Callback::from(|page: u32| NavTarget::href("").query([("page", page.to_string())]));

    html! { <Pagination page=1 pages=9 {href} /> }
}
