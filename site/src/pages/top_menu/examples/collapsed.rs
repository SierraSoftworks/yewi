use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let chosen = use_state(|| "Nothing chosen yet".to_string());
    let choose = |what: &'static str| {
        let chosen = chosen.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            chosen.set(format!("Opened {what}"));
        })
    };

    let brand = html! { <AppBarBrand logo={html! { <Icon icon={icons::ACTIVITY} /> }}>{ "Grey" }</AppBarBrand> };
    let actions = html! {
        <>
            <Pill tone={Tone::Ok} active=true>{ "live" }</Pill>
            <ThemeToggle />
        </>
    };

    html! {
        <Stack>
            <AppBar {brand} {actions}>
                <TopMenu collapse={TopMenuCollapse::Always}>
                    <TopMenuItem href="#services" current=true on_click={choose("Services")}>{ "Services" }</TopMenuItem>
                    <TopMenuItem href="#incidents" on_click={choose("Incidents")}>{ "Incidents" }</TopMenuItem>
                    <TopMenuGroup label="Admin">
                        <MenuItem on_select={choose("Accounts")}>{ "Accounts" }</MenuItem>
                        <MenuItem on_select={choose("Audit log")}>{ "Audit log" }</MenuItem>
                    </TopMenuGroup>
                </TopMenu>
            </AppBar>
            <span role="status">{ (*chosen).clone() }</span>
        </Stack>
    }
}
