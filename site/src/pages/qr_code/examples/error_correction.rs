use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let value = "https://example.com/invite/7f3a9c";
    let levels = [
        (QrEcc::Low, "Low"),
        (QrEcc::Medium, "Medium"),
        (QrEcc::Quartile, "Quartile"),
        (QrEcc::High, "High"),
    ];

    html! {
        <Cluster>
            { for levels.into_iter().map(|(ecc, name)| html! {
                <QrCode {value} {ecc} size=144 label={format!("Invite link, {} error correction", name.to_lowercase())}>
                    { name }
                </QrCode>
            }) }
        </Cluster>
    }
}
