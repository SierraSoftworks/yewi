use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Card title="Needs attention" subtitle="4 of 31 hosts" attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <NotificationList>
                <NotificationItem title="web-03.eu-west-1" href="#web-03" tone={Tone::Danger} meta="12 min">
                    <NotificationReason tone={Tone::Danger}>{ "Agent offline" }</NotificationReason>
                    <NotificationReason tone={Tone::Warn}>{ "Plan is stale" }</NotificationReason>
                </NotificationItem>
                <NotificationItem title="cache-02.us-east-1" href="#cache-02" tone={Tone::Warn} meta="1 h">
                    <NotificationReason tone={Tone::Warn}>{ "Agent 2.13.1 is behind 2.14.0" }</NotificationReason>
                </NotificationItem>
                <NotificationItem title="queue-01.us-east-1" href="#queue-01" tone={Tone::Active} meta="3 min">
                    { "Rolling out revision 4f2a9c1" }
                </NotificationItem>
                <NotificationItem title="batch-07.eu-west-1" tone={Tone::Info} meta="Yesterday">
                    { "Waiting for its registration to be approved" }
                </NotificationItem>
            </NotificationList>
        </Card>
    }
}
