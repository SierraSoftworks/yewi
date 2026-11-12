use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let chosen = use_state(|| "Nothing chosen yet".to_string());
    let choose = |what: &'static str| {
        let chosen = chosen.clone();
        Callback::from(move |_: MouseEvent| chosen.set(format!("Opened {what}")))
    };

    let brand = html! { <AppBarBrand logo={html! { <Icon icon={icons::BOXES} /> }}>{ "Buckle" }</AppBarBrand> };

    html! {
        <Stack>
            <AppBar {brand}>
                <TopMenu>
                    <TopMenuItem href="#overview">{ "Overview" }</TopMenuItem>
                    <TopMenuGroup label="Fleet" current=true>
                        <MenuItem icon={icons::SERVER} on_select={choose("Nodes")}>{ "Nodes" }</MenuItem>
                        <MenuItem icon={icons::PACKAGE} on_select={choose("Packages")}>{ "Packages" }</MenuItem>
                        <MenuItem icon={icons::HISTORY} on_select={choose("Rollouts")}>{ "Rollouts" }</MenuItem>
                    </TopMenuGroup>
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
