use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xl} align={Align::Center}>
            <Badge tone={Tone::Danger} label="New notifications">
                <IconButton icon={icons::BELL} label="Notifications" />
            </Badge>
            <Badge tone={Tone::Info} label="Update available">
                <Button>{ "Settings" }</Button>
            </Badge>
            <Cluster gap={Space::Xs} align={Align::Center}>
                { "Changelog" }
                <Badge tone={Tone::Info} label="New" />
            </Cluster>
        </Cluster>
    }
}
