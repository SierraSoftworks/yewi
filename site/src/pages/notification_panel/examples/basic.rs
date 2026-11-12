use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster justify={Justify::End}>
            <NotificationPanel
                title="Needs attention"
                label="3 hosts need attention"
                description="Offline agents, failing health checks and version skew"
                count=3
                tone={Tone::Danger}
            >
                <NotificationItem title="web-03.eu-west-1" href="#web-03" tone={Tone::Danger}>
                    <NotificationReason tone={Tone::Danger}>{ "Agent offline for 12 minutes" }</NotificationReason>
                </NotificationItem>
                <NotificationItem title="db-01.eu-west-1" href="#db-01" tone={Tone::Danger}>
                    <NotificationReason tone={Tone::Danger}>{ "Health check failing" }</NotificationReason>
                    <NotificationReason tone={Tone::Warn}>{ "Plan is stale" }</NotificationReason>
                </NotificationItem>
                <NotificationItem title="cache-02.us-east-1" href="#cache-02" tone={Tone::Warn}>
                    <NotificationReason tone={Tone::Warn}>{ "Agent 2.13.1 is behind 2.14.0" }</NotificationReason>
                </NotificationItem>
            </NotificationPanel>
        </Cluster>
    }
}
