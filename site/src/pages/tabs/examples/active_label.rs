use yew::prelude::*;
use yewi::prelude::*;

fn account(label: &'static str, style: &'static str) -> Html {
    html! {
        <Tabs {label} attrs={attrs! { "style" => style }}>
            <Tab value="profile" label="Profile">{ "Name, callsign and groups." }</Tab>
            <Tab value="devices" label="Devices">{ "Enrolled phones and radios." }</Tab>
            <Tab value="credentials" label="Credentials">{ "Client certificates." }</Tab>
        </Tabs>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="18rem" gap={Space::Xl}>
            <Stack gap={Space::Sm}>
                <span>{ "A. Accent label (default)" }</span>
                { account("Accent label", "") }
            </Stack>
            <Stack gap={Space::Sm}>
                <span>{ "B. Text-colour label" }</span>
                { account("Text-colour label", "--yi-tabs-fg-active: var(--yi-fg)") }
            </Stack>
        </Grid>
    }
}
