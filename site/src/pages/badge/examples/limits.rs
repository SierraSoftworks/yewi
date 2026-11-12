use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Lg}>
            <Cluster gap={Space::Xs} align={Align::Center}>{ "Events" }<Badge count=99 /></Cluster>
            <Cluster gap={Space::Xs} align={Align::Center}>{ "Events" }<Badge count=1280 /></Cluster>
            <Cluster gap={Space::Xs} align={Align::Center}>{ "Events" }<Badge count=1280 max=999 /></Cluster>
            <Cluster gap={Space::Xs} align={Align::Center}>{ "Errors" }<Badge count=0 /></Cluster>
            <Cluster gap={Space::Xs} align={Align::Center}>{ "Errors" }<Badge count=0 show_zero=true /></Cluster>
        </Cluster>
    }
}
