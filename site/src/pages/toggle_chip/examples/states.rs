use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <ToggleChipGroup label="Small">
                <ToggleChip>{ "Off" }</ToggleChip>
                <ToggleChip pressed=true>{ "On" }</ToggleChip>
                <ToggleChip disabled=true>{ "Off, disabled" }</ToggleChip>
                <ToggleChip pressed=true disabled=true>{ "On, disabled" }</ToggleChip>
            </ToggleChipGroup>
            <ToggleChipGroup label="Medium">
                <ToggleChip size={Size::Medium}>{ "Off" }</ToggleChip>
                <ToggleChip size={Size::Medium} pressed=true>{ "On" }</ToggleChip>
                <ToggleChip size={Size::Medium} icon={icons::PIN}>{ "With an icon" }</ToggleChip>
            </ToggleChipGroup>
        </Stack>
    }
}
