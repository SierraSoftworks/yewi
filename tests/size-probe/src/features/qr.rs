use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    html! { <QrCode value="https://example.com/">{ "Scan to open" }</QrCode> }
}
