use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    // A certificate bundle, say: far more than a QR code can carry.
    let value = "MIIC".repeat(1000);

    html! {
        <Cluster>
            <QrCode value={value.clone()} />
            <QrCode {value} fallback={html! { "This bundle is too large to scan. Download it instead." }} />
        </Cluster>
    }
}
