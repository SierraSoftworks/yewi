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
                <span>{ "Text-colour label (default)" }</span>
                { account("Text-colour label", "") }
            </Stack>
            <Stack gap={Space::Sm}>
                <span>{ "Accent label" }</span>
                { account("Accent label", "--yi-tabs-fg-active: var(--yi-accent-text)") }
            </Stack>
        </Grid>
    }
}
