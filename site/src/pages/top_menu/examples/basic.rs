use yew::prelude::*;
use yewi::prelude::*;

const PAGES: [&str; 4] = ["Workflows", "Connections", "Activity", "Data"];

#[function_component]
pub fn Example() -> Html {
    let page = use_state(|| PAGES[0]);

    let brand = html! { <AppBarBrand logo={html! { <Icon icon={icons::WORKFLOW} /> }}>{ "Automate" }</AppBarBrand> };
    let actions = html! {
        <>
            <ThemeToggle />
            <Avatar name="Ada Lovelace" size={Size::Small} />
        </>
    };

    html! {
        <Stack>
            <AppBar {brand} {actions}>
                <TopMenu>
                    { for PAGES.into_iter().map(|name| {
                        // An app links to its own pages; the example stays on this one.
                        let on_click = {
                            let page = page.clone();
                            Callback::from(move |event: MouseEvent| {
                                event.prevent_default();
                                page.set(name);
                            })
                        };
                        html! {
                            <TopMenuItem href={format!("#{}", name.to_lowercase())} current={*page == name} {on_click}>
                                { name }
                            </TopMenuItem>
                        }
                    }) }
                    <TopMenuItem href={NavTarget::href("https://example.com/docs").external()}>{ "Docs" }</TopMenuItem>
                </TopMenu>
            </AppBar>
            <span role="status">{ format!("Showing {}", *page) }</span>
        </Stack>
    }
}
