use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Alert title="Certificate renewed">
                { "The new certificate is valid until 14 April 2026." }
            </Alert>
            <Alert tone={Tone::Ok}>{ "All 31 hosts are running the latest release." }</Alert>
            <Alert tone={Tone::Warn} title="Certificate expires in 6 days">
                { "Renewal is waiting for a DNS record to propagate." }
            </Alert>
            <Alert tone={Tone::Danger} title="Rollout failed">
                { "3 of 31 hosts did not pass their health check and were rolled back." }
            </Alert>
            <Alert tone={Tone::Active} title="Rollout in progress">{ "12 of 31 hosts done." }</Alert>
            <Alert tone={Tone::Neutral}>{ "This workspace is archived and read-only." }</Alert>
        </Stack>
    }
}
