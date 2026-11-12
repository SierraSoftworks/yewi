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
            <MenuButton label="View" icon={icons::SETTINGS}>
                <MenuGroup label="Display">
                    <MenuItem checked={*relative} on_select={toggle(&relative)}>{ "Relative times" }</MenuItem>
                    <MenuItem checked={*wrap} on_select={toggle(&wrap)}>{ "Wrap long lines" }</MenuItem>
                </MenuGroup>
                <MenuGroup label="Logs">
                    <MenuItem icon={icons::DOWNLOAD} aside="12 MB">{ "Download" }</MenuItem>
                    <MenuItem icon={icons::EXTERNAL_LINK} href="#query-syntax">{ "Query syntax" }</MenuItem>
                </MenuGroup>
                <MenuItem icon={icons::TRASH_2} tone={Tone::Danger}>{ "Clear logs" }</MenuItem>
            </MenuButton>
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
