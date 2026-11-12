use yew::prelude::*;
use yewi::prelude::*;


const MENU: [&str; 3] = ["Dashboards", "Alerts", "Settings"];

#[function_component]
pub fn Example() -> Html {
    let section = use_state(|| MENU[0]);

    let brand = html! { <AppBarBrand logo={html! { <Icon icon={icons::ACTIVITY} /> }}>{ "Grey" }</AppBarBrand> };
    let actions = html! {
        <>
            <Pill tone={Tone::Ok} active=true>{ "live" }</Pill>
            <IconButton icon={icons::BELL} label="Notifications" variant={ButtonVariant::Subtle} />
            <Avatar name="Ada Lovelace" size={Size::Small} />
        </>
    };

    html! {
        <Stack>
            <AppBar {brand} {actions}>
                <Cluster gap={Space::Xs} attrs={attrs! { "role" => "group", "aria-label" => "Sections" }}>
                    { for MENU.into_iter().map(|name| {
                        let on_click = {
                            let section = section.clone();
                            Callback::from(move |_| section.set(name))
                        };
                        html! {
                            <Button variant={ButtonVariant::Subtle} pressed={*section == name} {on_click}>{ name }</Button>
                        }
                    }) }
                </Cluster>
            </AppBar>
            <span role="status">{ format!("Showing {}", *section) }</span>
        </Stack>
    }
}
