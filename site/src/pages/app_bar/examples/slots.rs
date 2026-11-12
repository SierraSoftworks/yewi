use yew::prelude::*;
use yewi::prelude::*;

const MENU: [&str; 3] = ["Dashboards", "Alerts", "Settings"];

#[function_component]
pub fn Example() -> Html {
    let section = use_state(|| MENU[0]);

    let brand = html! { <AppBarBrand logo={html! { <Icon icon={icons::ACTIVITY} /> }}>{ "Grey" }</AppBarBrand> };
    // Given to the menu rather than the bar, so that on a narrow screen they
    // fold into its dropdown with the destinations.
    let actions = html! {
        <>
            <Pill tone={Tone::Ok} active=true>{ "live" }</Pill>
            <IconButton icon={icons::BELL} label="Notifications" variant={ButtonVariant::Subtle} />
            <Avatar name="Ada Lovelace" size={Size::Small} />
        </>
    };

    html! {
        <Stack>
            <AppBar {brand}>
                <TopMenu label="Sections" {actions}>
                    { for MENU.into_iter().map(|name| {
                        // An app links to its own pages; the example stays on this one.
                        let on_click = {
                            let section = section.clone();
                            Callback::from(move |event: MouseEvent| {
                                event.prevent_default();
                                section.set(name);
                            })
                        };
                        html! {
                            <TopMenuItem href={format!("#{}", name.to_lowercase())} current={*section == name} {on_click}>
                                { name }
                            </TopMenuItem>
                        }
                    }) }
                </TopMenu>
            </AppBar>
            <span role="status">{ format!("Showing {}", *section) }</span>
        </Stack>
    }
}
