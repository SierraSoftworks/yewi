use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let alerts = |variant: AlertVariant| {
        html! {
            <Stack>
                <Alert {variant} title="Maintenance tonight">
                    { "Probes pause between 22:00 and 22:30 UTC." }
                </Alert>
                <Alert {variant} tone={Tone::Warn} title="Probe is flapping">
                    { "4 state changes in the last hour." }
                </Alert>
                <Alert {variant} tone={Tone::Danger} title="Refresh failed">
                    { "The agent did not respond within 10 seconds." }
                </Alert>
            </Stack>
        }
    };

    html! {
        <Grid min="18rem">
            { alerts(AlertVariant::Soft) }
            { alerts(AlertVariant::Edge) }
        </Grid>
    }
}
