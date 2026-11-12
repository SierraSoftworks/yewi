use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let chosen = use_state(|| "Nothing chosen yet");
    let choose = |action: &'static str| {
        let chosen = chosen.clone();
        Callback::from(move |_| chosen.set(action))
    };

    html! {
        <Stack align={Align::Start}>
            <Card flush=true>
                <Menu label="Dashboard">
                    <MenuItem icon={icons::PENCIL} on_select={choose("Renamed the dashboard")}>{ "Rename" }</MenuItem>
                    <MenuItem icon={icons::COPY} on_select={choose("Duplicated the dashboard")}>{ "Duplicate" }</MenuItem>
                    <MenuItem icon={icons::DOWNLOAD} on_select={choose("Exported the dashboard")}>{ "Export" }</MenuItem>
                    <MenuItem icon={icons::ARCHIVE} on_select={choose("Archived the dashboard")}>{ "Archive" }</MenuItem>
                </Menu>
            </Card>
            <span role="status">{ *chosen }</span>
        </Stack>
    }
}
