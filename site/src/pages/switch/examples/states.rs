use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="11rem">
            <Stack gap={Space::Sm}>
                <ToggleSwitch>{ "Off" }</ToggleSwitch>
                <ToggleSwitch checked=true>{ "On" }</ToggleSwitch>
            </Stack>
            <Stack gap={Space::Sm}>
                <ToggleSwitch disabled=true>{ "Disabled, off" }</ToggleSwitch>
                <ToggleSwitch disabled=true checked=true>{ "Disabled, on" }</ToggleSwitch>
            </Stack>
            <Stack gap={Space::Sm}>
                <ToggleSwitch size={Size::Small} checked=true>{ "Small" }</ToggleSwitch>
                <ToggleSwitch checked=true>{ "Medium" }</ToggleSwitch>
                <ToggleSwitch size={Size::Large} checked=true>{ "Large" }</ToggleSwitch>
            </Stack>
        </Grid>
    }
}
