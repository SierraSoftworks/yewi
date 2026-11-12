use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="12rem">
            <Stack gap={Space::Sm}>
                <Checkbox>{ "Unchecked" }</Checkbox>
                <Checkbox checked=true>{ "Checked" }</Checkbox>
                <Checkbox indeterminate=true>{ "Indeterminate" }</Checkbox>
                <Checkbox invalid=true>{ "Invalid" }</Checkbox>
            </Stack>
            <Stack gap={Space::Sm}>
                <Checkbox disabled=true>{ "Disabled" }</Checkbox>
                <Checkbox disabled=true checked=true>{ "Disabled, checked" }</Checkbox>
                <Checkbox disabled=true indeterminate=true>{ "Disabled, indeterminate" }</Checkbox>
            </Stack>
            <Stack gap={Space::Sm}>
                <Checkbox size={Size::Small} checked=true>{ "Small" }</Checkbox>
                <Checkbox checked=true>{ "Medium" }</Checkbox>
                <Checkbox size={Size::Large} checked=true>{ "Large" }</Checkbox>
            </Stack>
        </Grid>
    }
}
