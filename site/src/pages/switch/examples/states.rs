use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="11rem">
            <Stack gap={Space::Sm}>
                <Switch>{ "Off" }</Switch>
                <Switch checked=true>{ "On" }</Switch>
            </Stack>
            <Stack gap={Space::Sm}>
                <Switch disabled=true>{ "Disabled, off" }</Switch>
                <Switch disabled=true checked=true>{ "Disabled, on" }</Switch>
            </Stack>
            <Stack gap={Space::Sm}>
                <Switch size={Size::Small} checked=true>{ "Small" }</Switch>
                <Switch checked=true>{ "Medium" }</Switch>
                <Switch size={Size::Large} checked=true>{ "Large" }</Switch>
            </Stack>
        </Grid>
    }
}
