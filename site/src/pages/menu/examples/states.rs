use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let relative = use_state(|| true);
    let wrap = use_state(|| false);
    let toggle = |option: &UseStateHandle<bool>| {
        let option = option.clone();
        Callback::from(move |_| option.set(!*option))
    };

    html! {
        <Stack align={Align::Start}>
            <Card flush=true>
                <Menu label="Log view">
                    <MenuItem checked={*relative} on_select={toggle(&relative)}>{ "Relative times" }</MenuItem>
                    <MenuItem checked={*wrap} on_select={toggle(&wrap)}>{ "Wrap long lines" }</MenuItem>
                    <MenuSeparator />
                    <MenuItem icon={icons::DOWNLOAD} disabled=true>{ "Download logs" }</MenuItem>
                    <MenuItem icon={icons::EXTERNAL_LINK} href="#query-syntax">{ "Query syntax" }</MenuItem>
                </Menu>
            </Card>
            <span role="status">
                { format!(
                    "{} times, {}",
                    if *relative { "Relative" } else { "Exact" },
                    if *wrap { "wrapped" } else { "unwrapped" },
                ) }
            </span>
        </Stack>
    }
}
