use yew::prelude::*;
use yewi::prelude::*;

const SIERRA: IconData = IconData::new("M3 19l6-11 4 7 2-3 6 7z");

#[function_component]
pub fn Example() -> Html {
    html! { <Icon icon={SIERRA} size="32px" label="Mountains" /> }
}
