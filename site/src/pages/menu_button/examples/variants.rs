use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let items = html! {
        <>
            <MenuItem>{ "Last hour" }</MenuItem>
            <MenuItem>{ "Last 24 hours" }</MenuItem>
            <MenuItem>{ "Last 7 days" }</MenuItem>
        </>
    };

    html! {
        <Stack>
            <Cluster>
                <MenuButton label="Default">{ items.clone() }</MenuButton>
                <MenuButton label="Primary" variant={ButtonVariant::Primary}>{ items.clone() }</MenuButton>
                <MenuButton label="Subtle" variant={ButtonVariant::Subtle}>{ items.clone() }</MenuButton>
                <MenuButton label="Disabled" disabled=true>{ items.clone() }</MenuButton>
            </Cluster>
            <Cluster>
                <MenuButton label="Small" size={Size::Small}>{ items.clone() }</MenuButton>
                <MenuButton label="Medium">{ items.clone() }</MenuButton>
                <MenuButton label="Large" size={Size::Large}>{ items.clone() }</MenuButton>
                <MenuButton label="Time range" icon={icons::HISTORY}>{ items.clone() }</MenuButton>
                <MenuButton label="No chevron" caret=false>{ items }</MenuButton>
            </Cluster>
        </Stack>
    }
}
