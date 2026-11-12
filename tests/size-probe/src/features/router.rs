//! Navigation components resolve their links against a router when the
//! `router` feature is on, which is what this measures.

use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    let href = Callback::from(|page: u32| NavTarget::href("/items").query([("page", page.to_string())]));

    html! {
        <>
            <Breadcrumbs>
                <Crumb href="/">{ "Home" }</Crumb>
                <Crumb>{ "Items" }</Crumb>
            </Breadcrumbs>
            <TabNav label="Sections">
                <TabLink href="/items" current=true>{ "Items" }</TabLink>
            </TabNav>
            <Pagination page=1 pages=9 {href} />
        </>
    }
}
