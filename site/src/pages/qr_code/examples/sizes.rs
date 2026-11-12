use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let value = "https://example.com/invite/7f3a9c";

    html! {
        <Cluster align={Align::End}>
            <QrCode {value} size=96 label="Invite link, small">{ "96" }</QrCode>
            <QrCode {value} size=144 label="Invite link, medium">{ "144" }</QrCode>
            <QrCode {value} label="Invite link, default size">{ "Default" }</QrCode>
        </Cluster>
    }
}
