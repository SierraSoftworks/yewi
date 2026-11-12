use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <QrCode value="https://github.com/SierraSoftworks/yewi" label="Link to the yewi repository">
            { "Scan to open the repository on your phone." }
        </QrCode>
    }
}
