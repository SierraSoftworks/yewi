use yew::prelude::*;
use yewi::prelude::*;

const PAGES: [&str; 4] = ["Overview", "Hosts", "Rollouts", "Settings"];

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let page = use_state(|| PAGES[0]);

    let show = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };
    let close = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };

    html! {
        <Stack>
            <Cluster>
                <Button icon={icons::MENU} on_click={show}>{ "Menu" }</Button>
            </Cluster>
            <span role="status">{ format!("Showing {}", *page) }</span>
            <Drawer open={*open} label="Navigation" side={DrawerSide::Start} size={Size::Small} on_close={close}>
                <Stack gap={Space::Xs}>
                    { for PAGES.into_iter().map(|name| {
                        let (open, page) = (open.clone(), page.clone());
                        let on_click = move |_| {
                            page.set(name);
                            open.set(false);
                        };
                        html! { <Button variant={ButtonVariant::Subtle} {on_click}>{ name }</Button> }
                    }) }
                </Stack>
            </Drawer>
        </Stack>
    }
}
